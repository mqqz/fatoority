//! Portable binding contracts run natively so LLVM coverage includes the JS API.
mod support;
use base64ct::{Base64, Encoding};
use fatoora_ffi::api::ffi::ClearanceMode;
use fatoora_ffi::common::ffi::Environment;
use fatoora_ffi::{
    api::ffi::*,
    common::ffi::BindingError,
    crypto::ffi::{Config, Csr, CsrProperties, SigningKey},
    invoice::ffi::SignedInvoice,
};
use serde_json::{Value, json};
use support::written;

const SIMPLE: &[u8] =
    include_bytes!("../../fatoora-core/tests/fixtures/invoices/sample-simplified-invoice.xml");
const STANDARD: &[u8] = include_bytes!(
    "../../fatoora-core/tests/fixtures/sdk-parity/cases/standard-invoice/sdk-signed.xml"
);
const CREDENTIALS: &[u8] =
    br#"{"requestID":123,"binarySecurityToken":"new-token","secret":"new-secret"}"#;
fn ok<T>(result: Result<T, Box<BindingError>>) -> T {
    result.unwrap_or_else(|e| panic!("{}: {}", e.code(), written(|out| e.message(out))))
}
fn csr() -> Box<Csr> {
    let props = ok(CsrProperties::from_str(include_bytes!(
        "../../fatoora-core/tests/fixtures/csr-configs/csr-config-example-EN.properties"
    )));
    let key = ok(SigningKey::from_der(include_bytes!(
        "../../fatoora-core/tests/fixtures/sdk-parity/credentials/private-key.der"
    )));
    ok(props.build(&key, Environment::Simulation))
}
fn headers(value: String) -> Vec<(String, String)> {
    serde_json::from_str(&value).unwrap()
}
fn header(values: &[(String, String)], name: &str, expected: &str) {
    assert_eq!(
        values
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str()),
        Some(expected)
    );
}

#[test]
fn prepared_invoices_own_wire_data_and_decode_operation_specific_outcomes() {
    let config = ok(Config::new(Environment::NonProduction));
    let client = ok(ZatcaClient::new(&config));
    let production = ok(ProductionCsid::new(
        Environment::NonProduction,
        None,
        b"token",
        b"secret",
    ));
    let compliance = ok(ComplianceCsid::new(
        Environment::NonProduction,
        None,
        b"token",
        b"secret",
    ));
    let simple = ok(SignedInvoice::from_xml(SIMPLE));
    let standard = ok(SignedInvoice::from_xml(STANDARD));
    let requests = [
        (
            ok(client.prepare_report_simplified_invoice(
                &simple,
                &production,
                ClearanceMode::Disabled,
                Some(b"ar"),
            )),
            SIMPLE,
            "/invoices/reporting/single",
            "ar",
            Some("0"),
            "reportingStatus",
            "REPORTED",
            "NOT_REPORTED",
        ),
        (
            ok(client.prepare_clear_standard_invoice(
                &standard,
                &production,
                ClearanceMode::Enabled,
                None,
            )),
            STANDARD,
            "/invoices/clearance/single",
            "en",
            Some("1"),
            "clearanceStatus",
            "CLEARED",
            "NOT_CLEARED",
        ),
        (
            ok(client.prepare_check_invoice_compliance(&simple, &compliance)),
            SIMPLE,
            "/compliance/invoices",
            "en",
            None,
            "status",
            "PASS",
            "ERROR",
        ),
    ];
    drop((client, production, compliance, simple, standard));
    for (request, xml, path, language, clearance, field, accepted, rejected) in requests {
        assert_eq!(written(|out| ok(request.method(out))), "POST");
        assert!(written(|out| ok(request.url(out))).ends_with(path));
        let values = headers(written(|out| ok(request.headers_json(out))));
        header(&values, "Authorization", "Basic dG9rZW46c2VjcmV0");
        header(&values, "Accept-Version", "V2");
        header(&values, "Content-Type", "application/json");
        header(&values, "Accept-Language", language);
        if let Some(clearance) = clearance {
            header(&values, "Clearance-Status", clearance);
        } else {
            assert!(
                !values
                    .iter()
                    .any(|(key, _)| key.eq_ignore_ascii_case("Clearance-Status"))
            );
        }
        let owned_body = ok(request.body());
        let body: Value = serde_json::from_slice(owned_body.as_slice()).unwrap();
        let source = fatoora_core::invoice::xml::parse::parse_signed_invoice_xml(
            std::str::from_utf8(xml).unwrap(),
        )
        .unwrap();
        assert_eq!(
            body,
            json!({"uuid":source.uuid(), "invoiceHash":source.invoice_hash(), "invoice":Base64::encode_string(xml)})
        );
        for (value, outcome) in [(accepted, 1), (rejected, 2), ("FUTURE", 0)] {
            // Conflicting unrelated statuses must not determine this request's outcome.
            let mut body = json!({"validationResults":{"status":"PASS"}, "reportingStatus":"REPORTED", "clearanceStatus":"CLEARED"});
            if field == "status" {
                body["validationResults"][field] = value.into();
            } else {
                body[field] = value.into();
            }
            let bytes = serde_json::to_vec(&body).unwrap();
            let response = ok(request.decode(202, &bytes));
            assert_eq!(response.http_status(), Some(202));
            assert_eq!(response.outcome() as u8, outcome);
            assert_eq!(response.ensure_accepted().is_ok(), outcome == 1);
            let error = request.decode(422, &bytes).err().unwrap();
            assert_eq!(error.code(), 10);
            let details: Value =
                serde_json::from_str(&written(|out| error.details_json(out))).unwrap();
            assert_eq!(details["http_status"], 422);
            assert_eq!(details["response"], body);
        }
        assert!(request.decode(200, b"not json").is_err());
        assert!(request.decode(0, b"{}").is_err());
        drop(request);
        assert_eq!(
            serde_json::from_slice::<Value>(owned_body.as_slice()).unwrap(),
            body
        );
    }
}

#[test]
fn prepared_credentials_retain_environment_payload_and_renewal_semantics() {
    for environment in [
        Environment::NonProduction,
        Environment::Simulation,
        Environment::Production,
    ] {
        let config = ok(Config::new(environment));
        let client = ok(ZatcaClient::new(&config));
        let csr = csr();
        let expected_csr = Base64::encode_string(written(|out| ok(csr.to_pem(out))).as_bytes());
        let compliance = ok(ComplianceCsid::new(
            environment,
            Some(b"456"),
            b"token",
            b"secret",
        ));
        let production = ok(ProductionCsid::new(environment, None, b"token", b"secret"));
        let issuance = ok(client.prepare_request_compliance_csid(&csr, b"123456"));
        let promotion = ok(client.prepare_request_production_csid(&compliance));
        let renewal = ok(client.prepare_renew_csid(&production, &csr, b"654321", Some(b"ar")));
        drop((client, csr, compliance, production));
        assert_eq!(written(|out| ok(issuance.method(out))), "POST");
        assert!(written(|out| ok(issuance.url(out))).ends_with("/compliance"));
        let values = headers(written(|out| ok(issuance.headers_json(out))));
        header(&values, "OTP", "123456");
        header(&values, "Accept-Version", "V2");
        assert!(
            !values
                .iter()
                .any(|(key, _)| key.eq_ignore_ascii_case("Authorization"))
        );
        assert_eq!(
            serde_json::from_slice::<Value>(ok(issuance.body()).as_slice()).unwrap(),
            json!({"csr":expected_csr})
        );
        let decoded = ok(issuance.decode(201, CREDENTIALS));
        assert_eq!(decoded.env().ok(), Some(environment));
        assert_eq!(
            written(|out| ok(ok(decoded.request_id()).unwrap().value(out))),
            "123"
        );
        assert_eq!(
            written(|out| ok(decoded.binary_security_token(out))),
            "new-token"
        );
        assert_eq!(written(|out| ok(decoded.secret(out))), "new-secret");
        for (request, method, otp) in [
            (&promotion, "POST", None),
            (&renewal, "PATCH", Some("654321")),
        ] {
            assert_eq!(written(|out| ok(request.method(out))), method);
            assert!(written(|out| ok(request.url(out))).ends_with("/production/csids"));
            let values = headers(written(|out| ok(request.headers_json(out))));
            header(&values, "Authorization", "Basic dG9rZW46c2VjcmV0");
            header(&values, "Accept-Version", "V2");
            let expected = if let Some(otp) = otp {
                header(&values, "OTP", otp);
                header(&values, "Accept-Language", "ar");
                json!({"csr":expected_csr})
            } else {
                json!({"compliance_request_id":"456"})
            };
            assert_eq!(
                serde_json::from_slice::<Value>(ok(request.body()).as_slice()).unwrap(),
                expected
            );
            let decoded = ok(request.decode(200, CREDENTIALS));
            assert_eq!(decoded.env().ok(), Some(environment));
            assert_eq!(
                written(|out| ok(ok(decoded.request_id()).unwrap().value(out))),
                "123"
            );
            assert_eq!(
                written(|out| ok(decoded.binary_security_token(out))),
                "new-token"
            );
            assert_eq!(written(|out| ok(decoded.secret(out))), "new-secret");
            for (status, body) in [
                (0, CREDENTIALS),
                (200, b"bad".as_slice()),
                (401, b"bad"),
                (500, b"bad"),
                (400, b"{}"),
                (200, b"{}"),
            ] {
                assert!(request.decode(status, body).is_err());
                assert!(issuance.decode(status, body).is_err());
            }
        }
        let wrapped = serde_json::to_vec(
            &json!({"value":serde_json::from_slice::<Value>(CREDENTIALS).unwrap()}),
        )
        .unwrap();
        assert_eq!(
            ok(renewal.decode(428, &wrapped)).env().ok(),
            Some(environment)
        );
        assert!(promotion.decode(428, &wrapped).is_err());
        assert!(issuance.decode(428, &wrapped).is_err());
    }
}

#[test]
fn prepare_rejects_bad_inputs_and_consumed_owners_without_transport() {
    let config = ok(Config::new(Environment::NonProduction));
    let client = ok(ZatcaClient::new(&config));
    let csr = csr();
    let mut simple = ok(SignedInvoice::from_xml(SIMPLE));
    let standard = ok(SignedInvoice::from_xml(STANDARD));
    let compliance = ok(ComplianceCsid::new(
        Environment::NonProduction,
        None,
        b"token",
        b"secret",
    ));
    let production = ok(ProductionCsid::new(
        Environment::NonProduction,
        None,
        b"token",
        b"secret",
    ));
    assert!(client.prepare_request_production_csid(&compliance).is_err());
    for invalid in [b"\xff".as_slice(), b"a\0b", b"a\r\nb"] {
        assert!(
            client
                .prepare_request_compliance_csid(&csr, invalid)
                .is_err()
        );
        assert!(
            client
                .prepare_renew_csid(&production, &csr, invalid, None)
                .is_err()
        );
    }
    for invalid in [b"\xff".as_slice(), b"a\0b"] {
        assert!(
            client
                .prepare_report_simplified_invoice(
                    &simple,
                    &production,
                    ClearanceMode::Disabled,
                    Some(invalid)
                )
                .is_err()
        );
        assert!(
            client
                .prepare_clear_standard_invoice(
                    &standard,
                    &production,
                    ClearanceMode::Disabled,
                    Some(invalid)
                )
                .is_err()
        );
        assert!(
            client
                .prepare_renew_csid(&production, &csr, b"123", Some(invalid))
                .is_err()
        );
    }
    assert!(
        client
            .prepare_report_simplified_invoice(
                &standard,
                &production,
                ClearanceMode::Disabled,
                None
            )
            .is_err()
    );
    assert!(
        client
            .prepare_clear_standard_invoice(&simple, &production, ClearanceMode::Disabled, None)
            .is_err()
    );
    let foreign = ok(ProductionCsid::new(
        Environment::Simulation,
        None,
        b"token",
        b"secret",
    ));
    let foreign_compliance = ok(ComplianceCsid::new(
        Environment::Simulation,
        Some(b"id"),
        b"token",
        b"secret",
    ));
    assert!(
        client
            .prepare_report_simplified_invoice(&simple, &foreign, ClearanceMode::Disabled, None)
            .is_err()
    );
    assert!(
        client
            .prepare_clear_standard_invoice(&standard, &foreign, ClearanceMode::Disabled, None)
            .is_err()
    );
    assert!(
        client
            .prepare_renew_csid(&foreign, &csr, b"123", None)
            .is_err()
    );
    assert!(
        client
            .prepare_check_invoice_compliance(&simple, &foreign_compliance)
            .is_err()
    );
    assert!(
        client
            .prepare_request_production_csid(&foreign_compliance)
            .is_err()
    );
    let prepared = ok(client.prepare_report_simplified_invoice(
        &simple,
        &production,
        ClearanceMode::Disabled,
        None,
    ));
    written(|out| ok(simple.into_xml(out)));
    assert!(
        client
            .prepare_check_invoice_compliance(&simple, &compliance)
            .is_err()
    );
    assert!(
        client
            .prepare_report_simplified_invoice(&simple, &production, ClearanceMode::Disabled, None)
            .is_err()
    );
    assert!(
        client
            .prepare_clear_standard_invoice(&simple, &production, ClearanceMode::Disabled, None)
            .is_err()
    );
    assert!(!ok(prepared.body()).as_slice().is_empty());
}

use super::*;
use base64ct::Encoding;
use httpmock::{Method::GET, MockServer};

async fn receive(
    status: u16,
    body: &str,
    operation: InvoiceOperation,
) -> Result<ValidationResponse, ZatcaError> {
    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET).path("/");
            then.status(status).body(body);
        })
        .await;
    let response = reqwest::get(server.url("/")).await.unwrap();
    let result = read_validation_response(response, operation).await;
    mock.assert_async().await;
    result
}

#[tokio::test]
async fn response_contract_preserves_rejections_and_status() {
    let body = r#"{"validationResults":{"status":"ERROR","errorMessages":[{"code":"BR-KSA-37","message":"invalid address"}]},"reportingStatus":"NOT_REPORTED"}"#;
    for status in [400, 409, 401, 403, 406, 429, 500, 503] {
        let err = receive(status, body, InvoiceOperation::Reporting)
            .await
            .unwrap_err();
        assert_eq!(err.http_status(), Some(status));
        let ZatcaError::Response(response) = &err else {
            panic!("{err:?}")
        };
        assert_eq!(response.body(), body);
        assert_eq!(
            response
                .validation_response()
                .unwrap()
                .validation_results()
                .error_messages()[0]
                .code(),
            Some("BR-KSA-37")
        );
        let details: serde_json::Value =
            serde_json::from_str(&crate::Error::from(err).details_json()).unwrap();
        assert_eq!(details["http_status"], status);
        assert_eq!(
            details["response"]["validationResults"]["errorMessages"][0]["code"],
            "BR-KSA-37"
        );
    }
}

#[tokio::test]
async fn response_contract_http_success_is_separate_from_acceptance() {
    for (operation, status_field, accepted, rejected) in [
        (
            InvoiceOperation::Reporting,
            "reportingStatus",
            "REPORTED",
            "NOT_REPORTED",
        ),
        (
            InvoiceOperation::Clearance,
            "clearanceStatus",
            "CLEARED",
            "NOT_CLEARED",
        ),
        (InvoiceOperation::Compliance, "status", "PASS", "ERROR"),
    ] {
        for (value, expected) in [
            (accepted, InvoiceOutcome::Accepted),
            (rejected, InvoiceOutcome::Rejected),
            ("FUTURE", InvoiceOutcome::Unknown),
        ] {
            let mut body = serde_json::json!({"validationResults":{"status":"WARNING", "warningMessages":[{"code":"WARNING"}]}});
            if operation == InvoiceOperation::Compliance {
                body["validationResults"][status_field] = value.into();
            } else {
                body[status_field] = value.into();
            }
            let response = receive(202, &body.to_string(), operation).await.unwrap();
            assert_eq!(response.http_status(), Some(202));
            assert_eq!(response.outcome(), expected);
            assert_eq!(
                response.ensure_accepted().is_ok(),
                expected == InvoiceOutcome::Accepted
            );
        }
    }
    let response = receive(
        200,
        r#"{"validationResults":{"status":"PASS"},"clearanceStatus":"CLEARED"}"#,
        InvoiceOperation::Reporting,
    )
    .await
    .unwrap();
    assert_eq!(response.outcome(), InvoiceOutcome::Unknown);
    let detached: ValidationResponse = serde_json::from_str(
        r#"{"validationResults":{"status":"PASS"},"reportingStatus":"REPORTED"}"#,
    )
    .unwrap();
    assert_eq!(detached.http_status(), None);
    assert_eq!(detached.outcome(), InvoiceOutcome::Unknown);
}

#[tokio::test]
async fn response_contract_preserves_cleared_xml_and_decode_failures() {
    let xml = "<Invoice>\n  exact content\n</Invoice>\n";
    let encoded = base64ct::Base64::encode_string(xml.as_bytes());
    for value in [
        Some(encoded.as_str()),
        None,
        Some("%%%"),
        Some("/w=="),
        Some("AA=="),
        Some(""),
    ] {
        let body = serde_json::json!({"validationResults":{},"clearanceStatus":"CLEARED","clearedInvoice":value});
        let response = receive(200, &body.to_string(), InvoiceOperation::Clearance)
            .await
            .unwrap();
        assert_eq!(response.cleared_invoice_base64(), value);
        assert_eq!(response.outcome(), InvoiceOutcome::Accepted);
        match value {
            Some(v) if v == encoded => assert_eq!(
                response.cleared_invoice_xml().unwrap().as_deref(),
                Some(xml)
            ),
            None => assert_eq!(response.cleared_invoice_xml().unwrap(), None),
            _ => {
                let err = response.cleared_invoice_xml().unwrap_err();
                assert_eq!(err.kind(), crate::ErrorKind::Parse);
                assert_eq!(err.http_status(), Some(200));
            }
        }
    }
}

#[tokio::test]
async fn response_contract_keeps_malformed_bodies() {
    for status in [200, 204, 400, 502] {
        for body in ["", "<html>gateway failure</html>", "{broken"] {
            if status == 204 && !body.is_empty() {
                continue;
            }
            let err = receive(status, body, InvoiceOperation::Reporting)
                .await
                .unwrap_err();
            assert_eq!(err.http_status(), Some(status));
            let details: serde_json::Value =
                serde_json::from_str(&crate::Error::from(err).details_json()).unwrap();
            assert_eq!(details["body"], body);
        }
    }
}

#[tokio::test]
async fn response_contract_redirect_is_not_followed() {
    let server = MockServer::start_async().await;
    let target = server
        .mock_async(|when, then| {
            when.path("/target");
            then.status(200);
        })
        .await;
    let redirect = server
        .mock_async(|when, then| {
            when.path("/redirect");
            then.status(303)
                .header("Location", server.url("/target"))
                .body("clearance disabled");
        })
        .await;
    let client = ZatcaClient::new(Config::default()).unwrap();
    let response = client
        ._client
        .get(server.url("/redirect"))
        .send()
        .await
        .unwrap();
    let err = read_validation_response(response, InvoiceOperation::Clearance)
        .await
        .unwrap_err();
    assert_eq!(err.http_status(), Some(303));
    redirect.assert_async().await;
    target.assert_hits_async(0).await;
}

#[tokio::test]
async fn response_contract_duplicate_clearance_and_contradictions() {
    let body = r#"{"validationResults":{"status":"WARNING","warningMessages":[{"code":"warning"}]},"clearanceStatus":"CLEARED","clearedInvoice":"PEludm9pY2UvPg=="}"#;
    let response = receive(208, body, InvoiceOperation::Clearance)
        .await
        .unwrap();
    assert_eq!(response.http_status(), Some(208));
    response.ensure_accepted().unwrap();
    assert_eq!(
        response.cleared_invoice_xml().unwrap().as_deref(),
        Some("<Invoice/>")
    );
    for body in [
        r#"{"validationResults":{"status":"ERROR"},"reportingStatus":"REPORTED"}"#,
        r#"{"validationResults":{"status":"PASS","errorMessages":[{"code":"error"}]},"reportingStatus":"REPORTED"}"#,
    ] {
        let response = receive(200, body, InvoiceOperation::Reporting)
            .await
            .unwrap();
        assert_eq!(response.outcome(), InvoiceOutcome::Unknown);
        let err = response.ensure_accepted().unwrap_err();
        assert_eq!(err.http_status(), Some(200));
    }
    let response = receive(
        200,
        r#"{"validationResults":{"status":"WARNING"}}"#,
        InvoiceOperation::Compliance,
    )
    .await
    .unwrap();
    response.ensure_accepted().unwrap();
    let detached: ValidationResponse = serde_json::from_str(r#"{"validationResults":{},"http_status":200,"operation":"Reporting","reportingStatus":"REPORTED"}"#).unwrap();
    assert_eq!(detached.http_status(), None);
    assert_eq!(detached.outcome(), InvoiceOutcome::Unknown);
}

#[tokio::test]
async fn response_contract_body_read_failure_keeps_status() {
    use std::io::{BufRead, BufReader, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let mut request = BufReader::new(&mut stream);
        let mut line = String::new();
        loop {
            line.clear();
            assert!(request.read_line(&mut line).unwrap() > 0);
            if line == "\r\n" {
                break;
            }
        }
        stream
            .write_all(
                b"HTTP/1.1 200 OK\r\nContent-Length: 10000\r\nConnection: close\r\n\r\n{partial",
            )
            .unwrap();
    });
    let response = reqwest::get(format!("http://{address}")).await.unwrap();
    let err = read_validation_response(response, InvoiceOperation::Reporting)
        .await
        .unwrap_err();
    server.join().unwrap();
    assert!(matches!(
        err,
        ZatcaError::ResponseRead {
            http_status: 200,
            ..
        }
    ));
    assert_eq!(err.kind(), crate::ErrorKind::Network);
}

#[tokio::test]
async fn response_contract_all_invoice_endpoints_reject_http_400() {
    let server = MockServer::start_async().await;
    let mut client = ZatcaClient::new(Config::default()).unwrap();
    client.base_url = format!("{}/", server.base_url());
    let body = r#"{"validationResults":{"status":"ERROR","errorMessages":[{"code":"REJECTED"}]}}"#;
    let mock = server
        .mock_async(|when, then| {
            when.method(httpmock::Method::POST);
            then.status(400).body(body);
        })
        .await;
    let simplified = super::tests::build_signed_invoice(crate::invoice::InvoiceType::Tax(
        crate::invoice::InvoiceSubType::Simplified,
    ));
    let standard = super::tests::build_signed_invoice(crate::invoice::InvoiceType::Tax(
        crate::invoice::InvoiceSubType::Standard,
    ));
    let pcsid = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");
    let ccsid = CsidCredentials::new(Environment::NonProduction, None, "token", "secret");
    assert_eq!(
        client
            .report_simplified_invoice(&simplified, &pcsid, false, None)
            .await
            .unwrap_err()
            .http_status(),
        Some(400)
    );
    assert_eq!(
        client
            .clear_standard_invoice(&standard, &pcsid, true, None)
            .await
            .unwrap_err()
            .http_status(),
        Some(400)
    );
    assert_eq!(
        client
            .check_invoice_compliance(&simplified, &ccsid)
            .await
            .unwrap_err()
            .http_status(),
        Some(400)
    );
    mock.assert_hits_async(3).await;
}

#[tokio::test]
async fn native_csid_body_read_failures_keep_legacy_classification() {
    use std::io::{Read, Write};
    for (policy, status, empty) in [
        (BodyReadPolicy::Invoice, 200, false),
        (BodyReadPolicy::Issuance, 200, false),
        (BodyReadPolicy::Issuance, 401, true),
        (BodyReadPolicy::Renewal, 200, true),
        (BodyReadPolicy::Renewal, 428, true),
    ] {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0; 4096];
            let _ = stream.read(&mut request).unwrap();
            write!(
                stream,
                "HTTP/1.1 {status} Test\r\nContent-Length: 100\r\nConnection: close\r\n\r\nx"
            )
            .unwrap();
        });
        let client = ZatcaClient::new(Config::default()).unwrap();
        let request = HttpRequest::new("GET", format!("http://{address}/"));
        let result = client.send(&request, policy).await;
        server.join().unwrap();
        if empty {
            assert_eq!(result.unwrap(), (status, vec![]));
        } else if matches!(policy, BodyReadPolicy::Invoice) {
            assert!(matches!(result, Err(ZatcaError::ResponseRead { .. })));
        } else {
            assert!(matches!(result, Err(ZatcaError::InvalidResponse(_))));
        }
    }
}

fn all_messages(results: &ValidationResults) -> Vec<&ValidationMessage> {
    let info: Vec<&ValidationMessage> = match results.info_messages() {
        MessageList::One(message) => vec![message],
        MessageList::Many(messages) => messages.iter().collect(),
        MessageList::Empty => Vec::new(),
    };
    info.into_iter()
        .chain(results.warning_messages())
        .chain(results.error_messages())
        .collect()
}

#[test]
fn status_enums_parse_every_value_in_the_live_capture() {
    let capture: serde_json::Value = serde_json::from_str(include_str!(
        "../../tests/fixtures/qr-correctness/development-api.json"
    ))
    .unwrap();
    let mut seen = std::collections::BTreeSet::new();
    for (name, case) in capture["cases"].as_object().unwrap() {
        let response: ValidationResponse =
            serde_json::from_value(case["response"].clone()).unwrap();
        let reporting = response.reporting_status();
        let clearance = response.clearance_status();
        assert!(
            reporting.is_some() != clearance.is_some(),
            "{name}: exactly one of reporting/clearance status"
        );
        if let Some(status) = reporting {
            assert!(!matches!(status, ReportingStatus::Other(_)), "{name}");
            seen.insert(status.to_string());
        }
        if let Some(status) = clearance {
            assert!(!matches!(status, ClearanceStatus::Other(_)), "{name}");
            seen.insert(status.to_string());
        }
        let results = response.validation_results();
        let status = results.status().expect("validation status");
        assert!(!matches!(status, ValidationStatus::Other(_)), "{name}");
        seen.insert(format!("results:{status}"));
        for message in all_messages(results) {
            let kind = message.message_type().expect("message type");
            let status = message.status().expect("message status");
            assert!(!matches!(kind, MessageType::Other(_)), "{name}: {kind}");
            assert!(!matches!(status, ValidationStatus::Other(_)), "{name}");
            seen.insert(format!("type:{kind}/status:{status}"));
        }
        // Serializing reproduces the captured wire values.
        let reserialized = serde_json::to_value(&response).unwrap();
        for field in ["reportingStatus", "clearanceStatus"] {
            if let Some(value) = case["response"].get(field) {
                assert_eq!(&reserialized[field], value, "{name}: {field}");
            }
        }
        assert_eq!(
            reserialized["validationResults"]["status"],
            case["response"]["validationResults"]["status"]
        );
    }
    for expected in [
        "REPORTED",
        "NOT_REPORTED",
        "CLEARED",
        "results:PASS",
        "results:WARNING",
        "results:ERROR",
        "type:INFO/status:PASS",
        "type:WARNING/status:WARNING",
        "type:ERROR/status:ERROR",
    ] {
        assert!(seen.contains(expected), "capture lacks {expected}");
    }
}

#[test]
fn status_enums_map_known_wire_values() {
    for (wire, status) in [
        ("REPORTED", ReportingStatus::Reported),
        ("NOT_REPORTED", ReportingStatus::NotReported),
    ] {
        assert_eq!(ReportingStatus::from(wire), status);
        assert_eq!(status.as_str(), wire);
    }
    for (wire, status) in [
        ("CLEARED", ClearanceStatus::Cleared),
        ("NOT_CLEARED", ClearanceStatus::NotCleared),
    ] {
        assert_eq!(ClearanceStatus::from(wire.to_owned()), status);
        assert_eq!(status.to_string(), wire);
    }
    for (wire, status) in [
        ("PASS", ValidationStatus::Pass),
        ("WARNING", ValidationStatus::Warning),
        ("ERROR", ValidationStatus::Error),
    ] {
        assert_eq!(ValidationStatus::from(wire), status);
        assert_eq!(status.as_str(), wire);
    }
    for (wire, kind) in [
        ("INFO", MessageType::Info),
        ("WARNING", MessageType::Warning),
        ("ERROR", MessageType::Error),
    ] {
        assert_eq!(wire.parse::<MessageType>(), Ok(kind.clone()));
        assert_eq!(kind.to_string(), wire);
    }
}

#[test]
fn status_enums_preserve_unknown_values() {
    for raw in ["WARN", "reported", "", " CLEARED", "NEW_STATUS"] {
        let reporting = ReportingStatus::from(raw);
        let clearance = ClearanceStatus::from(raw.to_owned());
        let validation: ValidationStatus = raw.parse().unwrap();
        let kind = MessageType::from(raw);
        assert_eq!(reporting, ReportingStatus::Other(raw.into()));
        assert_eq!(clearance, ClearanceStatus::Other(raw.into()));
        assert_eq!(validation, ValidationStatus::Other(raw.into()));
        assert_eq!(kind, MessageType::Other(raw.into()));
        assert_eq!(reporting.as_str(), raw);
        assert_eq!(clearance.to_string(), raw);
        assert_eq!(validation.as_str(), raw);
        assert_eq!(kind.to_string(), raw);
    }
}

#[test]
fn status_enums_serde_round_trip_as_plain_strings() {
    let body = r#"{"clearedInvoice":null,"validationResults":{"infoMessages":{"type":"NOTICE","code":"c","category":null,"message":null,"status":"PASS"},"warningMessages":[{"type":"WARNING","code":null,"category":null,"message":null,"status":"WARN"}],"errorMessages":[],"status":"PARTIAL"},"reportingStatus":"QUEUED","clearanceStatus":"NOT_CLEARED","qrSellertStatus":null,"qrBuyertStatus":null}"#;
    let response: ValidationResponse = serde_json::from_str(body).unwrap();
    assert_eq!(
        response.reporting_status(),
        Some(&ReportingStatus::Other("QUEUED".into()))
    );
    assert_eq!(
        response.clearance_status(),
        Some(&ClearanceStatus::NotCleared)
    );
    let results = response.validation_results();
    assert_eq!(
        results.status(),
        Some(&ValidationStatus::Other("PARTIAL".into()))
    );
    let MessageList::One(info) = results.info_messages() else {
        panic!("expected one info message");
    };
    assert_eq!(
        info.message_type(),
        Some(&MessageType::Other("NOTICE".into()))
    );
    assert_eq!(info.status(), Some(&ValidationStatus::Pass));
    let warning = &results.warning_messages()[0];
    assert_eq!(warning.message_type(), Some(&MessageType::Warning));
    assert_eq!(
        warning.status(),
        Some(&ValidationStatus::Other("WARN".into()))
    );
    assert_eq!(serde_json::to_string(&response).unwrap(), body);

    for status in [ValidationStatus::Error, ValidationStatus::Other("x".into())] {
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, format!("\"{}\"", status.as_str()));
        assert_eq!(
            serde_json::from_str::<ValidationStatus>(&json).unwrap(),
            status
        );
    }
    assert!(serde_json::from_str::<ReportingStatus>("1").is_err());
}

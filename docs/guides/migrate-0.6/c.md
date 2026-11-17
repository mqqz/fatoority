---
title: Migrate C and C++ Code from fatoority 0.5 to 0.6
description: Upgrade C and C++ code from the fatoority 0.5 native library to 0.6. Functions follow the Rust core names, enums are typed and validation drops the Config argument.
---

# Migrate from 0.5 to 0.6: C and C++

The C facade (`fatoora.h`) and the C++ headers follow the core names in 0.6 and
use typed enums. There are no compatibility aliases: rebuild against the 0.6
headers and fix each compile error. Take `fatoora.h`, `fatoora.c`, the
generated headers and the shared library from the same 0.6.0
[release assets](https://github.com/mqqz/fatoority/releases); mixing 0.5
headers with a 0.6 library, or the reverse, is not supported.

## Upgrade checklist

1. Rename the functions and types in the [table below](#renamed-functions).
2. Drop the `FatooraConfig *` argument from `fatoora_xml_validate` and
   `fatoora_xml_validate_zatca`. Validation never depended on the environment.
3. Read the ZATCA validation result from the returned
   `FatooraZatcaValidationReport *` instead of parsing JSON, or call
   `fatoora_zatca_validation_report_to_json` for the 0.5 text.
4. Pass `FATOORA_CLEARANCE_DISABLED`/`FATOORA_CLEARANCE_ENABLED` instead of
   `false`/`true`, and update output variables to the typed enums, such as
   `FatooraVatCategory` instead of `uint8_t`.
5. Update response status reads: the status getters now write an enum plus
   `bool *has_value`; the new `_raw` getters return the 0.5 text.

The existing constants (`FATOORA_NON_PRODUCTION`, `FATOORA_TAX`,
`FATOORA_VAT_STANDARD`, `FATOORA_FLAG_EXPORT`, ...) keep their values, so code
that already used them only needs the renames. Passing `0`/`1` or
`false`/`true` still compiles; any value that is not a declared enumerator
returns `FATOORA_INVALID_INPUT`.

## Validate an invoice

```diff
-FatooraConfig *config = NULL;
-char *report = NULL;
-TRY(fatoora_config_new(FATOORA_NON_PRODUCTION, &config, &error));
-TRY(fatoora_xml_validate(config, xml, &valid, &error));
-TRY(fatoora_xml_validate_zatca(config, xml, options, &report, &error));
-puts(report); /* Parse is_valid with a JSON parser. */
-fatoora_string_free(report);
-fatoora_config_free(config);
+FatooraZatcaValidationReport *report = NULL;
+char *json = NULL;
+TRY(fatoora_xml_validate(xml, &valid, &error));
+TRY(fatoora_xml_validate_zatca(xml, options, &report, &error));
+TRY(fatoora_zatca_validation_report_is_valid(report, &valid, &error));
+TRY(fatoora_zatca_validation_report_to_json(report, &json, &error));
+puts(json);
+fatoora_string_free(json);
+fatoora_zatca_validation_report_free(report);
```

The report also has `_is_complete`, `_has_errors`, `_stages_len` and
`_stage(index)`, with typed findings on each stage report. See
[invoice validation](../../reference/invoice-validation.md#report-object-in-the-generated-bindings).

## Sign and report an invoice

```diff
-FatooraSigner *signer = NULL;
-FatooraCsidProduction *credentials = NULL;
-TRY(fatoora_signer_from_pem(cert, key, &signer, &error));
-TRY(fatoora_signer_sign(signer, invoice, &signed_invoice, &error));
-TRY(fatoora_csid_production_create(FATOORA_NON_PRODUCTION, NULL,
-    token, secret, &credentials, &error));
-TRY(fatoora_zatca_client_report_simplified_invoice(client, signed_invoice,
-    credentials, false, "en", &response, &error));
-TRY(fatoora_validation_response_reporting_status(response, &status, &error)); /* char * */
-fatoora_csid_production_free(credentials);
-fatoora_signer_free(signer);
+FatooraInvoiceSigner *signer = NULL;
+FatooraProductionCsid *credentials = NULL;
+FatooraReportingStatus status;
+bool has_status = false;
+TRY(fatoora_invoice_signer_from_pem(cert, key, &signer, &error));
+TRY(fatoora_invoice_signer_sign(signer, invoice, &signed_invoice, &error));
+TRY(fatoora_production_csid_new(FATOORA_NON_PRODUCTION, NULL,
+    token, secret, &credentials, &error));
+TRY(fatoora_zatca_client_report_simplified_invoice(client, signed_invoice,
+    credentials, FATOORA_CLEARANCE_DISABLED, "en", &response, &error));
+TRY(fatoora_validation_response_reporting_status(response, &status, &has_status, &error));
+TRY(fatoora_validation_response_reporting_status_raw(response, &raw_status, &error)); /* char * */
+fatoora_production_csid_free(credentials);
+fatoora_invoice_signer_free(signer);
```

`fatoora_zatca_client_with_timeout(config, timeout_ms, &client, &error)`
is new: it creates a client whose requests fail with a network error after
`timeout_ms` milliseconds. `fatoora_zatca_client_new` still applies no
timeout.

## Document allowances and payment means

The builder's aggregate setters are replaced by explicit entries:

```diff
-TRY(fatoora_invoice_builder_set_payment_means_code(builder, "10", &error));
-TRY(fatoora_invoice_builder_invoice_level_discount(builder, "10", &error));
-TRY(fatoora_invoice_builder_allowance_reason(builder, "Loyalty discount", &error));
+TRY(fatoora_payment_means_new("10", NULL, &means, &error));
+TRY(fatoora_invoice_builder_add_payment_means(builder, means, &error));
+TRY(fatoora_adjustment_amount("10", &amount, &error));
+TRY(fatoora_adjustment_with_reason(amount, "Loyalty discount", NULL, &discount, &error));
+TRY(fatoora_tax_exemption_new(NULL, NULL, &exemption, &error));
+TRY(fatoora_invoice_builder_add_allowance(builder, discount, FATOORA_VAT_STANDARD, "15",
+    exemption, &error));
```

Each `add_allowance`/`add_charge` call appends one entry in the VAT category
and at the rate you pass. Free `means`, `amount`, `discount` and `exemption`
with their `_free` functions when done.

## Renamed functions

| 0.5 | 0.6 |
| --- | --- |
| `FatooraCsidCompliance`, `fatoora_csid_compliance_*` | `FatooraComplianceCsid`, `fatoora_compliance_csid_*` |
| `FatooraCsidProduction`, `fatoora_csid_production_*` | `FatooraProductionCsid`, `fatoora_production_csid_*` |
| `FatooraSigner`, `fatoora_signer_*` | `FatooraInvoiceSigner`, `fatoora_invoice_signer_*` |
| `fatoora_zatca_client_create`, `fatoora_csid_compliance_create`, `fatoora_csid_production_create` | `fatoora_zatca_client_new`, `fatoora_compliance_csid_new`, `fatoora_production_csid_new` (every constructor is `_new`) |
| `fatoora_adjustment_percentage(base, percent, ...)` | `fatoora_adjustment_percent_of(base, percent, ...)`; `fatoora_adjustment_percentage` now reads the percentage |
| `fatoora_adjustment_percent` | `fatoora_adjustment_percentage` |
| `fatoora_adjustment_with_supplied_amount` | `fatoora_adjustment_supplied_amount` |
| `fatoora_allowance_charge_without_vat_rate`, `fatoora_vat_breakdown_without_vat_rate` | `fatoora_allowance_charge_omit_vat_rate`, `fatoora_vat_breakdown_omit_vat_rate` |
| `fatoora_zatca_client_post_csr_for_ccsid`, `_post_ccsid_for_pcsid` | `fatoora_zatca_client_request_compliance_csid`, `_request_production_csid` |
| `fatoora_zatca_client_prepare_post_csr_for_ccsid`, `_prepare_post_ccsid_for_pcsid` | `fatoora_zatca_client_prepare_request_compliance_csid`, `_prepare_request_production_csid` |
| `fatoora_csr_properties_from_properties_str`, `_parse_csr_config_file` | `fatoora_csr_properties_from_str`, `_from_file` |
| `fatoora_finalized_invoice_hash_base64`, `fatoora_signed_invoice_hash_base64` | `fatoora_finalized_invoice_hash`, `fatoora_signed_invoice_hash` |
| `fatoora_invoice_builder_flags`, `fatoora_invoice_data_flags_raw` | `fatoora_invoice_builder_set_flags`, `fatoora_invoice_data_flags` |
| `fatoora_invoice_builder_set_tax_exemption` | `fatoora_invoice_builder_add_tax_exemption` |
| `fatoora_invoice_builder_add_document_adjustment(builder, charge, ...)` | `fatoora_invoice_builder_add_allowance(builder, ...)` or `_add_charge` |
| `fatoora_invoice_builder_set_note`, `_set_note_without_language` | `fatoora_invoice_builder_add_note(builder, language or NULL, text, ...)` |
| `fatoora_invoice_builder_set_payment_means_code` | `fatoora_payment_means_new` plus `fatoora_invoice_builder_add_payment_means` |
| `fatoora_invoice_builder_set_allowance`, `_allowance_reason`, `_invoice_level_charge`, `_invoice_level_discount` | `fatoora_invoice_builder_add_allowance`/`_add_charge` with an adjustment carrying the reason |
| `fatoora_allowance_charge_is_charge` | `fatoora_allowance_charge_kind` (writes `FatooraAdjustmentKind`) |
| `fatoora_invoice_line_item_adjustment_is_charge` | `fatoora_invoice_line_item_adjustment_kind` |
| `fatoora_xml_validate(config, xml, ...)` | `fatoora_xml_validate(xml, ...)` |
| `fatoora_xml_validate_zatca(config, xml, options, char **out, ...)` | `fatoora_xml_validate_zatca(xml, options, FatooraZatcaValidationReport **out, ...)` |

New status getters: `fatoora_validation_response_reporting_status_raw`,
`_clearance_status_raw`, `fatoora_validation_results_status_raw`,
`fatoora_validation_message_message_type_raw` and `_status_raw`. Credential
`_request_id` getters write `NULL` when the credentials have no request ID.

## C++

The C++ headers under `bindings/cpp` are generated from the same bridge and
follow the same renames in snake_case: `fatoora::InvoiceSigner`,
`fatoora::ComplianceCsid`, `fatoora::ProductionCsid`,
`ZatcaClient::request_compliance_csid`, `CsrProperties::from_str`,
`FinalizedInvoice::hash`, `InvoiceBuilder::set_flags`, `add_allowance` and
`add_charge`, `ZatcaClient::new_` (formerly `create`), `Adjustment::percent_of`,
`supplied_amount` and `percentage`, and `omit_vat_rate`. Enum parameters take `fatoora::Environment`,
`fatoora::VatCategory`, `fatoora::ClearanceMode` and the other enum classes
instead of integers. `fatoora::Xml::validate(xml)` and
`fatoora::Xml::validate_zatca(xml, options_json)` take no `Config`, and the
latter returns a `fatoora::ZatcaValidationReport`. The raw Diplomat headers do
not validate enum values: passing a value that is not a declared enumerator is
undefined behaviour.

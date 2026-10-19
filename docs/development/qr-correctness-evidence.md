---
title: QR correctness evidence and workflow limits
description: Published requirements, pinned SDK comparisons, native integrity policies and migration notes for QR correctness.
---

# QR correctness evidence

## Requirements and observed behavior

The [May 2023 ZATCA security standard](https://www.zatca.gov.sa/ar/E-Invoicing/SystemsDevelopers/Documents/20230519_ZATCA_Electronic_Invoice_Security_Features_Implementation_Standards_vF.pdf), section 4.1, lists Tags 7–8 for stamped invoices and restricts Tag 9 to simplified invoices and their notes. Section 2.2.1 distinguishes platform clearance stamping of standard invoices from EGS stamping of simplified invoices.

The pinned SDK `238-R3.4.8` independently generated Tags 1–8 for standard tax invoices, credit notes and debit notes; simplified equivalents contained Tags 1–9. Fatoority produced the same tag sequences and decoded values for Tags 4, 5 and 9. Tag 9 contains the certificate's binary signature bytes. Certificate signature metadata remains available for standard documents.

These standard cases are local SDK signing exercises. They are not gateway-cleared documents and establish no platform issuer trust. The original standard XML samples contain Tags 1–8. One foreign-currency sample fails strict Tag 5 comparison as described below. No newly obtained production clearance fixture or live clearance submission was used. Imported certificates are checked against their document's signature and QR; they need not match a locally configured signer.

## Findings and changes

| Finding | Root cause | Changed paths |
| --- | --- | --- |
| QR-001 | Certificate metadata was treated as unconditional QR content | `invoice/qr.rs` centralizes subtype and required stamp tags; `invoice/sign.rs` supplies Tag 9 only for simplified documents |
| QR-002 | The model QR constructor formatted money independently of source XML | Typed signing now serializes first; the obsolete model constructor was removed. XML construction preserves decimal text |
| QR-003 | `sign_with_bundle()` reconstructed a second QR from typed totals | `invoice/builder.rs` retains the payload extracted and checked against the final XML. Signed CLI commands use that payload |
| QR-004 | Invalid Base64 was unwrapped or replaced with empty bytes; Tag 9 TLV errors were ignored | `invoice/qr.rs` bounds binary inputs and propagates decoding and field-length errors through structured QR errors |
| QR-005 | The integrity decoder required nine tags and the import decoder had different framing checks | `invoice/xml/parse.rs` and `validation/integrity.rs` share bounded decoding and explicit subtype applicability. The SDK report still skips standard signature/QR stages |
| QR-006 | Tag 4 used byte equality while Tag 5 used decimal equality | Both fields use the existing bounded exact XML-decimal implementation. A separate SDK-profile check retains Tag 4 lexical matching. BT-110 selects the document-currency tax amount |

Signed imports now verify signature, digest, certificate metadata and QR consistency. They preserve the supplied XML and reject mismatches. The original XML signature verifier and signing postconditions remain in use. No certificate trust or revocation checks were added.

## Decimal observations

For each of `300`, `300.0`, `300.00`, `300.01`, `0.00`, `45.0`, `45.00` and `123456789012345678901234.56`, both generators retained the supplied XML spelling in Tags 4 and 5. The pinned SDK accepted those QR fields on Fatoority-signed XML. The decimal probes deliberately alter totals without rebuilding invoice lines; some therefore fail business rules. QR acceptance is a separate result.

The SDK and development API reject QR Tag 4 `115.0` against XML `115.00`. The underlying exact comparison recognizes monetary equality, while a separate SDK-profile formatting check rejects the different spelling under `SDK-QR-004`. Signed import checks monetary integrity; full-profile validation remains a separate operation. The frozen `qr-total-scale` SDK output and native SDK-profile expectation are unchanged. Tag 5 retains `SDK-QR-003`, including rejection of NaN, exponent syntax and float-collision mismatches. Business-rule scale checks remain separate.

The sample `Standard/Invoice/Advance Payment adjustments with foreign currency invoice.xml` has document currency USD, accounting VAT `1068.75` SAR and invoice VAT `285` USD. Its original QR uses `1068.75` in Tag 5. The published security standard names BT-110; the [XML standard](https://zatca.gov.sa/ar/E-Invoicing/SystemsDevelopers/Documents/20230519_ZATCA_Electronic_Invoice_XML_Implementation_Standard_%20vF.pdf), sections 9.2 and BR-KSA-CL-02, distinguishes BT-110 from accounting-currency BT-111. Fatoority selects `285` and rejects the original sample on strict import under `SDK-QR-005`. This selection follows the published business-term interpretation. The pinned SDK regenerated `1068.75`; this observed difference is recorded in the comparison report. Standard SDK-profile QR validation remains inapplicable.

## Reproduce the comparison

Build the CLI, then run with the pinned SDK installed. Choose new output directories. Set `TMPDIR` to a filesystem with enough space for an isolated SDK copy if needed.

```sh
cargo build -p fatoora-cli --locked
python scripts/qr_parity.py --sdk-root "$FATOORA_HOME" --output /tmp/qr-comparison
python scripts/sdk_parity.py verify --sdk-root "$FATOORA_HOME" --output /tmp/sdk-verification
```

`qr_parity.py` writes the input XML, SDK and Fatoority signed XML, raw process evidence and `report.json`. The report contains source/input hashes, encoded QR payloads, decoded tag sequences, exact Tags 4/5, Tag 9 in Base64, SDK validation outcomes and Fatoority stage results. It verifies that the installed SDK remains unchanged. A reviewed copy of the report is stored in `tests/fixtures/qr-correctness/report.json` under `fatoora-core`.

The broader SDK verification checks freshly exported model and XML signing output against existing golden validation outcomes. It includes standard documents for which the SDK skips QR verification; those skips remain explicit.

## Regression coverage and API changes

Tests cover both signing paths and all six invoice/note combinations, unchanged imported XML, equality of embedded and exposed QR, certificate metadata retention, binary Tag 9 content, lexical decimal preservation, exact decimal equivalence, document-currency VAT selection, malformed Base64, oversized fields, subtype errors, TLV framing/order/duplicates and tampering. C/C++ and Python checks exercise the shared native boundary.

Public method signatures are unchanged. `QrCodeError::InvalidField` adds an invalid-input error to a non-exhaustive enum, with structured error details for bindings. Signed parsing is stricter; callers that used it to inspect corrupted documents should retain their source XML and use finalized parsing for supported invoice fields. The CLI rejects a failed signed import instead of generating an unsigned replacement QR. See [migration guidance](sdk-parity.md#qr-correctness-change-and-migration).

## Verification performed

Formatting and workspace Clippy passed with warnings denied. The workspace unit, integration and documentation suites passed with live ZATCA API tests disabled. The pinned SDK golden verification passed, and the 16-case SDK QR comparison retained matching capture-source hashes. Release metadata, frozen business-rule/integrity evidence and 73 release-tool unit tests passed. Native C/C++ contracts and C examples passed. All 71 Python binding tests passed against the current native library in an isolated copy of the existing Python package.

No new wheel, NuGet, JVM, WASM or platform release package was built. C#, Java and JavaScript runtime suites were not rerun. No registry publication or platform certificate trust check occurred. The development compliance probes below used no production endpoint.

## Development compliance API observations

One disposable compliance credential and 15 explicitly approved public test probes were sent to `https://gw-fatoora.zatca.gov.sa/e-invoicing/developer-portal/compliance/invoices`. Fresh issue dates and UUIDs were used; paired QR mutations retained the same signed invoice, certificate and digest. The credential secret and private key were temporary. The recorded responses are in `tests/fixtures/qr-correctness/development-api.json` under `fatoora-core`.

| Probe | Development compliance result |
| --- | --- |
| Six subtype-correct baselines | HTTP 200/202; `PASS` or `WARNING`, with `REPORTED`/`CLEARED` response fields |
| Tag 4 `115.0` or `115` against XML `115.00` | HTTP 400; `invoiceTotal_QRCODE_INVALID` |
| Tag 5 `15.0` against XML `15.00` | HTTP 200; `PASS` |
| Incorrect Tag 4 `116.00` against XML `115.00` | HTTP 400; `invoiceTotal_QRCODE_INVALID` |
| Incorrect Tag 5 `16.00` against XML `15.00` | HTTP 200; `PASS`, despite the mismatch |
| Simplified invoice missing Tag 9 | HTTP 400; `QRCODE_INVALID` |
| Standard invoice containing Tag 9 | HTTP 202; `WARNING`, with `CLEARED` |
| Standard foreign-currency invoice with Tag 5 `285` USD or `1068.75` SAR | Both HTTP 200; `PASS`, with `CLEARED` |

These observations confirm Tag 4's additional formatting constraint. Acceptance of the incorrect VAT and unexpected standard Tag 9 controls prevents treating the development service as an integrity oracle. It does not resolve the standard-invoice VAT currency discrepancy. Fatoority retains exact VAT mismatch rejection and the published BT-110 interpretation. The standard warnings concern existing supply-date, charge-reason and exemption-reason fields; they are unrelated to the QR probes.

The endpoint was the development **compliance check**, not the reporting or clearance submission endpoints. Its `REPORTED`/`CLEARED` fields are recorded response values; it returned no cleared XML. No production acceptance or genuine platform-stamped XML fixture was established. `scripts/qr_development_api.py` reproduces this credential-issuing, external-submission workflow only when explicitly authorized.

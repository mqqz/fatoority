---
title: Migrate a Python Project from fatoority 0.5 to 0.6
description: Upgrade Python code from fatoority 0.5 to 0.6. Getters become properties, invoice data becomes frozen dataclasses, and renamed 0.5 names warn until 0.7.
---

# Migrate from 0.5 to 0.6: Python

The `fatoority` package (imported as `fatoora`) adopts the core names with a
Pythonic surface. Most renamed 0.5 names still work in 0.6 and emit a
`DeprecationWarning` naming their replacement; they are removed in 0.7. A few
changes are hard breaks, listed first below.

```sh
pip install "fatoority==0.6.0"
```

The wheel bundles its native library, so no separate native upgrade is needed.

## Upgrade checklist

1. Drop the parentheses from getters: `invoice.id()` becomes `invoice.id`,
   `response.reporting_status()` becomes `response.reporting_status`. Calling a
   property fails like calling its value
   (`TypeError: 'str' object is not callable`).
2. Pass the arguments that 0.5.1 warned about by keyword: everything after
   `name` in `set_seller`/`set_buyer`, `language` in `add_note`, every
   `CsrProperties` field, and the clearance flag and `accept_language` in
   `report_simplified_invoice` and `clear_standard_invoice`. Positional use now
   raises `TypeError`.
3. Expect `datetime`/`date` values where 0.5 returned text: `issue_datetime`,
   `signing_time` and `ZatcaReport.evaluated_at` are timezone-aware UTC
   `datetime`s; `issue_date`, the delivery dates and
   `OriginalInvoiceRef.issue_date` are `datetime.date`s.
4. Expect tuples where 0.5 returned lists: `party.identifiers` and
   `payment_means.instruction_notes`.
5. Run your tests with `python -W error::DeprecationWarning` (or pytest's
   `filterwarnings = ["error::DeprecationWarning"]`) and replace each name the
   warnings point at.

The [Python reference](../../reference/bindings/python.md#migrate-from-05) has
the complete 0.5 to 0.6 table, one row per renamed or retyped name.

## Build an invoice

`InvoiceBuilder(...)` replaces `InvoiceBuilder.new(...)` and takes the header
fields as keywords. Setters return the builder, so calls chain. Dates and UUIDs
accept `datetime`, `date` and `uuid.UUID` values as well as strings; a naive
`datetime` raises `ValueError`. `Address`, `PaymentMeans`, `Party`,
`Adjustment` and the other invoice values are frozen dataclasses built with
their constructors.

=== "0.5"

    ```python
    from pathlib import Path
    from fatoora import InvoiceBuilder, InvoiceSubType, InvoiceTypeKind, VatCategory

    builder = InvoiceBuilder.new(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)
    builder.set_id("INV-1001")
    builder.set_uuid("8e6000cf-1a98-4174-b3e7-b5d5954bc10d")
    builder.set_issue_datetime("2026-10-26T09:00:00Z")
    builder.set_currency("SAR")
    builder.set_previous_invoice_hash(
        "NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ=="
    )
    builder.set_invoice_counter(1)
    builder.set_payment_means_code("10")
    builder.set_vat_category(VatCategory.STANDARD)
    builder.set_seller(
        name="Example Store", country_code="SA", city="Riyadh",
        street="King Fahd Road", building_number="1234", postal_code="12222",
        district="Olaya", additional_number="5678", vat_id="399999999900003",
        other_id="7003339333", other_id_scheme="CRN",
    )
    builder.add_line_item(
        description="Notebook", quantity="2", unit_code="PCE", unit_price="50",
        vat_rate="15", vat_category=VatCategory.STANDARD,
    )
    invoice = builder.build()
    Path("invoice.xml").write_text(invoice.to_xml(), encoding="utf-8")
    totals = invoice.totals()
    print(f"Payable: {totals.payable_amount} SAR")
    ```

=== "0.6"

    ```python
    from datetime import datetime, timezone
    from pathlib import Path
    from uuid import UUID
    from fatoora import (Address, InvoiceBuilder, InvoiceSubType, InvoiceTypeKind,
                         PaymentMeans, VatCategory)

    seller_address = Address(
        "SA", "Riyadh", "King Fahd Road", "1234", "12222",
        additional_number="5678", district="Olaya",
    )
    invoice = (
        InvoiceBuilder(
            InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED,
            id="INV-1001",
            uuid=UUID("8e6000cf-1a98-4174-b3e7-b5d5954bc10d"),
            issue_datetime=datetime(2026, 10, 26, 9, 0, tzinfo=timezone.utc),
            currency="SAR",
            previous_invoice_hash="NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ==",
            invoice_counter=1,
            vat_category=VatCategory.STANDARD,
        )
        .set_seller("Example Store", address=seller_address, vat_id="399999999900003",
                    other_id="7003339333", other_id_scheme="CRN")
        .add_payment_means(PaymentMeans("10"))
        .add_line_item(description="Notebook", quantity="2", unit_code="PCE", unit_price="50",
                       vat_rate="15", vat_category=VatCategory.STANDARD)
        .build()
    )
    Path("invoice.xml").write_text(invoice.xml, encoding="utf-8")
    totals = invoice.totals
    print(f"Payable: {totals.payable_amount} SAR")
    ```

`set_payment_means_code(code)` still works with a warning, but it now appends a
payment means instead of changing the code of an existing one.

## Document allowances and charges

`add_allowance` and `add_charge` append one entry each, in the VAT category and
at the rate you pass. The 0.5 aggregate setters (`set_allowance`,
`invoice_level_discount`, `invoice_level_charge`, `allowance_reason`) are
deprecated. They still produce one allowance and one charge in the document VAT
category, applied by `build()`, which raises `ValueError` if that category's
lines have more than one VAT rate or if they are combined with
`add_allowance`, `add_charge` or `add_adjustment`.

=== "0.5"

    ```python
    builder.set_vat_category(VatCategory.STANDARD)
    builder.invoice_level_discount("10")
    builder.allowance_reason("Loyalty discount")
    ```

=== "0.6"

    ```python
    from fatoora import Adjustment, VatCategory

    builder.add_allowance(
        Adjustment("10", reason="Loyalty discount"), VatCategory.STANDARD, "15",
    )
    ```

`Adjustment` is the one allowance and charge model, for lines, for the document
and for the values read back from `invoice.adjustments`. `Adjustment.amount(v)`
becomes `Adjustment(v)`, `Adjustment.percentage(base, percent)` becomes
`Adjustment.percent_of(base, percent)`, `.value` becomes `.amount` and
`.percent` becomes `.percentage`. `AllowanceCharge(...)` and
`add_document_adjustment(charge, ...)` are deprecated.

## Request credentials and report an invoice

`ComplianceCsid` and `ProductionCsid` replace `CsidCompliance` and
`CsidProduction`, and are built with their constructors:
`ProductionCsid(env, token=..., secret=..., request_id=None)` replaces
`ProductionCsid.new(env, token, secret, request_id)` (the arguments after `env`
are keyword-only, so their order no longer matters). `request_compliance_csid`
and `request_production_csid` replace `post_csr_for_ccsid` and
`post_ccsid_for_pcsid`. The `Clearance-Status` header is a `ClearanceMode`, as in
the other languages: `clearance=ClearanceMode.DISABLED` replaces
`clearance_status=False`. Everything is importable from the `fatoora` package
root.

=== "0.5"

    ```python
    from pathlib import Path
    import os
    from fatoora.api import CsidProduction, ZatcaClient
    from fatoora.config import Config, Environment
    from fatoora.invoice import parse_signed_invoice_xml

    environment = Environment.NON_PRODUCTION
    token = os.environ["ZATCA_TOKEN"]
    secret = os.environ["ZATCA_SECRET"]
    with Config(environment) as config, ZatcaClient(config) as client:
        with CsidProduction.new(environment, token, secret, None) as credentials:
            with parse_signed_invoice_xml(Path("signed.xml").read_text(encoding="utf-8")) as invoice:
                with client.report_simplified_invoice(invoice, credentials, clearance_status=False, accept_language="en") as response:
                    response.ensure_accepted()
                    print("Invoice reporting accepted:", response.reporting_status())
    ```

=== "0.6"

    ```python
    import os
    from fatoora import (ClearanceMode, Config, Environment, ProductionCsid, SignedInvoice,
                         ZatcaClient)

    environment = Environment.NON_PRODUCTION
    token = os.environ["ZATCA_TOKEN"]
    secret = os.environ["ZATCA_SECRET"]
    with Config(environment) as config, ZatcaClient(config, timeout=30) as client:
        with ProductionCsid(environment, token=token, secret=secret) as credentials:
            with SignedInvoice.from_file("signed.xml") as invoice:
                with client.report_simplified_invoice(
                    invoice, credentials, clearance=ClearanceMode.DISABLED, accept_language="en"
                ) as response:
                    response.ensure_accepted()
                    print("Invoice reporting accepted:", response.reporting_status)
    ```

`timeout` is new and optional: seconds (`int`/`float`) or a
`datetime.timedelta` bounding each request. A request that exceeds it raises
`NetworkError`; the default `None` applies no timeout. `clearance_status=False`
and `ProductionCsid.new(...)` keep working with a `DeprecationWarning`.
`reporting_status` is a `ReportingStatus` (and `clearance_status` a
`ClearanceStatus`): `str` enums, so comparing with `"REPORTED"` and printing
still work. `ComplianceCsid.request_id` and `ProductionCsid.request_id` are `None` instead of
`""` when absent.

## Validate and hash an invoice

Validation no longer takes a `Config`. `validate_zatca` returns a frozen
`ZatcaReport` instead of a dictionary; `report.to_dict()` returns the 0.5
dictionary shape and `report.to_json()` the JSON text.

=== "0.5"

    ```python
    from pathlib import Path
    from fatoora import invoice_hash_base64_from_xml_str
    from fatoora.config import Config, Environment
    from fatoora.validation import validate_xml_invoice_from_str, validate_zatca_invoice_from_str

    xml = Path("invoice.xml").read_text(encoding="utf-8")
    with Config(Environment.NON_PRODUCTION) as config:
        validate_xml_invoice_from_str(config, xml)
        report = validate_zatca_invoice_from_str(
            config, xml, previous_invoice_hash=Path("previous-hash.txt").read_text().strip(),
        )
        if not report["is_valid"]:
            raise SystemExit(1)
    next_previous_hash = invoice_hash_base64_from_xml_str(xml)
    ```

=== "0.6"

    ```python
    from pathlib import Path
    from fatoora import hash_xml, validate_xml, validate_zatca

    xml = Path("invoice.xml").read_text(encoding="utf-8")
    validate_xml(xml)
    report = validate_zatca(
        xml, previous_invoice_hash=Path("previous-hash.txt").read_text().strip(),
    )
    for finding in report.findings:
        print(finding.severity.value, finding.code, finding.message)
    if not report.is_valid:
        raise SystemExit(1)
    next_previous_hash = hash_xml(xml)
    ```

On a parsed or built invoice, `hash()` replaces `hash_base64()`;
`signed.invoice_hash` is the hash stored in the signature. `SignedInvoice.from_xml`
and `from_file` (and the same on `FinalizedInvoice`) replace the
`parse_*_invoice_xml[_file]` functions, and the `xml` property replaces
`to_xml()`/`into_xml()`.

## Read a ZATCA response

=== "0.5"

    ```python
    with response.validation_results() as results:
        summary = {
            "http_status": response.http_status(),
            "outcome": response.outcome().name,
            "reporting_status": response.reporting_status(),
            "validation_status": results.status(),
            "warnings": [(m.code(), m.message()) for m in results.warning_messages()],
        }
    ```

=== "0.6"

    ```python
    with response.validation_results as results:
        summary = {
            "http_status": response.http_status,
            "outcome": response.outcome.name,
            "reporting_status": response.reporting_status,
            "validation_status": results.status,
            "warnings": [(m.code, m.message) for m in results.warning_messages],
        }
    ```

Validation statuses and message types are `str` enums too
(`ValidationStatus.PASS`, `MessageType.WARNING`). A status this version does not
recognize keeps the exact text ZATCA sent, in a value whose `name` is `"OTHER"`.

## Other renames

These keep working with a `DeprecationWarning` until 0.7:

| 0.5 | 0.6 |
| --- | --- |
| `Signer` | `InvoiceSigner` |
| `CsrProperties.new(*, ...)` | `CsrProperties(*, ...)` |
| `ComplianceCsid.new(env, token, secret, request_id)`, `ProductionCsid.new(...)` | `ComplianceCsid(env, token=..., secret=..., request_id=...)`, `ProductionCsid(...)` |
| `clearance_status=False` / `True` | `clearance=ClearanceMode.DISABLED` / `ClearanceMode.ENABLED` |
| `CsrProperties.from_properties_str(text)`, `parse_csr_config(text)` | `CsrProperties.from_str(text)` |
| `CsrProperties.parse_csr_config_file(path)` | `CsrProperties.from_file(path)` |
| `InvoiceBuilder.new_with_reference(type, subtype, ref, reason)` | `InvoiceBuilder(type, subtype, original_invoice=ref, original_invoice_reason=reason)` |
| `LineItemBuilder.new()` | `LineItemBuilder()` |
| `builder.flags(value)` | `builder.set_flags(value)` |
| `set_tax_exemption(...)` | `add_tax_exemption(...)` |
| `set_note(language, text)`, `set_note_without_language(text)` | `add_note(text, language=...)` |
| `Address.new(...)`, `PaymentMeans.new(...)`, `OriginalInvoiceRef.new(...)` | `Address(...)`, `PaymentMeans(...)`, `OriginalInvoiceRef(...)` |
| `DocumentReference(...)` | `OriginalInvoiceRef(...)` |
| `VatId`, `OtherId`, `(value, scheme)` tuples | `Identifier(value, scheme)` |

Calling `close()` or using `with` on the invoice value dataclasses (`Address`,
`Party`, `Adjustment`, ...) is a deprecated no-op. Native objects such as
`Config`, `ZatcaClient`, `InvoiceBuilder` and invoices remain context managers.

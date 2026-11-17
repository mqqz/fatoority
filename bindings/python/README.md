## Fatoority Python bindings

This package wraps Diplomat-generated native bindings with the public `fatoora`
Python API. Install with `pip install fatoority` and import as `fatoora`. See [Diplomat bindings](../../docs/development/diplomat.md).

### Development (uv)

```bash
uv venv
uv pip install .
```

### Build a wheel (uv)

```bash
uv build --wheel
```

### Wheels (CI)

- GitHub Actions uses `cibuildwheel` to produce platform wheels in `dist/` (wheel-only publishing).
- See `.github/workflows/python.yml` for the build matrix.

### Notes

- The build step compiles `fatoora-ffi` with `cargo build -p fatoora-ffi --release`.
- The shared library is bundled into the Python package.
- Building the native extension requires CMake and a C++20 compiler. Nanobind is
  pinned in the build dependencies. Reinstall the package after native changes.
- ZATCA API responses are exposed via owned objects with read-only properties (no JSON payloads).
- Errors are raised as typed exceptions mapped from FFI error codes (see `FfiErrorKind`). Each exception exposes `.code`, `.kind`, and `.details`, a dictionary containing structured validation issues or diagnostics. Unknown codes remain available on a generic `FfiError`.
- Reinstall the package after changes; the extension loads its bundled shared library.

### Examples

```bash
python examples/load_lib.py
python examples/invoice_basic.py
python examples/invoice_parse.py
```

### Tests

```bash
uv pip install -e .[dev]
SKIP_ZATCA_LIVE_API=1 uv run pytest tests
```

### High-level API

```python
from datetime import datetime, timezone
from uuid import uuid4

from fatoora import (Address, InvoiceBuilder, InvoiceSubType, InvoiceTypeKind,
                     PaymentMeans, VatCategory)

invoice = (
    InvoiceBuilder(
        InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED,
        id="INV-1",
        uuid=uuid4(),
        issue_datetime=datetime.now(timezone.utc),
        currency="SAR",
        previous_invoice_hash="hash",
        invoice_counter=1,
        vat_category=VatCategory.STANDARD,
    )
    .set_seller("Acme Inc", vat_id="399999999900003",
                address=Address.new("SAU", "Riyadh", "King Fahd", "1234", "12222"))
    .add_payment_means(PaymentMeans.new("10"))
    .add_line_item(description="Item", quantity="1.0", unit_code="PCE",
                   unit_price="100.0", vat_rate="15.0", vat_category=VatCategory.STANDARD)
    .build()
)
print(invoice.id, invoice.issue_datetime, invoice.totals.payable_amount)
print(invoice.xml)
```

Header fields can be passed to the constructor or set with the chaining setters.
Timestamps accept a timezone-aware `datetime` or `YYYY-MM-DDTHH:MM:SSZ` text; UUIDs
accept `uuid.UUID` or text. Invoice data is read through properties
(`invoice.seller.address.city`).

### Local ZATCA validation

```python
from fatoora import validate_zatca

report = validate_zatca(xml, previous_invoice_hash=previous_hash_from_history)
if not report.is_valid:
    for finding in report.findings:
        print(finding.severity.value, finding.code, finding.message)
```

`validate_zatca` returns a frozen `ZatcaReport` dataclass (`report.to_json()` gives
the JSON report). The report retains warnings and distinguishes rejection from
incomplete coverage. Missing predecessor context leaves the report incomplete.
Execution failures raise a typed exception with any partial report in
`error.details["report"]`. Standard invoices skip signature/QR checks in this SDK
profile. Local integrity does not establish issuer trust or remote acceptance. The
wheel embeds runtime schemas and native rules; Java, SDK files and a source
checkout are unnecessary.

### Deprecations

0.6 turns getters into properties (a hard break: drop the parentheses) and
renames parts of the 0.5 API (`Signer` -> `InvoiceSigner`, ...). The renamed
names keep working with a `DeprecationWarning` until 0.7; see the
[Python reference](../../docs/reference/bindings/python.md#migrate-from-05). The
public surface is recorded in `tests/public_api.json`; regenerate it after an
intentional change with `python tests/test_public_api.py --update`.

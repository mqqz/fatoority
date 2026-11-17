"""Exercise the installed wheel without a source checkout or test fixtures."""

from pathlib import Path

import fatoora
from fatoora import Address, Config, Environment


package = Path(fatoora.__file__).parent
assert not (package / "_lib.py").exists(), "wheel must not retain the legacy loader"
assert not (package / "fatoora_ffi.h").exists(), "wheel must not retain legacy declarations"
from fatoora import _native
assert _native.__file__

bundled_libraries = package.parent / "fatoority.libs"
for directory in (package, bundled_libraries):
    assert not any("xml2" in path.name.lower() or "libxml" in path.name.lower() for path in directory.rglob("*")), "wheel must not bundle libxml2"

with Config(Environment.NON_PRODUCTION) as config:
    assert config.env == Environment.NON_PRODUCTION

with Address.new(
    "SA", "Riyadh", "King Fahd", "1234", "12222", district="Olaya"
) as address:
    assert address.district == "Olaya"

from fatoora import InvoiceBuilder, InvoiceTypeKind, InvoiceSubType, PaymentMeans, VatCategory
from fatoora.errors import InvalidInputError

maps = Path("/proc/self/maps")
if maps.exists():
    loaded = [line.split()[-1] for line in maps.read_text().splitlines() if "libfatoora_ffi" in line]
    assert loaded and all(Path(path).parent in (package, bundled_libraries) for path in loaded), loaded

builder = InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)
builder.set_id("INV-1")
builder.set_uuid("8e6000cf-1a98-4174-b3e7-b5d5954bc10d")
builder.set_issue_datetime("2024-01-01T12:30:00Z")
builder.set_previous_invoice_hash("hash")
builder.set_invoice_counter(1)
builder.set_currency("SAR")
builder.add_payment_means(PaymentMeans.new("10"))
builder.set_vat_category(VatCategory.STANDARD)
builder.set_seller(name="Seller", vat_id="399999999900003", country_code="SA", city="Riyadh",
                   street="King Fahd", building_number="1234", postal_code="12222")
builder.add_line_item("Item", 3, "PCE", "0.3333", 15, VatCategory.STANDARD)
xml = builder.build().xml
assert ">1.15</cbc:TaxInclusiveAmount>" in xml
try:
    builder.build()
except InvalidInputError as error:
    assert error.code == 1 and error.details["type"] == "binding_error"
else:
    raise AssertionError("consumed builder must fail")
print("Installed wheel exercised generated bindings and bundled library")

from fatoora import SigningKey
with SigningKey.generate() as key:
    der = key.to_der()
    assert isinstance(der, bytes) and der
with SigningKey.from_der(der) as restored:
    assert restored.to_der() == der

from fatoora import Severity, ZatcaStage, ZatcaStageStatus, validate_xml, validate_zatca

# Build an unsigned standard invoice to exercise embedded schema resources and
# both business-rule profiles without reading source-tree fixtures.
seed = "NWZlY2ViNjZmZmM4NmYzOGQ5NTI3ODZjNmQ2OTZjNzljMmRiYzIzOWRkNGU5MWI0NjcyOWQ3M2EyN2ZiNTdlOQ=="
invoice = (
    InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.STANDARD, id="WHEEL-1",
                   uuid="123e4567-e89b-12d3-a456-426614174000",
                   issue_datetime="2024-01-01T12:30:00Z", currency="SAR",
                   previous_invoice_hash=seed, invoice_counter=1,
                   vat_category=VatCategory.STANDARD)
    .add_payment_means(PaymentMeans.new("10"))
    .set_seller("Acme Inc", vat_id="399999999900003", country_code="SAU", city="Riyadh",
                street="King Fahd", building_number="1234", postal_code="12222", district="Olaya")
    .add_line_item(description="Item", quantity="1.0", unit_code="PCE", unit_price="100.0",
                   vat_rate="15.0", vat_category=VatCategory.STANDARD)
    .build()
)
assert validate_xml(invoice.xml)
report = validate_zatca(invoice.xml, previous_invoice_hash=seed,
                        evaluated_at="2026-09-23T12:00:00+03:00")
assert report.is_complete, report
assert len(report.stages[1].evaluated_assertions) == 105
assert len(report.stages[2].evaluated_assertions) == 152
assert report.stage(ZatcaStage.SIGNATURE).status is ZatcaStageStatus.NOT_APPLICABLE
assert all(f.severity is not Severity.ERROR for f in report.findings)
assert not validate_zatca("<wrong/>").is_valid
for notice in ("LICENSE-LGPL-3.0.txt", "LICENSE-GPL-3.0.txt", "NOTICE.md"):
    assert (package / "licenses/zatca" / notice).is_file(), notice
print("Installed wheel exercised embedded schemas and all 257 business-rule assertions")

# The 0.5 getter call form keeps working (with a DeprecationWarning) on every
# interpreter a wheel is built for; see fatoora/_compat.py.
import warnings
with warnings.catch_warnings(record=True) as caught:
    warnings.simplefilter("always")
    assert invoice.id() == "WHEEL-1"
    assert invoice.buyer() is None
assert invoice.id == "WHEEL-1" and invoice.buyer is None
assert [w.category for w in caught] == [DeprecationWarning, DeprecationWarning], caught
print("Deprecated getter calls still resolve")

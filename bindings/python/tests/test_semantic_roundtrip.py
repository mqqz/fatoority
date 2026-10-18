from decimal import Decimal
from pathlib import Path

from fatoora import (AllowanceCharge, TaxExemption, VatCategory,
                     parse_finalized_invoice_xml, parse_signed_invoice_xml,
                     invoice_hash_base64_from_xml_str)
from test_numeric_contract import builder


def test_new_metadata_crosses_native_binding_and_xml():
    b = builder()
    b.set_delivery("2024-01-01", "2024-01-02")
    b.set_buyer("المشتري", "SA", "Riyadh", "Street", "", "", vat_id=None)
    reason = TaxExemption("VATEX-SA-29", "إعفاء & <test>")
    b.add_line_item("سلعة", 2, "PCE", "10.1234", 0, VatCategory.EXEMPT,
                    id="original-line", exemption=reason)
    b.set_tax_exemption(VatCategory.EXEMPT, 0, reason)
    b.add_adjustment(AllowanceCharge(False, Decimal("1"), VatCategory.EXEMPT,
                                    Decimal("0"), "خصم", "95", reason))
    b.set_payment_instruction("تحويل")
    b.set_prepaid_amount("1")
    b.set_payable_rounding_amount("-0.01")
    invoice = b.build()
    parsed = parse_finalized_invoice_xml(invoice.xml())
    assert parsed.actual_delivery_date() == "2024-01-01"
    assert parsed.latest_delivery_date() == "2024-01-02"
    assert parsed.buyer().name() == "المشتري"
    assert parsed.buyer().vat_id() is None
    assert parsed.line_items() == invoice.line_items()
    assert parsed.line_item(0).exemption == reason
    assert parsed.adjustments() == invoice.adjustments()
    assert parsed.vat_breakdown() == invoice.vat_breakdown()
    assert parsed.payment_instruction() == "تحويل"
    assert parsed.totals() == invoice.totals()


def test_signed_import_retains_source_and_raw_hash():
    path = Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures/invoices/Standard/Invoice/Exempt Tax Invoice.xml"
    xml = path.read_text()
    signed = parse_signed_invoice_xml(xml)
    assert signed.xml() == xml
    assert invoice_hash_base64_from_xml_str(signed.xml()) == invoice_hash_base64_from_xml_str(xml)
    assert signed.buyer() is not None
    assert any(group.exemption.reason_code for group in signed.vat_breakdown())


def test_prepayment_reference_and_tax_breakdown_are_exposed():
    path = Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures/invoices/Standard/Invoice/Advance Payment adjustments.xml"
    invoice = parse_finalized_invoice_xml(path.read_text())
    item = invoice.line_item(1)
    assert item.document_references[0].id == "46531"
    assert item.document_references[0].issue_time == "12:28:17"
    assert item.document_references[0].document_type_code == "386"
    assert item.tax_subtotals[0].taxable_amount == Decimal("1000")
    assert item.tax_subtotals[0].tax_amount == Decimal("150")
    assert parse_finalized_invoice_xml(invoice.xml()).line_items() == invoice.line_items()


def test_repeated_adjustment_classifications_can_be_rebuilt():
    from dataclasses import replace
    import xml.etree.ElementTree as ET
    import pytest
    from fatoora.errors import FfiError

    exemption = TaxExemption("VATEX-SA-29", "إعفاء & <test>")
    adjustment = AllowanceCharge(True, Decimal("1"), VatCategory.EXEMPT,
                                 Decimal("0"), "رسوم &", "95", exemption)
    source = builder()
    source.add_line_item("item", 1, "PCE", 100, 0, VatCategory.EXEMPT)
    source.add_adjustment(adjustment)
    invoice = source.build()
    xml = invoice.xml()
    root = ET.fromstring(xml)
    ns = {"cac": "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"}
    category = root.find("cac:AllowanceCharge/cac:TaxCategory", ns)
    # Duplicate the original XML category without rewriting namespace prefixes.
    start = xml.index("<cac:TaxCategory>", xml.index("<cac:AllowanceCharge>"))
    end = xml.index("</cac:TaxCategory>", start) + len("</cac:TaxCategory>")
    assert category is not None
    for count in [2, 3, 1]:
        parsed = parse_finalized_invoice_xml(xml[:start] + xml[start:end] * count + xml[end:])
        restored = parsed.adjustments()[0]
        parsed.close()
        assert restored == replace(adjustment, tax_category_count=count)
        b = builder()
        b.add_line_item("item", 1, "PCE", 100, 0, VatCategory.EXEMPT)
        for invalid_count in [-1, 1 << 128]:
            with pytest.raises((TypeError, OverflowError)):
                b.add_adjustment(replace(restored, tax_category_count=invalid_count))
        b.add_adjustment(restored)
        with pytest.raises(FfiError):
            b.add_adjustment(replace(restored, tax_category_count=0))
        rebuilt = b.build()
        assert rebuilt.adjustments() == [restored]
        assert rebuilt.totals() == invoice.totals()
        output = ET.fromstring(rebuilt.xml())
        assert len(output.findall("cac:AllowanceCharge/cac:TaxCategory", ns)) == count
        assert parse_finalized_invoice_xml(rebuilt.xml()).adjustments() == [restored]

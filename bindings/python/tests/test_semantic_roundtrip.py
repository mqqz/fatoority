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
    assert signed.hash_base64() == invoice_hash_base64_from_xml_str(xml)
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
        for invalid_count in [0, 1025]:
            with pytest.raises(FfiError):
                b.add_adjustment(replace(restored, tax_category_count=invalid_count))
        rebuilt = b.build()
        assert rebuilt.adjustments() == [restored]
        assert rebuilt.totals() == invoice.totals()
        output = ET.fromstring(rebuilt.xml())
        assert len(output.findall("cac:AllowanceCharge/cac:TaxCategory", ns)) == count
        assert parse_finalized_invoice_xml(rebuilt.xml()).adjustments() == [restored]


def test_optional_line_builder_and_repeated_metadata():
    from fatoora import Adjustment, LineItemBuilder, PaymentMeans, OriginalInvoiceRef
    line = LineItemBuilder.new()
    line.set_item_name("case of items")
    line.set_invoiced_quantity("30")
    line.set_invoiced_quantity_unit_code("PCE")
    line.set_item_net_price("100")
    line.set_item_gross_price("120")
    line.set_item_price_discount("20")
    line.set_item_price_base_quantity("12")
    line.set_item_price_base_quantity_unit_code("PCE")
    line.set_vat_category(VatCategory.STANDARD)
    line.set_vat_rate("15")
    line.set_item_seller_identifier("SKU", None)
    line.add_allowance(Adjustment.percentage("250", "10").with_reason("discount", "95"))
    line.add_charge(Adjustment.amount("5").with_reason("freight"))
    item = line.build()
    assert item.total_amount == Decimal("230")
    b = builder()
    b.add_item(item)
    b.add_note("one", "en")
    b.add_note("اثنان")
    b.add_payment_means(PaymentMeans.new("30", "first").with_instruction_note("second").with_account("account", "net 30"))
    b.add_seller_identifier("secondary", "OTH")
    b.add_billing_reference(OriginalInvoiceRef.new("original", None, None, None, None))
    b.set_purchase_order_id("PO")
    b.set_contract_id("CON")
    b.set_buyer_without_address("buyer")
    b.add_buyer_identifier("buyer-id", "NAT")
    b.add_document_adjustment(False, Adjustment.percentage("230", "10"), VatCategory.STANDARD, "15")
    invoice = b.build()
    restored = parse_finalized_invoice_xml(invoice.xml())
    assert restored.line_items() == invoice.line_items()
    assert restored.adjustments() == invoice.adjustments()
    assert restored.line_item(0).item_price_base_quantity == Decimal("12")
    assert [note.text() for note in restored.notes()] == ["one", "اثنان"]
    assert restored.payment_means()[-1].payment_terms() == "net 30"
    assert restored.billing_references()[0].id() == "original"
    assert restored.purchase_order_id() == "PO"
    assert restored.contract_id() == "CON"
    assert restored.buyer().address() is None
    assert restored.buyer().identifiers()[0].value() == "buyer-id"
    assert restored.seller().identifiers()[-1].value() == "secondary"
    assert restored.payment_means()[-1].instruction_notes() == ["first", "second"]
    assert restored.issue_time() == "12:30:00Z"


def test_percentage_adjustment_keeps_repeated_tax_categories():
    adjustment = AllowanceCharge(False, Decimal("10"), VatCategory.STANDARD,
                                 Decimal("15"), tax_category_count=3,
                                 base_amount=Decimal("100"), percentage=Decimal("10"))
    b = builder()
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    b.add_adjustment(adjustment)
    invoice = b.build()
    assert invoice.adjustments() == [adjustment]
    assert parse_finalized_invoice_xml(invoice.xml()).adjustments() == [adjustment]


def test_python_party_setters_expose_province():
    b = builder()
    b.set_seller("Seller", "SA", "Riyadh", "Street", "1234", "12222",
                 "399999999900003", province="Riyadh Province")
    b.set_buyer("Buyer", "SA", "Jeddah", "Street", "", "", province="Makkah Province")
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    restored = parse_finalized_invoice_xml(b.build().xml())
    assert restored.seller().address().province() == "Riyadh Province"
    assert restored.buyer().address().province() == "Makkah Province"


def test_out_of_scope_adjustment_without_rate_survives_rebuild():
    adjustment = AllowanceCharge(True, Decimal("10"), VatCategory.OUT_OF_SCOPE,
                                 Decimal("0"), omitted_rate=True)
    b = builder()
    b.add_line_item("item", 1, "PCE", 100, 0, VatCategory.OUT_OF_SCOPE)
    b.add_adjustment(adjustment)
    invoice = b.build()
    restored = parse_finalized_invoice_xml(invoice.xml())
    assert restored.adjustments() == [adjustment]
    assert restored.xml() == invoice.xml()


def test_line_tax_presence_survives_python_value_rebuild():
    import itertools
    import xml.etree.ElementTree as ET

    ns = {"cac": "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2",
          "cbc": "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"}
    source = builder()
    source.add_line_item("item", 1, "PCE", 100, 0, VatCategory.OUT_OF_SCOPE)
    xml = source.build().xml()
    for rate_present, tax_present, gross_present in itertools.product([False, True], repeat=3):
        if not tax_present and gross_present:
            continue
        root = ET.fromstring(xml)
        line = root.find("cac:InvoiceLine", ns)
        category = line.find("cac:Item/cac:ClassifiedTaxCategory", ns)
        tax = line.find("cac:TaxTotal", ns)
        if not rate_present:
            category.remove(category.find("cbc:Percent", ns))
        if not tax_present:
            line.remove(tax)
        elif not gross_present:
            tax.remove(tax.find("cbc:RoundingAmount", ns))
        parsed = parse_finalized_invoice_xml(ET.tostring(root, encoding="unicode"))
        item = parsed.line_item(0)
        assert item.omitted_vat_rate == (not rate_present)
        assert item.omitted_tax_amount == (not tax_present)
        assert item.omitted_gross_amount == (not gross_present)
        rebuilt = builder()
        rebuilt.add_item(item)
        result = rebuilt.build()
        assert result.line_item(0) == item
        regenerated = ET.fromstring(result.xml()).find("cac:InvoiceLine", ns)
        def values(node):
            return (node.tag, tuple(sorted(node.attrib.items())), (node.text or "").strip(), tuple(values(child) for child in node))
        assert values(regenerated) == values(line)


def test_breakdown_rate_absence_survives_all_python_entrypoints():
    from fatoora import VatBreakdown
    from dataclasses import replace
    import pytest

    group = VatBreakdown(VatCategory.OUT_OF_SCOPE, Decimal(0), Decimal(100), Decimal(0),
                         TaxExemption(), omitted_rate=True)
    for use_value in [False, True]:
        b = builder()
        b.add_line_item("item", 1, "PCE", 100, 0, VatCategory.OUT_OF_SCOPE,
                        tax_subtotals=(group,))
        original = b.build()
        rebuilt = builder()
        if use_value:
            rebuilt.add_item(original.line_item(0))
        else:
            rebuilt.add_line_item("item", 1, "PCE", 100, 0, VatCategory.OUT_OF_SCOPE,
                                  tax_subtotals=original.line_item(0).tax_subtotals)
        rebuilt.add_vat_breakdown(group)
        output = rebuilt.build().xml()
        import xml.etree.ElementTree as ET
        ns = {"cac": "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2",
              "cbc": "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"}
        categories = ET.fromstring(output).findall(".//cac:TaxSubtotal/cac:TaxCategory", ns)
        assert len(categories) == 2
        assert all(category.find("cbc:Percent", ns) is None for category in categories)
        parsed = parse_finalized_invoice_xml(output)
        assert parsed.vat_breakdown() == [group]
        assert parsed.line_item(0).tax_subtotals == (group,)
    from fatoora.errors import FfiError
    with pytest.raises(FfiError):
        builder().add_vat_breakdown(replace(group, category=VatCategory.STANDARD))


def test_presence_flags_reject_hidden_values_and_keep_explicit_zeroes():
    from dataclasses import replace
    import pytest
    from fatoora.errors import FfiError

    b = builder()
    b.add_line_item("item", 1, "PCE", 100, 15, VatCategory.STANDARD)
    item = b.build().line_item(0)
    with pytest.raises(ValueError, match="omitted VAT rate"):
        builder().add_item(replace(item, omitted_vat_rate=True))
    with pytest.raises(FfiError):
        builder().add_item(replace(item, omitted_tax_amount=True))
    with pytest.raises(FfiError):
        builder().add_item(replace(item, vat_rate=Decimal(0), omitted_vat_rate=True))

from pathlib import Path

import pytest

from fatoora import SignedInvoice
from fatoora.errors import FatooraError


def test_signed_xml_copy_survives_close():
    path = Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures/invoices/sample-simplified-invoice.xml"
    xml = path.read_text()
    signed = SignedInvoice.from_xml(xml)
    copied = signed.xml
    assert copied == xml
    signed.close()
    assert copied == xml
    with pytest.raises(FatooraError):
        signed.xml


def test_generated_signed_invoice_consumption_keeps_copy_and_owned_data():
    from fatoora import _native

    path = Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures/invoices/sample-simplified-invoice.xml"
    xml = path.read_text()
    signed = _native.SignedInvoice.from_xml(xml)
    copied = signed.xml()
    data = signed.data()
    invoice_id = data.id()
    assert signed.into_xml() == xml
    assert copied == xml
    assert data.id() == invoice_id
    for method in (signed.xml, signed.into_xml, signed.data):
        with pytest.raises(Exception) as caught:
            method()
        error = caught.value.args[0]
        assert isinstance(error, _native.BindingError)
        assert error.code() == 1
    del signed
    assert data.id() == invoice_id


def test_closed_facade_is_idempotent_and_rejects_use():
    path = Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures/invoices/sample-simplified-invoice.xml"
    signed = SignedInvoice.from_xml(path.read_text())
    signed.close()
    signed.close()
    with pytest.raises(FatooraError) as caught:
        signed.xml
    assert caught.value.code == 1


def test_signed_import_rejects_tampered_qr():
    import base64
    import xml.etree.ElementTree as ET

    path = Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures/invoices/sample-simplified-invoice.xml"
    xml = path.read_text()
    signed = SignedInvoice.from_xml(xml)
    qr = signed.qr_code
    ns = {"a": "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2",
          "c": "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"}
    embedded = next(n.findtext("a:Attachment/c:EmbeddedDocumentBinaryObject", namespaces=ns)
                    for n in ET.fromstring(xml).findall("a:AdditionalDocumentReference", ns)
                    if n.findtext("c:ID", namespaces=ns) == "QR")
    assert qr == embedded
    raw = bytearray(base64.b64decode(qr))
    raw[2] ^= 1
    with pytest.raises(FatooraError):
        SignedInvoice.from_xml(xml.replace(qr, base64.b64encode(raw).decode()))

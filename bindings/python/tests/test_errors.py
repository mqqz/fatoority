import pytest

from fatoora import InvoiceBuilder, InvoiceSubType, InvoiceTypeKind
from fatoora.errors import FfiError, FfiErrorKind, ValidationError, error_class_for_code


def test_validation_issues_cross_the_ffi_boundary():
    builder = InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)
    with pytest.raises(ValidationError) as caught:
        builder.build()
    error = caught.value
    assert error.code == 2
    assert error.kind == FfiErrorKind.VALIDATION
    assert error.details["type"] == "invoice_validation"
    assert any(issue["field"] == "seller" for issue in error.details["issues"])
    assert all("kind" in issue for issue in error.details["issues"])


def test_unknown_codes_and_details_remain_available():
    details = {"type": "future_error", "new_field": ["some value"]}
    error = error_class_for_code(999)("future message", 999, details)
    assert type(error) is FfiError
    assert error.code == 999
    assert error.kind is None
    assert error.details == details
    assert str(error) == "future message"




def test_invalid_key_uses_shared_input_classification():
    from fatoora.api import SigningKey
    from fatoora.errors import InvalidInputError

    with pytest.raises(InvalidInputError) as caught:
        SigningKey.from_pem("invalid key")
    assert caught.value.details["type"] == "key_decode"


def test_decimal_error_keeps_its_details():
    from fatoora import VatCategory
    from fatoora.errors import InvalidInputError

    builder = InvoiceBuilder(InvoiceTypeKind.TAX, InvoiceSubType.SIMPLIFIED)
    with pytest.raises(InvalidInputError) as caught:
        builder.add_line_item("item", "not-a-number", "PCE", "1", "15", VatCategory.STANDARD)
    assert caught.value.details["type"] == "invalid_decimal"


def test_imported_invoice_validation_keeps_structured_issues():
    from pathlib import Path
    import re
    from fatoora import FinalizedInvoice

    fixture = Path(__file__).resolve().parents[3] / "fatoora-core/tests/fixtures/invoices/sample-simplified-invoice.xml"
    xml = re.sub(r"(<cbc:Percent>)[^<]*(</cbc:Percent>)", r"\g<1>-1\g<2>", fixture.read_text())
    with pytest.raises(ValidationError) as caught:
        FinalizedInvoice.from_xml(xml)
    assert caught.value.details["type"] == "invoice_validation"
    assert caught.value.details["issues"][0]["field"] == "line_item_vat_rate"
    assert caught.value.details["issues"][0]["kind"] == "out_of_range"
    assert caught.value.details["issues"][0]["line_item_index"] == 0



def test_generated_error_is_owned_and_preserves_missing_property():
    import gc
    from fatoora import CsrProperties
    from fatoora.errors import InvalidInputError
    with pytest.raises(InvalidInputError) as caught:
        CsrProperties.from_str("csr.common.name=example")
    details = caught.value.details
    del caught
    gc.collect()
    assert details["type"] == "missing_property"
    assert details["key"]


@pytest.mark.parametrize("cls,builtin", [
    ("InvalidInputError", ValueError),
    ("IoError", OSError),
])
def test_error_classes_also_subclass_builtin_exceptions(cls, builtin):
    import pickle
    from fatoora import errors

    error_type = getattr(errors, cls)
    assert issubclass(error_type, builtin)
    assert issubclass(error_type, FfiError)
    details = {"type": "io", "path": "/tmp/x"}
    code = 6 if cls == "IoError" else 1
    error = error_type("file not found", code, details)
    assert str(error) == "file not found"
    assert error.args == ("file not found",)
    assert error.code == code
    assert error.kind == FfiErrorKind(code)
    assert error.details == details
    assert error_class_for_code(code) is error_type
    # The three-argument constructor must not be reinterpreted as errno/strerror/filename.
    if builtin is OSError:
        assert error.errno is None and error.strerror is None and error.filename is None
    for args in [("only message",), ("message", code)]:
        e = error_type(*args)
        assert str(e) == args[0] and e.args == (args[0],)
    restored = pickle.loads(pickle.dumps(error))
    assert type(restored) is error_type
    assert (str(restored), restored.args, restored.code, restored.details) == (
        "file not found", ("file not found",), code, details)


def test_missing_invoice_file_is_catchable_as_oserror():
    from fatoora import FinalizedInvoice

    with pytest.raises(OSError) as caught:
        FinalizedInvoice.from_file("/nonexistent/fatoora/invoice.xml")
    assert isinstance(caught.value, FfiError)

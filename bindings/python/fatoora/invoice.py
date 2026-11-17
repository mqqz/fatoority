from . import api as _api
from ._compat import alias_getattr as _alias_getattr
from .api import (
    Address,
    Adjustment,
    TaxExemption,
    VatBreakdown,
    FinalizedInvoice,
    Identifier,
    InvoiceBuilder,
    InvoiceFlag,
    InvoiceLineItem,
    InvoiceNote,
    InvoiceSubType,
    InvoiceTotals,
    InvoiceTypeKind,
    OriginalInvoiceRef,
    Party,
    PaymentMeans,
    SignedInvoice,
    VatCategory,
    parse_finalized_invoice_xml,
    parse_finalized_invoice_xml_file,
    parse_signed_invoice_xml,
    parse_signed_invoice_xml_file,
)

__all__ = [
    "Address",
    "Adjustment",
    "TaxExemption",
    "VatBreakdown",
    "FinalizedInvoice",
    "Identifier",
    "InvoiceBuilder",
    "InvoiceFlag",
    "InvoiceLineItem",
    "InvoiceNote",
    "InvoiceSubType",
    "InvoiceTotals",
    "InvoiceTypeKind",
    "OriginalInvoiceRef",
    "Party",
    "PaymentMeans",
    "SignedInvoice",
    "VatCategory",
    "parse_finalized_invoice_xml",
    "parse_finalized_invoice_xml_file",
    "parse_signed_invoice_xml",
    "parse_signed_invoice_xml_file",
]

# 0.5 names merged into the models above; they warn when used.
__getattr__ = _alias_getattr(__name__, {k: _api.DEPRECATED_ALIASES[k] for k in
                                        ["AllowanceCharge", "DocumentReference", "OtherId", "VatId"]})

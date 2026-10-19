#include "diplomat_nanobind_common.hpp"


#include "Address.hpp"
#include "Adjustment.hpp"
#include "AllowanceCharge.hpp"
#include "InvoiceBuilder.hpp"
#include "InvoiceLineItem.hpp"
#include "OriginalInvoiceRef.hpp"
#include "PaymentMeans.hpp"
#include "TaxExemption.hpp"
#include "VatBreakdown.hpp"

namespace _native {
void add_InvoiceBuilder_binding(nb::module_ mod) {
    PyType_Slot _native_InvoiceBuilder_slots[] = {
        {Py_tp_free, (void *)_native::InvoiceBuilder::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::InvoiceBuilder> opaque(mod, "InvoiceBuilder", nb::type_slots(_native_InvoiceBuilder_slots));
    opaque
        .def("add_adjustment", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::add_adjustment)), "adjustment"_a)
        .def("add_billing_reference", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::add_billing_reference)), "value"_a)
        .def("add_buyer_identifier", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::add_buyer_identifier)), "value"_a, "scheme"_a= nb::none())
        .def("add_document_adjustment", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::add_document_adjustment)), "charge"_a, "value"_a, "category"_a, "rate"_a, "exemption"_a)
        .def("add_item", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::add_item)), "item"_a)
        .def("add_line_item", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::add_line_item)), "description"_a, "quantity"_a, "unit_code"_a, "unit_price"_a, "vat_rate"_a, "category"_a)
        .def("add_note", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::add_note)), "language"_a= nb::none(), "value"_a)
        .def("add_payment_means", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::add_payment_means)), "value"_a)
        .def("add_seller_identifier", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::add_seller_identifier)), "value"_a, "scheme"_a= nb::none())
        .def("add_vat_breakdown", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::add_vat_breakdown)), "value"_a)
        .def("allowance_reason", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::allowance_reason)), "value"_a)
        .def("build", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::build)))
        .def("flags", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::flags)), "value"_a)
        .def("invoice_level_charge", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::invoice_level_charge)), "value"_a)
        .def("invoice_level_discount", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::invoice_level_discount)), "value"_a)
        .def_static("new", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::new_)), "kind"_a, "subtype"_a, "original_id"_a= nb::none(), "original_uuid"_a= nb::none(), "original_date"_a= nb::none(), "reason"_a= nb::none())
        .def_static("new_with_reference", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::new_with_reference)), "kind"_a, "subtype"_a, "reference"_a, "reason"_a, "Construct a credit or debit note with a complete billing reference.")
        .def("set_allowance", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_allowance)), "reason"_a, "amount"_a)
        .def("set_buyer", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_buyer)), "name"_a, "address"_a, "vat_id"_a= nb::none(), "other_id"_a= nb::none(), "scheme"_a= nb::none())
        .def("set_buyer_without_address", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_buyer_without_address)), "name"_a, "vat_id"_a= nb::none(), "other_id"_a= nb::none(), "scheme"_a= nb::none())
        .def("set_contract_id", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_contract_id)), "value"_a)
        .def("set_currency", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_currency)), "value"_a)
        .def("set_delivery", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_delivery)), "actual"_a= nb::none(), "latest"_a= nb::none())
        .def("set_id", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_id)), "value"_a)
        .def("set_invoice_counter", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_invoice_counter)), "value"_a)
        .def("set_issue_datetime", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_issue_datetime)), "value"_a)
        .def("set_note", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_note)), "language"_a, "value"_a)
        .def("set_note_without_language", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_note_without_language)), "value"_a)
        .def("set_payable_rounding_amount", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_payable_rounding_amount)), "amount"_a)
        .def("set_payment_instruction", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_payment_instruction)), "instruction"_a)
        .def("set_payment_means_code", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_payment_means_code)), "value"_a)
        .def("set_prepaid_amount", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_prepaid_amount)), "amount"_a)
        .def("set_previous_invoice_hash", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_previous_invoice_hash)), "value"_a)
        .def("set_purchase_order_id", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_purchase_order_id)), "value"_a)
        .def("set_seller", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_seller)), "name"_a, "address"_a, "vat_id"_a, "other_id"_a= nb::none(), "scheme"_a= nb::none())
        .def("set_tax_currency", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_tax_currency)), "currency"_a, "amount"_a)
        .def("set_tax_exemption", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_tax_exemption)), "category"_a, "rate"_a, "exemption"_a)
        .def("set_uuid", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_uuid)), "value"_a)
        .def("set_vat_category", std::move(maybe_op_unwrap(&_native::InvoiceBuilder::set_vat_category)), "value"_a);
}

} 
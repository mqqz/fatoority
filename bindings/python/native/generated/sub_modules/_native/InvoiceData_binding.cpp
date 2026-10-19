#include "diplomat_nanobind_common.hpp"


#include "InvoiceData.hpp"

namespace _native {
void add_InvoiceData_binding(nb::module_ mod) {
    PyType_Slot _native_InvoiceData_slots[] = {
        {Py_tp_free, (void *)_native::InvoiceData::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::InvoiceData> opaque(mod, "InvoiceData", nb::type_slots(_native_InvoiceData_slots));
    opaque
        .def("actual_delivery_date", std::move(maybe_op_unwrap(&_native::InvoiceData::actual_delivery_date)))
        .def("adjustment", std::move(maybe_op_unwrap(&_native::InvoiceData::adjustment)), "index"_a)
        .def("adjustments_len", &_native::InvoiceData::adjustments_len)
        .def("allowance_reason", std::move(maybe_op_unwrap(&_native::InvoiceData::allowance_reason)))
        .def("billing_reference", std::move(maybe_op_unwrap(&_native::InvoiceData::billing_reference)), "index"_a)
        .def("billing_references_len", &_native::InvoiceData::billing_references_len)
        .def("buyer", std::move(maybe_op_unwrap(&_native::InvoiceData::buyer)))
        .def("contract_id", std::move(maybe_op_unwrap(&_native::InvoiceData::contract_id)))
        .def("currency", std::move(maybe_op_unwrap(&_native::InvoiceData::currency)))
        .def("flags_raw", &_native::InvoiceData::flags_raw)
        .def("id", std::move(maybe_op_unwrap(&_native::InvoiceData::id)))
        .def("invoice_counter", &_native::InvoiceData::invoice_counter)
        .def("invoice_level_charge", std::move(maybe_op_unwrap(&_native::InvoiceData::invoice_level_charge)))
        .def("invoice_level_discount", std::move(maybe_op_unwrap(&_native::InvoiceData::invoice_level_discount)))
        .def("invoice_sub_type", &_native::InvoiceData::invoice_sub_type)
        .def("invoice_type_kind", &_native::InvoiceData::invoice_type_kind)
        .def("issue_date", std::move(maybe_op_unwrap(&_native::InvoiceData::issue_date)))
        .def("issue_datetime", std::move(maybe_op_unwrap(&_native::InvoiceData::issue_datetime)))
        .def("issue_time", std::move(maybe_op_unwrap(&_native::InvoiceData::issue_time)))
        .def("latest_delivery_date", std::move(maybe_op_unwrap(&_native::InvoiceData::latest_delivery_date)))
        .def("line_item", std::move(maybe_op_unwrap(&_native::InvoiceData::line_item)), "index"_a)
        .def("line_items_len", &_native::InvoiceData::line_items_len)
        .def("note", std::move(maybe_op_unwrap(&_native::InvoiceData::note)))
        .def("note_at", std::move(maybe_op_unwrap(&_native::InvoiceData::note_at)), "index"_a)
        .def("notes_len", &_native::InvoiceData::notes_len)
        .def("original_invoice_reason", std::move(maybe_op_unwrap(&_native::InvoiceData::original_invoice_reason)))
        .def("original_invoice_ref", std::move(maybe_op_unwrap(&_native::InvoiceData::original_invoice_ref)))
        .def("payment_instruction", std::move(maybe_op_unwrap(&_native::InvoiceData::payment_instruction)))
        .def("payment_means", std::move(maybe_op_unwrap(&_native::InvoiceData::payment_means)), "index"_a)
        .def("payment_means_code", std::move(maybe_op_unwrap(&_native::InvoiceData::payment_means_code)))
        .def("payment_means_len", &_native::InvoiceData::payment_means_len)
        .def("previous_invoice_hash", std::move(maybe_op_unwrap(&_native::InvoiceData::previous_invoice_hash)))
        .def("purchase_order_id", std::move(maybe_op_unwrap(&_native::InvoiceData::purchase_order_id)))
        .def("seller", std::move(maybe_op_unwrap(&_native::InvoiceData::seller)))
        .def("tax_currency", std::move(maybe_op_unwrap(&_native::InvoiceData::tax_currency)))
        .def("tax_currency_amount", std::move(maybe_op_unwrap(&_native::InvoiceData::tax_currency_amount)))
        .def("uuid", std::move(maybe_op_unwrap(&_native::InvoiceData::uuid)))
        .def("vat_category", &_native::InvoiceData::vat_category);
}

} 
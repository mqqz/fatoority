#include "diplomat_nanobind_common.hpp"


#include "InvoiceLineItem.hpp"
#include "OriginalInvoiceRef.hpp"
#include "TaxExemption.hpp"
#include "VatBreakdown.hpp"

namespace _native {
void add_InvoiceLineItem_binding(nb::module_ mod) {
    PyType_Slot _native_InvoiceLineItem_slots[] = {
        {Py_tp_free, (void *)_native::InvoiceLineItem::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::InvoiceLineItem> opaque(mod, "InvoiceLineItem", nb::type_slots(_native_InvoiceLineItem_slots));
    opaque
        .def("description", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::description)))
        .def("document_reference", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::document_reference)), "index"_a)
        .def("document_references_len", &_native::InvoiceLineItem::document_references_len)
        .def("exemption", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::exemption)))
        .def("id", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::id)))
        .def_static("new", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::new_)), "description"_a, "quantity"_a, "unit_code"_a, "unit_price"_a, "rate"_a, "category"_a)
        .def("quantity", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::quantity)))
        .def("tax_subtotal", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::tax_subtotal)), "index"_a)
        .def("tax_subtotals_len", &_native::InvoiceLineItem::tax_subtotals_len)
        .def("total_amount", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::total_amount)))
        .def("unit_code", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::unit_code)))
        .def("unit_price", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::unit_price)))
        .def("vat_amount", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::vat_amount)))
        .def("vat_category", &_native::InvoiceLineItem::vat_category)
        .def("vat_rate", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::vat_rate)))
        .def("with_document_reference", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::with_document_reference)), "reference"_a)
        .def("with_metadata", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::with_metadata)), "id"_a= nb::none(), "exemption"_a)
        .def("with_tax_subtotal", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::with_tax_subtotal)), "subtotal"_a);
}

} 
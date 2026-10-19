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
        .def("adjustment", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::adjustment)), "index"_a)
        .def("adjustment_is_charge", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::adjustment_is_charge)), "index"_a)
        .def("adjustments_len", &_native::InvoiceLineItem::adjustments_len)
        .def("description", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::description)))
        .def("document_reference", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::document_reference)), "index"_a)
        .def("document_references_len", &_native::InvoiceLineItem::document_references_len)
        .def("exemption", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::exemption)))
        .def("has_gross_amount", &_native::InvoiceLineItem::has_gross_amount)
        .def("has_tax_total", &_native::InvoiceLineItem::has_tax_total)
        .def("has_vat_rate", &_native::InvoiceLineItem::has_vat_rate)
        .def("id", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::id)))
        .def("item_buyer_identifier", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::item_buyer_identifier)))
        .def("item_gross_price", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::item_gross_price)))
        .def("item_price_base_quantity", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::item_price_base_quantity)))
        .def("item_price_base_quantity_unit_code", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::item_price_base_quantity_unit_code)))
        .def("item_price_discount", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::item_price_discount)))
        .def("item_seller_identifier", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::item_seller_identifier)))
        .def("item_standard_identifier", std::move(maybe_op_unwrap(&_native::InvoiceLineItem::item_standard_identifier)))
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
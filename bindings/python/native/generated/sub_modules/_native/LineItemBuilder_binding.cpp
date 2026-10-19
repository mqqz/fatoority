#include "diplomat_nanobind_common.hpp"


#include "Adjustment.hpp"
#include "LineItemBuilder.hpp"
#include "OriginalInvoiceRef.hpp"
#include "TaxExemption.hpp"
#include "VatBreakdown.hpp"

namespace _native {
void add_LineItemBuilder_binding(nb::module_ mod) {
    PyType_Slot _native_LineItemBuilder_slots[] = {
        {Py_tp_free, (void *)_native::LineItemBuilder::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::LineItemBuilder> opaque(mod, "LineItemBuilder", nb::type_slots(_native_LineItemBuilder_slots));
    opaque
        .def("add_allowance", std::move(maybe_op_unwrap(&_native::LineItemBuilder::add_allowance)), "value"_a)
        .def("add_charge", std::move(maybe_op_unwrap(&_native::LineItemBuilder::add_charge)), "value"_a)
        .def("add_document_reference", std::move(maybe_op_unwrap(&_native::LineItemBuilder::add_document_reference)), "value"_a)
        .def("add_tax_subtotal", std::move(maybe_op_unwrap(&_native::LineItemBuilder::add_tax_subtotal)), "value"_a)
        .def("build", std::move(maybe_op_unwrap(&_native::LineItemBuilder::build)))
        .def_static("new", std::move(maybe_op_unwrap(&_native::LineItemBuilder::new_)))
        .def("set_exemption", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_exemption)), "value"_a)
        .def("set_gross_amount_present", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_gross_amount_present)), "present"_a)
        .def("set_id", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_id)), "value"_a)
        .def("set_invoiced_quantity", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_invoiced_quantity)), "value"_a)
        .def("set_invoiced_quantity_unit_code", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_invoiced_quantity_unit_code)), "value"_a)
        .def("set_item_buyer_identifier", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_item_buyer_identifier)), "value"_a, "scheme"_a= nb::none())
        .def("set_item_gross_price", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_item_gross_price)), "value"_a)
        .def("set_item_name", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_item_name)), "value"_a)
        .def("set_item_net_price", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_item_net_price)), "value"_a)
        .def("set_item_price_base_quantity", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_item_price_base_quantity)), "value"_a)
        .def("set_item_price_base_quantity_unit_code", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_item_price_base_quantity_unit_code)), "value"_a)
        .def("set_item_price_discount", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_item_price_discount)), "value"_a)
        .def("set_item_seller_identifier", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_item_seller_identifier)), "value"_a, "scheme"_a= nb::none())
        .def("set_item_standard_identifier", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_item_standard_identifier)), "value"_a, "scheme"_a= nb::none())
        .def("set_line_amount_with_vat", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_line_amount_with_vat)), "value"_a)
        .def("set_line_net_amount", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_line_net_amount)), "value"_a)
        .def("set_line_vat_amount", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_line_vat_amount)), "value"_a)
        .def("set_tax_total_present", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_tax_total_present)), "present"_a)
        .def("set_vat_category", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_vat_category)), "value"_a)
        .def("set_vat_rate", std::move(maybe_op_unwrap(&_native::LineItemBuilder::set_vat_rate)), "value"_a);
}

} 
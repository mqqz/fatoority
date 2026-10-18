#include "diplomat_nanobind_common.hpp"


#include "TaxExemption.hpp"
#include "VatBreakdown.hpp"

namespace _native {
void add_VatBreakdown_binding(nb::module_ mod) {
    PyType_Slot _native_VatBreakdown_slots[] = {
        {Py_tp_free, (void *)_native::VatBreakdown::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::VatBreakdown> opaque(mod, "VatBreakdown", nb::type_slots(_native_VatBreakdown_slots));
    opaque
        .def("category", &_native::VatBreakdown::category)
        .def("exemption", std::move(maybe_op_unwrap(&_native::VatBreakdown::exemption)))
        .def_static("new", std::move(maybe_op_unwrap(&_native::VatBreakdown::new_)), "category"_a, "rate"_a, "taxable_amount"_a, "tax_amount"_a, "exemption"_a)
        .def("rate", std::move(maybe_op_unwrap(&_native::VatBreakdown::rate)))
        .def("tax_amount", std::move(maybe_op_unwrap(&_native::VatBreakdown::tax_amount)))
        .def("taxable_amount", std::move(maybe_op_unwrap(&_native::VatBreakdown::taxable_amount)));
}

} 
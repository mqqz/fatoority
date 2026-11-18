#include "diplomat_nanobind_common.hpp"


#include "AdjustmentKind.hpp"
#include "AllowanceCharge.hpp"
#include "TaxExemption.hpp"
#include "VatCategory.hpp"

namespace _native {
void add_AllowanceCharge_binding(nb::module_ mod) {
    PyType_Slot _native_AllowanceCharge_slots[] = {
        {Py_tp_free, (void *)_native::AllowanceCharge::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::AllowanceCharge> opaque(mod, "AllowanceCharge", nb::type_slots(_native_AllowanceCharge_slots));
    opaque
        .def("amount", std::move(maybe_op_unwrap(&_native::AllowanceCharge::amount)))
        .def("base_amount", &_native::AllowanceCharge::base_amount)
        .def("category", &_native::AllowanceCharge::category)
        .def("exemption", std::move(maybe_op_unwrap(&_native::AllowanceCharge::exemption)))
        .def("has_vat_rate", &_native::AllowanceCharge::has_vat_rate)
        .def("kind", &_native::AllowanceCharge::kind)
        .def_static("new", std::move(maybe_op_unwrap(&_native::AllowanceCharge::new_)), "kind"_a, "amount"_a, "reason"_a= nb::none(), "reason_code"_a= nb::none(), "category"_a, "rate"_a, "exemption"_a)
        .def("omit_vat_rate", std::move(maybe_op_unwrap(&_native::AllowanceCharge::omit_vat_rate)))
        .def("percentage", &_native::AllowanceCharge::percentage)
        .def("rate", std::move(maybe_op_unwrap(&_native::AllowanceCharge::rate)))
        .def("reason", &_native::AllowanceCharge::reason)
        .def("reason_code", &_native::AllowanceCharge::reason_code)
        .def("tax_category_count", &_native::AllowanceCharge::tax_category_count)
        .def("with_percentage", std::move(maybe_op_unwrap(&_native::AllowanceCharge::with_percentage)), "base"_a, "percent"_a, "Retain this adjustment's metadata and check its percentage calculation.")
        .def("with_tax_category_count", std::move(maybe_op_unwrap(&_native::AllowanceCharge::with_tax_category_count)), "count"_a, "Return an independent adjustment with the given number of identical tax classifications.");
}

} 
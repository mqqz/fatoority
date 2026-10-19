#include "diplomat_nanobind_common.hpp"


#include "AllowanceCharge.hpp"
#include "TaxExemption.hpp"

namespace _native {
void add_AllowanceCharge_binding(nb::module_ mod) {
    PyType_Slot _native_AllowanceCharge_slots[] = {
        {Py_tp_free, (void *)_native::AllowanceCharge::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::AllowanceCharge> opaque(mod, "AllowanceCharge", nb::type_slots(_native_AllowanceCharge_slots));
    opaque
        .def("amount", std::move(maybe_op_unwrap(&_native::AllowanceCharge::amount)))
        .def("base_amount", std::move(maybe_op_unwrap(&_native::AllowanceCharge::base_amount)))
        .def("category", &_native::AllowanceCharge::category)
        .def("exemption", std::move(maybe_op_unwrap(&_native::AllowanceCharge::exemption)))
        .def("is_charge", &_native::AllowanceCharge::is_charge)
        .def_static("new", std::move(maybe_op_unwrap(&_native::AllowanceCharge::new_)), "charge"_a, "amount"_a, "reason"_a= nb::none(), "reason_code"_a= nb::none(), "category"_a, "rate"_a, "exemption"_a)
        .def("percentage", std::move(maybe_op_unwrap(&_native::AllowanceCharge::percentage)))
        .def("rate", std::move(maybe_op_unwrap(&_native::AllowanceCharge::rate)))
        .def("reason", std::move(maybe_op_unwrap(&_native::AllowanceCharge::reason)))
        .def("reason_code", std::move(maybe_op_unwrap(&_native::AllowanceCharge::reason_code)))
        .def("tax_category_count", &_native::AllowanceCharge::tax_category_count)
        .def("with_tax_category_count", std::move(maybe_op_unwrap(&_native::AllowanceCharge::with_tax_category_count)), "count"_a, "Return an independent adjustment with the given number of identical tax classifications.");
}

} 
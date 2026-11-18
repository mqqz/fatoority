#include "diplomat_nanobind_common.hpp"


#include "Adjustment.hpp"

namespace _native {
void add_Adjustment_binding(nb::module_ mod) {
    PyType_Slot _native_Adjustment_slots[] = {
        {Py_tp_free, (void *)_native::Adjustment::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::Adjustment> opaque(mod, "Adjustment", nb::type_slots(_native_Adjustment_slots));
    opaque
        .def_static("amount", std::move(maybe_op_unwrap(&_native::Adjustment::amount)), "value"_a)
        .def("base_amount", &_native::Adjustment::base_amount)
        .def_static("percent_of", std::move(maybe_op_unwrap(&_native::Adjustment::percent_of)), "base"_a, "percent"_a, "Create an adjustment of `percent` percent of `base`.")
        .def("percentage", &_native::Adjustment::percentage)
        .def("reason", &_native::Adjustment::reason)
        .def("reason_code", &_native::Adjustment::reason_code)
        .def("supplied_amount", std::move(maybe_op_unwrap(&_native::Adjustment::supplied_amount)), "value"_a, "Return a copy with an explicit amount, checked against the percentage calculation.")
        .def("value", std::move(maybe_op_unwrap(&_native::Adjustment::value)))
        .def("with_reason", std::move(maybe_op_unwrap(&_native::Adjustment::with_reason)), "reason"_a= nb::none(), "code"_a= nb::none());
}

} 
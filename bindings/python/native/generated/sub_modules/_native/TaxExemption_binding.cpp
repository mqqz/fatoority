#include "diplomat_nanobind_common.hpp"


#include "TaxExemption.hpp"

namespace _native {
void add_TaxExemption_binding(nb::module_ mod) {
    PyType_Slot _native_TaxExemption_slots[] = {
        {Py_tp_free, (void *)_native::TaxExemption::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::TaxExemption> opaque(mod, "TaxExemption", nb::type_slots(_native_TaxExemption_slots));
    opaque
        .def_static("new", std::move(maybe_op_unwrap(&_native::TaxExemption::new_)), "code"_a= nb::none(), "reason"_a= nb::none())
        .def("reason", std::move(maybe_op_unwrap(&_native::TaxExemption::reason)))
        .def("reason_code", std::move(maybe_op_unwrap(&_native::TaxExemption::reason_code)));
}

} 
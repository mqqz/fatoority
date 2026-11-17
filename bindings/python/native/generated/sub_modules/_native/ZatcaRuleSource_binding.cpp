#include "diplomat_nanobind_common.hpp"


#include "ZatcaRuleSource.hpp"

namespace _native {
void add_ZatcaRuleSource_binding(nb::module_ mod) {
    PyType_Slot _native_ZatcaRuleSource_slots[] = {
        {Py_tp_free, (void *)_native::ZatcaRuleSource::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::ZatcaRuleSource> opaque(mod, "ZatcaRuleSource", nb::type_slots(_native_ZatcaRuleSource_slots), "Pinned upstream rule source of a business-rule stage.");
    opaque
        .def("sha256", std::move(maybe_op_unwrap(&_native::ZatcaRuleSource::sha256)), "Hex SHA-256 digest of the pinned stylesheet.")
        .def("stylesheet", std::move(maybe_op_unwrap(&_native::ZatcaRuleSource::stylesheet)), "File name of the upstream rule stylesheet.");
}

} 
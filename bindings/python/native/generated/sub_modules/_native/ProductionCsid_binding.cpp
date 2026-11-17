#include "diplomat_nanobind_common.hpp"


#include "Environment.hpp"
#include "ProductionCsid.hpp"

namespace _native {
void add_ProductionCsid_binding(nb::module_ mod) {
    PyType_Slot _native_ProductionCsid_slots[] = {
        {Py_tp_free, (void *)_native::ProductionCsid::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::ProductionCsid> opaque(mod, "ProductionCsid", nb::type_slots(_native_ProductionCsid_slots));
    opaque
        .def("binary_security_token", std::move(maybe_op_unwrap(&_native::ProductionCsid::binary_security_token)))
        .def_static("create", std::move(maybe_op_unwrap(&_native::ProductionCsid::create)), "environment"_a, "request_id"_a= nb::none(), "token"_a, "secret"_a)
        .def("env", std::move(maybe_op_unwrap(&_native::ProductionCsid::env)))
        .def("request_id", std::move(maybe_op_unwrap(&_native::ProductionCsid::request_id)))
        .def("secret", std::move(maybe_op_unwrap(&_native::ProductionCsid::secret)));
}

} 
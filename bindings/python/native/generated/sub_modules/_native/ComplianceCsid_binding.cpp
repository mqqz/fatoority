#include "diplomat_nanobind_common.hpp"


#include "ComplianceCsid.hpp"
#include "Environment.hpp"

namespace _native {
void add_ComplianceCsid_binding(nb::module_ mod) {
    PyType_Slot _native_ComplianceCsid_slots[] = {
        {Py_tp_free, (void *)_native::ComplianceCsid::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::ComplianceCsid> opaque(mod, "ComplianceCsid", nb::type_slots(_native_ComplianceCsid_slots));
    opaque
        .def("binary_security_token", std::move(maybe_op_unwrap(&_native::ComplianceCsid::binary_security_token)))
        .def("env", std::move(maybe_op_unwrap(&_native::ComplianceCsid::env)))
        .def_static("new", std::move(maybe_op_unwrap(&_native::ComplianceCsid::new_)), "environment"_a, "request_id"_a= nb::none(), "token"_a, "secret"_a)
        .def("request_id", &_native::ComplianceCsid::request_id)
        .def("secret", std::move(maybe_op_unwrap(&_native::ComplianceCsid::secret)));
}

} 
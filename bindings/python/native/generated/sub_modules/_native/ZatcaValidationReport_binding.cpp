#include "diplomat_nanobind_common.hpp"


#include "ZatcaValidationReport.hpp"

namespace _native {
void add_ZatcaValidationReport_binding(nb::module_ mod) {
    PyType_Slot _native_ZatcaValidationReport_slots[] = {
        {Py_tp_free, (void *)_native::ZatcaValidationReport::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::ZatcaValidationReport> opaque(mod, "ZatcaValidationReport", nb::type_slots(_native_ZatcaValidationReport_slots), "Owned results of local ZATCA validation with explicit stage coverage.\n\nCompletion and validity describe only the local profile; they do not\nestablish acceptance by ZATCA or certificate issuer trust.");
    opaque
        .def("evaluated_at", std::move(maybe_op_unwrap(&_native::ZatcaValidationReport::evaluated_at)), "RFC 3339 instant used for time-dependent rules.")
        .def("has_errors", &_native::ZatcaValidationReport::has_errors, "Whether any stage, including an incomplete stage, reported an error.")
        .def("is_complete", &_native::ZatcaValidationReport::is_complete, "Whether every required stage finished exactly once.")
        .def("is_valid", &_native::ZatcaValidationReport::is_valid, "Whether validation is complete and contains no error findings.")
        .def("profile", std::move(maybe_op_unwrap(&_native::ZatcaValidationReport::profile)), "Identifier of the validation profile (the ZATCA SDK release it mirrors).")
        .def("schema_version", &_native::ZatcaValidationReport::schema_version, "Version of the report's serialized format.")
        .def("stage", std::move(maybe_op_unwrap(&_native::ZatcaValidationReport::stage)), "index"_a, "The stage report at `index`, in stage order.")
        .def("stages_len", &_native::ZatcaValidationReport::stages_len)
        .def("to_json", std::move(maybe_op_unwrap(&_native::ZatcaValidationReport::to_json)), "The report serialized as JSON, including the derived `is_complete`,\n`has_errors` and `is_valid` fields.");
}

} 
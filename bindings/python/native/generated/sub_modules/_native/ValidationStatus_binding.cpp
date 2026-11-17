#include "diplomat_nanobind_common.hpp"


#include "ValidationStatus.hpp"

namespace _native {
void add_ValidationStatus_binding(nb::module_ mod) {
    nb::class_<_native::ValidationStatus> e_class(mod, "ValidationStatus", "Overall `validationResults.status`, or the `status` of one validation message.\n\n`Unknown` marks a value this binding version does not recognize; read the\nverbatim wire value with the sibling `*_raw` getter.");
    
        nb::enum_<_native::ValidationStatus::Value> enumerator(e_class, "ValidationStatus");
        enumerator
            .value("Unknown", _native::ValidationStatus::Unknown)
            .value("Pass", _native::ValidationStatus::Pass, "Wire value `PASS`.")
            .value("Warning", _native::ValidationStatus::Warning, "Wire value `WARNING`.")
            .value("Error", _native::ValidationStatus::Error, "Wire value `ERROR`.")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::ValidationStatus::Value>())
            .def(nb::self == _native::ValidationStatus::Value())
            .def("__repr__", [](const _native::ValidationStatus& self){
                return nb::str(nb::cast(_native::ValidationStatus::Value(self)));
            });
}

} 
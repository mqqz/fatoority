#include "diplomat_nanobind_common.hpp"


#include "Severity.hpp"

namespace _native {
void add_Severity_binding(nb::module_ mod) {
    nb::class_<_native::Severity> e_class(mod, "Severity", "Severity of a validation finding.");
    
        nb::enum_<_native::Severity::Value> enumerator(e_class, "Severity");
        enumerator
            .value("Warning", _native::Severity::Warning, "Reported but does not make the document invalid.")
            .value("Error", _native::Severity::Error, "Makes the document invalid.")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::Severity::Value>())
            .def(nb::self == _native::Severity::Value())
            .def("__repr__", [](const _native::Severity& self){
                return nb::str(nb::cast(_native::Severity::Value(self)));
            });
}

} 
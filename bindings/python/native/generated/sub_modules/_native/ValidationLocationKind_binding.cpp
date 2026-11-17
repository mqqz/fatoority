#include "diplomat_nanobind_common.hpp"


#include "ValidationLocationKind.hpp"

namespace _native {
void add_ValidationLocationKind_binding(nb::module_ mod) {
    nb::class_<_native::ValidationLocationKind> e_class(mod, "ValidationLocationKind", "Which kind of location a finding carries.\n\n`Unknown` marks a location kind this binding version does not recognize.");
    
        nb::enum_<_native::ValidationLocationKind::Value> enumerator(e_class, "ValidationLocationKind");
        enumerator
            .value("Unknown", _native::ValidationLocationKind::Unknown)
            .value("Field", _native::ValidationLocationKind::Field, "A model field path such as `line_items[0].quantity`; see `location_path`.")
            .value("XPath", _native::ValidationLocationKind::XPath, "A namespace-independent XPath; see `location_path`.")
            .value("Xml", _native::ValidationLocationKind::Xml, "One-based XML source coordinates; see `location_line`/`location_column`.")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::ValidationLocationKind::Value>())
            .def(nb::self == _native::ValidationLocationKind::Value())
            .def("__repr__", [](const _native::ValidationLocationKind& self){
                return nb::str(nb::cast(_native::ValidationLocationKind::Value(self)));
            });
}

} 
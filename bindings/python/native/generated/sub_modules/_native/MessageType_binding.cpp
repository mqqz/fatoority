#include "diplomat_nanobind_common.hpp"


#include "MessageType.hpp"

namespace _native {
void add_MessageType_binding(nb::module_ mod) {
    nb::class_<_native::MessageType> e_class(mod, "MessageType", "The `type` of a validation message.\n\n`Unknown` marks a value this binding version does not recognize; read the\nverbatim wire value with the sibling `*_raw` getter.");
    
        nb::enum_<_native::MessageType::Value> enumerator(e_class, "MessageType");
        enumerator
            .value("Unknown", _native::MessageType::Unknown)
            .value("Info", _native::MessageType::Info, "Wire value `INFO`.")
            .value("Warning", _native::MessageType::Warning, "Wire value `WARNING`.")
            .value("Error", _native::MessageType::Error, "Wire value `ERROR`.")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::MessageType::Value>())
            .def(nb::self == _native::MessageType::Value())
            .def("__repr__", [](const _native::MessageType& self){
                return nb::str(nb::cast(_native::MessageType::Value(self)));
            });
}

} 
#include "diplomat_nanobind_common.hpp"


#include "ClearanceStatus.hpp"

namespace _native {
void add_ClearanceStatus_binding(nb::module_ mod) {
    nb::class_<_native::ClearanceStatus> e_class(mod, "ClearanceStatus", "`clearanceStatus` of a clearance response.\n\n`Unknown` marks a value this binding version does not recognize; read the\nverbatim wire value with the sibling `*_raw` getter.");
    
        nb::enum_<_native::ClearanceStatus::Value> enumerator(e_class, "ClearanceStatus");
        enumerator
            .value("Unknown", _native::ClearanceStatus::Unknown)
            .value("Cleared", _native::ClearanceStatus::Cleared, "Wire value `CLEARED`.")
            .value("NotCleared", _native::ClearanceStatus::NotCleared, "Wire value `NOT_CLEARED`.")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::ClearanceStatus::Value>())
            .def(nb::self == _native::ClearanceStatus::Value())
            .def("__repr__", [](const _native::ClearanceStatus& self){
                return nb::str(nb::cast(_native::ClearanceStatus::Value(self)));
            });
}

} 
#include "diplomat_nanobind_common.hpp"


#include "ClearanceMode.hpp"

namespace _native {
void add_ClearanceMode_binding(nb::module_ mod) {
    nb::class_<_native::ClearanceMode> e_class(mod, "ClearanceMode", "Value of the `Clearance-Status` header sent with reporting and clearance requests.");
    
        nb::enum_<_native::ClearanceMode::Value> enumerator(e_class, "ClearanceMode");
        enumerator
            .value("Disabled", _native::ClearanceMode::Disabled, "Header value `0` (clearance is switched off for the taxpayer).")
            .value("Enabled", _native::ClearanceMode::Enabled, "Header value `1` (clearance is switched on for the taxpayer).")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::ClearanceMode::Value>())
            .def(nb::self == _native::ClearanceMode::Value())
            .def("__repr__", [](const _native::ClearanceMode& self){
                return nb::str(nb::cast(_native::ClearanceMode::Value(self)));
            });
}

} 
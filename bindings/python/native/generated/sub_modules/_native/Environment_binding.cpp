#include "diplomat_nanobind_common.hpp"


#include "Environment.hpp"

namespace _native {
void add_Environment_binding(nb::module_ mod) {
    nb::class_<_native::Environment> e_class(mod, "Environment", "ZATCA environment selecting the API base URL and certificate profile.\n\nDiscriminants match the `u8` codes used before 0.6. .NET names it\n`ZatcaEnvironment` so it cannot be confused with `System.Environment`.");
    
        nb::enum_<_native::Environment::Value> enumerator(e_class, "Environment");
        enumerator
            .value("NonProduction", _native::Environment::NonProduction)
            .value("Simulation", _native::Environment::Simulation)
            .value("Production", _native::Environment::Production)
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::Environment::Value>())
            .def(nb::self == _native::Environment::Value())
            .def("__repr__", [](const _native::Environment& self){
                return nb::str(nb::cast(_native::Environment::Value(self)));
            });
}

} 
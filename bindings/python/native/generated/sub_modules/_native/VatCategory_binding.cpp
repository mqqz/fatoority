#include "diplomat_nanobind_common.hpp"


#include "VatCategory.hpp"

namespace _native {
void add_VatCategory_binding(nb::module_ mod) {
    nb::class_<_native::VatCategory> e_class(mod, "VatCategory", "VAT category of a line, breakdown, or document-level adjustment.\n\nDiscriminants match the `u8` codes used before 0.6.");
    
        nb::enum_<_native::VatCategory::Value> enumerator(e_class, "VatCategory");
        enumerator
            .value("Exempt", _native::VatCategory::Exempt, "UNCL5305 `E`.")
            .value("Standard", _native::VatCategory::Standard, "UNCL5305 `S`.")
            .value("Zero", _native::VatCategory::Zero, "UNCL5305 `Z`.")
            .value("OutOfScope", _native::VatCategory::OutOfScope, "UNCL5305 `O`.")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::VatCategory::Value>())
            .def(nb::self == _native::VatCategory::Value())
            .def("__repr__", [](const _native::VatCategory& self){
                return nb::str(nb::cast(_native::VatCategory::Value(self)));
            });
}

} 
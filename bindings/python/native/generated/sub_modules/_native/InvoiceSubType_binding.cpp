#include "diplomat_nanobind_common.hpp"


#include "InvoiceSubType.hpp"

namespace _native {
void add_InvoiceSubType_binding(nb::module_ mod) {
    nb::class_<_native::InvoiceSubType> e_class(mod, "InvoiceSubType", "Standard (B2B, cleared) or simplified (B2C, reported) invoice.\n\nDiscriminants match the `u8` codes used before 0.6.");
    
        nb::enum_<_native::InvoiceSubType::Value> enumerator(e_class, "InvoiceSubType");
        enumerator
            .value("Standard", _native::InvoiceSubType::Standard)
            .value("Simplified", _native::InvoiceSubType::Simplified)
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::InvoiceSubType::Value>())
            .def(nb::self == _native::InvoiceSubType::Value())
            .def("__repr__", [](const _native::InvoiceSubType& self){
                return nb::str(nb::cast(_native::InvoiceSubType::Value(self)));
            });
}

} 
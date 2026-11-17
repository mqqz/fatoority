#include "diplomat_nanobind_common.hpp"


#include "InvoiceFlag.hpp"

namespace _native {
void add_InvoiceFlag_binding(nb::module_ mod) {
    nb::class_<_native::InvoiceFlag> e_class(mod, "InvoiceFlag", "Named bits of the `u8` invoice flag mask accepted by\n`InvoiceBuilder::flags` and returned by `InvoiceData::flags_raw`.\n\nEach discriminant is the flag's bit value; `bit()` returns it as a `u8`\nfor languages whose enums do not expose their discriminant.");
    
        nb::enum_<_native::InvoiceFlag::Value> enumerator(e_class, "InvoiceFlag");
        enumerator
            .value("ThirdParty", _native::InvoiceFlag::ThirdParty)
            .value("Nominal", _native::InvoiceFlag::Nominal)
            .value("Export", _native::InvoiceFlag::Export)
            .value("Summary", _native::InvoiceFlag::Summary)
            .value("SelfBilled", _native::InvoiceFlag::SelfBilled)
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::InvoiceFlag::Value>())
            .def(nb::self == _native::InvoiceFlag::Value())
            .def("__repr__", [](const _native::InvoiceFlag& self){
                return nb::str(nb::cast(_native::InvoiceFlag::Value(self)));
            });
}

} 
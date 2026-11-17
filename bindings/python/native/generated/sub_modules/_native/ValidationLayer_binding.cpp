#include "diplomat_nanobind_common.hpp"


#include "ValidationLayer.hpp"

namespace _native {
void add_ValidationLayer_binding(nb::module_ mod) {
    nb::class_<_native::ValidationLayer> e_class(mod, "ValidationLayer", "The validation layer that produced a finding.\n\n`Unknown` marks a layer this binding version does not recognize.");
    
        nb::enum_<_native::ValidationLayer::Value> enumerator(e_class, "ValidationLayer");
        enumerator
            .value("Unknown", _native::ValidationLayer::Unknown)
            .value("FieldChecks", _native::ValidationLayer::FieldChecks, "Builder/model field checks.")
            .value("Xsd", _native::ValidationLayer::Xsd, "UBL XML schema (XSD) validation.")
            .value("BusinessRules", _native::ValidationLayer::BusinessRules, "CEN EN 16931 and KSA business rules.")
            .value("Signature", _native::ValidationLayer::Signature, "Cryptographic signature, digest and certificate checks.")
            .value("Qr", _native::ValidationLayer::Qr, "QR payload consistency.")
            .value("PreviousInvoiceHash", _native::ValidationLayer::PreviousInvoiceHash, "Previous invoice hash comparison.")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::ValidationLayer::Value>())
            .def(nb::self == _native::ValidationLayer::Value())
            .def("__repr__", [](const _native::ValidationLayer& self){
                return nb::str(nb::cast(_native::ValidationLayer::Value(self)));
            });
}

} 
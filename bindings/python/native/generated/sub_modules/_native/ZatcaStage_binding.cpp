#include "diplomat_nanobind_common.hpp"


#include "ZatcaStage.hpp"

namespace _native {
void add_ZatcaStage_binding(nb::module_ mod) {
    nb::class_<_native::ZatcaStage> e_class(mod, "ZatcaStage", "A stage of the local ZATCA validation profile.\n\n`Unknown` marks a stage this binding version does not recognize.");
    
        nb::enum_<_native::ZatcaStage::Value> enumerator(e_class, "ZatcaStage");
        enumerator
            .value("Unknown", _native::ZatcaStage::Unknown)
            .value("Xsd", _native::ZatcaStage::Xsd, "UBL XML schema (XSD) validation.")
            .value("Cen", _native::ZatcaStage::Cen, "CEN EN 16931 business rules.")
            .value("Ksa", _native::ZatcaStage::Ksa, "ZATCA KSA-specific business rules.")
            .value("Signature", _native::ZatcaStage::Signature, "Cryptographic signature, digest and certificate checks.")
            .value("Qr", _native::ZatcaStage::Qr, "QR payload consistency checks.")
            .value("PreviousInvoiceHash", _native::ZatcaStage::PreviousInvoiceHash, "Comparison of the embedded previous invoice hash with the supplied one.")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::ZatcaStage::Value>())
            .def(nb::self == _native::ZatcaStage::Value())
            .def("__repr__", [](const _native::ZatcaStage& self){
                return nb::str(nb::cast(_native::ZatcaStage::Value(self)));
            });
}

} 
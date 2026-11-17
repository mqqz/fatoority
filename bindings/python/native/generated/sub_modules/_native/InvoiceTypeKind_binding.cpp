#include "diplomat_nanobind_common.hpp"


#include "InvoiceTypeKind.hpp"

namespace _native {
void add_InvoiceTypeKind_binding(nb::module_ mod) {
    nb::class_<_native::InvoiceTypeKind> e_class(mod, "InvoiceTypeKind", "Kind of invoice document (UNCL1001 type code).\n\nDiscriminants match the `u8` codes used before 0.6.");
    
        nb::enum_<_native::InvoiceTypeKind::Value> enumerator(e_class, "InvoiceTypeKind");
        enumerator
            .value("Tax", _native::InvoiceTypeKind::Tax, "Tax invoice (`388`).")
            .value("Prepayment", _native::InvoiceTypeKind::Prepayment, "Prepayment invoice (`386`).")
            .value("CreditNote", _native::InvoiceTypeKind::CreditNote, "Credit note (`381`).")
            .value("DebitNote", _native::InvoiceTypeKind::DebitNote, "Debit note (`383`).")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::InvoiceTypeKind::Value>())
            .def(nb::self == _native::InvoiceTypeKind::Value())
            .def("__repr__", [](const _native::InvoiceTypeKind& self){
                return nb::str(nb::cast(_native::InvoiceTypeKind::Value(self)));
            });
}

} 
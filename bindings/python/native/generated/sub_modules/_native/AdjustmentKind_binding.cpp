#include "diplomat_nanobind_common.hpp"


#include "AdjustmentKind.hpp"

namespace _native {
void add_AdjustmentKind_binding(nb::module_ mod) {
    nb::class_<_native::AdjustmentKind> e_class(mod, "AdjustmentKind", "Whether a document-level adjustment is an allowance (discount) or a charge.");
    
        nb::enum_<_native::AdjustmentKind::Value> enumerator(e_class, "AdjustmentKind");
        enumerator
            .value("Allowance", _native::AdjustmentKind::Allowance)
            .value("Charge", _native::AdjustmentKind::Charge)
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::AdjustmentKind::Value>())
            .def(nb::self == _native::AdjustmentKind::Value())
            .def("__repr__", [](const _native::AdjustmentKind& self){
                return nb::str(nb::cast(_native::AdjustmentKind::Value(self)));
            });
}

} 
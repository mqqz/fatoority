#include "diplomat_nanobind_common.hpp"


#include "ZatcaStageStatus.hpp"

namespace _native {
void add_ZatcaStageStatus_binding(nb::module_ mod) {
    nb::class_<_native::ZatcaStageStatus> e_class(mod, "ZatcaStageStatus", "Whether a stage ran and produced a complete result.\n\n`Unknown` marks a status this binding version does not recognize.");
    
        nb::enum_<_native::ZatcaStageStatus::Value> enumerator(e_class, "ZatcaStageStatus");
        enumerator
            .value("Unknown", _native::ZatcaStageStatus::Unknown)
            .value("NotRun", _native::ZatcaStageStatus::NotRun, "The stage did not start, typically because an earlier stage failed.")
            .value("Completed", _native::ZatcaStageStatus::Completed, "All checks finished; findings can still contain validation errors.")
            .value("NotApplicable", _native::ZatcaStageStatus::NotApplicable, "The profile does not require this stage for the document.")
            .value("ContextRequired", _native::ZatcaStageStatus::ContextRequired, "Required caller context was unavailable.")
            .value("EvaluationFailed", _native::ZatcaStageStatus::EvaluationFailed, "Execution stopped before the stage completed; findings may be partial.")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::ZatcaStageStatus::Value>())
            .def(nb::self == _native::ZatcaStageStatus::Value())
            .def("__repr__", [](const _native::ZatcaStageStatus& self){
                return nb::str(nb::cast(_native::ZatcaStageStatus::Value(self)));
            });
}

} 
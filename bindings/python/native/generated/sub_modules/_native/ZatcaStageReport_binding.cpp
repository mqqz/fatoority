#include "diplomat_nanobind_common.hpp"


#include "ZatcaStageReport.hpp"

namespace _native {
void add_ZatcaStageReport_binding(nb::module_ mod) {
    PyType_Slot _native_ZatcaStageReport_slots[] = {
        {Py_tp_free, (void *)_native::ZatcaStageReport::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::ZatcaStageReport> opaque(mod, "ZatcaStageReport", nb::type_slots(_native_ZatcaStageReport_slots), "Coverage and findings from one validation stage.");
    opaque
        .def("evaluated_assertion", std::move(maybe_op_unwrap(&_native::ZatcaStageReport::evaluated_assertion)), "index"_a, "Assertion site at `index` whose selected contexts were all evaluated.")
        .def("evaluated_assertions_len", &_native::ZatcaStageReport::evaluated_assertions_len)
        .def("finding", std::move(maybe_op_unwrap(&_native::ZatcaStageReport::finding)), "index"_a)
        .def("findings_len", &_native::ZatcaStageReport::findings_len)
        .def("provenance", std::move(maybe_op_unwrap(&_native::ZatcaStageReport::provenance)), "Pinned rule source, for the CEN and KSA business-rule stages.")
        .def("stage", &_native::ZatcaStageReport::stage)
        .def("status", &_native::ZatcaStageReport::status);
}

} 
#include "diplomat_nanobind_common.hpp"


#include "ReportingStatus.hpp"

namespace _native {
void add_ReportingStatus_binding(nb::module_ mod) {
    nb::class_<_native::ReportingStatus> e_class(mod, "ReportingStatus", "`reportingStatus` of a reporting response.\n\n`Unknown` marks a value this binding version does not recognize; read the\nverbatim wire value with the sibling `*_raw` getter.");
    
        nb::enum_<_native::ReportingStatus::Value> enumerator(e_class, "ReportingStatus");
        enumerator
            .value("Unknown", _native::ReportingStatus::Unknown)
            .value("Reported", _native::ReportingStatus::Reported, "Wire value `REPORTED`.")
            .value("NotReported", _native::ReportingStatus::NotReported, "Wire value `NOT_REPORTED`.")
            .export_values();
    
        e_class
            .def(nb::init_implicit<_native::ReportingStatus::Value>())
            .def(nb::self == _native::ReportingStatus::Value())
            .def("__repr__", [](const _native::ReportingStatus& self){
                return nb::str(nb::cast(_native::ReportingStatus::Value(self)));
            });
}

} 
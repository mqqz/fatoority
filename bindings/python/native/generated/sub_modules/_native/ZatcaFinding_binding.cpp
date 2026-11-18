#include "diplomat_nanobind_common.hpp"


#include "ZatcaFinding.hpp"

namespace _native {
void add_ZatcaFinding_binding(nb::module_ mod) {
    PyType_Slot _native_ZatcaFinding_slots[] = {
        {Py_tp_free, (void *)_native::ZatcaFinding::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::ZatcaFinding> opaque(mod, "ZatcaFinding", nb::type_slots(_native_ZatcaFinding_slots), "One validation finding with its source assertion, when applicable.");
    opaque
        .def("assertion_site", &_native::ZatcaFinding::assertion_site, "Rule assertion that produced the finding, for business-rule findings.")
        .def("code", std::move(maybe_op_unwrap(&_native::ZatcaFinding::code)), "Library-owned rule identifier, independent of message wording.")
        .def("layer", &_native::ZatcaFinding::layer)
        .def("location_column", &_native::ZatcaFinding::location_column, "One-based XML column, or null when unknown.")
        .def("location_kind", &_native::ZatcaFinding::location_kind, "The kind of location, or null when the source cannot locate the finding.")
        .def("location_line", &_native::ZatcaFinding::location_line, "One-based XML line, or null when the location is not XML coordinates.")
        .def("location_path", std::move(maybe_op_unwrap(&_native::ZatcaFinding::location_path)), "The field path or XPath, or null for other location kinds.")
        .def("message", std::move(maybe_op_unwrap(&_native::ZatcaFinding::message)), "Human-readable text; the wording is not a stable API.")
        .def("severity", &_native::ZatcaFinding::severity);
}

} 
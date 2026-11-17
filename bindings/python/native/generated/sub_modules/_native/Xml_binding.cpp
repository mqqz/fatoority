#include "diplomat_nanobind_common.hpp"


#include "Xml.hpp"

namespace _native {
void add_Xml_binding(nb::module_ mod) {
    PyType_Slot _native_Xml_slots[] = {
        {Py_tp_free, (void *)_native::Xml::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::Xml> opaque(mod, "Xml", nb::type_slots(_native_Xml_slots));
    opaque
        .def_static("hash", std::move(maybe_op_unwrap(&_native::Xml::hash)), "xml"_a)
        .def_static("validate", std::move(maybe_op_unwrap(&_native::Xml::validate)), "xml"_a, "Check the XML against the UBL schema and business rules, mirroring core\n`Validator::validate_xml`. Returns true or a validation error.")
        .def_static("validate_zatca", std::move(maybe_op_unwrap(&_native::Xml::validate_zatca)), "xml"_a, "options_json"_a= nb::none(), "Run the local ZATCA validation profile and return its owned report.\n\nInspect `is_valid` before acceptance. Options default when absent;\n`options_json` is limited to 4 KiB. Validation needs no configuration:\nit mirrors the stateless core `Validator::validate_zatca`.");
}

} 
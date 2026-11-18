#include "diplomat_nanobind_common.hpp"


#include "ValidationMessage.hpp"

namespace _native {
void add_ValidationMessage_binding(nb::module_ mod) {
    PyType_Slot _native_ValidationMessage_slots[] = {
        {Py_tp_free, (void *)_native::ValidationMessage::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::ValidationMessage> opaque(mod, "ValidationMessage", nb::type_slots(_native_ValidationMessage_slots));
    opaque
        .def("category", &_native::ValidationMessage::category)
        .def("code", &_native::ValidationMessage::code)
        .def("message", &_native::ValidationMessage::message)
        .def("message_type", &_native::ValidationMessage::message_type, "The parsed message `type`, or null when the message has none.")
        .def("message_type_raw", &_native::ValidationMessage::message_type_raw, "The message `type` exactly as ZATCA sent it, or null when absent.")
        .def("status", &_native::ValidationMessage::status, "The parsed message `status`, or null when the message has none.")
        .def("status_raw", &_native::ValidationMessage::status_raw, "The message `status` exactly as ZATCA sent it, or null when absent.");
}

} 
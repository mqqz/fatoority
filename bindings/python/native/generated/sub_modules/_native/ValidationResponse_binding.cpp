#include "diplomat_nanobind_common.hpp"


#include "ValidationResponse.hpp"

namespace _native {
void add_ValidationResponse_binding(nb::module_ mod) {
    PyType_Slot _native_ValidationResponse_slots[] = {
        {Py_tp_free, (void *)_native::ValidationResponse::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::ValidationResponse> opaque(mod, "ValidationResponse", nb::type_slots(_native_ValidationResponse_slots));
    opaque
        .def("clearance_status", &_native::ValidationResponse::clearance_status, "The parsed `clearanceStatus`, or null when the response has none.")
        .def("clearance_status_raw", std::move(maybe_op_unwrap(&_native::ValidationResponse::clearance_status_raw)), "The `clearanceStatus` exactly as ZATCA sent it, or null when absent.")
        .def("cleared_invoice_base64", std::move(maybe_op_unwrap(&_native::ValidationResponse::cleared_invoice_base64)))
        .def("cleared_invoice_xml", std::move(maybe_op_unwrap(&_native::ValidationResponse::cleared_invoice_xml)))
        .def("ensure_accepted", std::move(maybe_op_unwrap(&_native::ValidationResponse::ensure_accepted)))
        .def("http_status", &_native::ValidationResponse::http_status)
        .def("outcome", &_native::ValidationResponse::outcome)
        .def("qr_buyer_status", std::move(maybe_op_unwrap(&_native::ValidationResponse::qr_buyer_status)))
        .def("qr_seller_status", std::move(maybe_op_unwrap(&_native::ValidationResponse::qr_seller_status)))
        .def("reporting_status", &_native::ValidationResponse::reporting_status, "The parsed `reportingStatus`, or null when the response has none.")
        .def("reporting_status_raw", std::move(maybe_op_unwrap(&_native::ValidationResponse::reporting_status_raw)), "The `reportingStatus` exactly as ZATCA sent it, or null when absent.")
        .def("validation_results", std::move(maybe_op_unwrap(&_native::ValidationResponse::validation_results)));
}

} 
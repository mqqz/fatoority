#include "diplomat_nanobind_common.hpp"


#include "ClearanceMode.hpp"
#include "ComplianceCsid.hpp"
#include "Config.hpp"
#include "Csr.hpp"
#include "ProductionCsid.hpp"
#include "SignedInvoice.hpp"
#include "ZatcaClient.hpp"

namespace _native {
void add_ZatcaClient_binding(nb::module_ mod) {
    PyType_Slot _native_ZatcaClient_slots[] = {
        {Py_tp_free, (void *)_native::ZatcaClient::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::ZatcaClient> opaque(mod, "ZatcaClient", nb::type_slots(_native_ZatcaClient_slots));
    opaque
        .def("check_invoice_compliance", std::move(maybe_op_unwrap(&_native::ZatcaClient::check_invoice_compliance)), "invoice"_a, "credentials"_a)
        .def("clear_standard_invoice", std::move(maybe_op_unwrap(&_native::ZatcaClient::clear_standard_invoice)), "invoice"_a, "credentials"_a, "clearance_mode"_a, "accept_language"_a= nb::none())
        .def_static("create", std::move(maybe_op_unwrap(&_native::ZatcaClient::create)), "config"_a)
        .def("prepare_check_invoice_compliance", std::move(maybe_op_unwrap(&_native::ZatcaClient::prepare_check_invoice_compliance)), "invoice"_a, "credentials"_a)
        .def("prepare_clear_standard_invoice", std::move(maybe_op_unwrap(&_native::ZatcaClient::prepare_clear_standard_invoice)), "invoice"_a, "credentials"_a, "clearance_mode"_a, "accept_language"_a= nb::none())
        .def("prepare_renew_csid", std::move(maybe_op_unwrap(&_native::ZatcaClient::prepare_renew_csid)), "credentials"_a, "csr"_a, "otp"_a, "accept_language"_a= nb::none())
        .def("prepare_report_simplified_invoice", std::move(maybe_op_unwrap(&_native::ZatcaClient::prepare_report_simplified_invoice)), "invoice"_a, "credentials"_a, "clearance_mode"_a, "accept_language"_a= nb::none())
        .def("prepare_request_compliance_csid", std::move(maybe_op_unwrap(&_native::ZatcaClient::prepare_request_compliance_csid)), "csr"_a, "otp"_a)
        .def("prepare_request_production_csid", std::move(maybe_op_unwrap(&_native::ZatcaClient::prepare_request_production_csid)), "credentials"_a)
        .def("renew_csid", std::move(maybe_op_unwrap(&_native::ZatcaClient::renew_csid)), "credentials"_a, "csr"_a, "otp"_a, "accept_language"_a= nb::none())
        .def("report_simplified_invoice", std::move(maybe_op_unwrap(&_native::ZatcaClient::report_simplified_invoice)), "invoice"_a, "credentials"_a, "clearance_mode"_a, "accept_language"_a= nb::none())
        .def("request_compliance_csid", std::move(maybe_op_unwrap(&_native::ZatcaClient::request_compliance_csid)), "csr"_a, "otp"_a)
        .def("request_production_csid", std::move(maybe_op_unwrap(&_native::ZatcaClient::request_production_csid)), "credentials"_a);
    
    // Blocking operations release the GIL; the Python facade locks all input owners.
    opaque.def("_blocking_request_compliance_csid", maybe_op_unwrap(&_native::ZatcaClient::request_compliance_csid), nb::call_guard<nb::gil_scoped_release>());
    opaque.def("_blocking_request_production_csid", maybe_op_unwrap(&_native::ZatcaClient::request_production_csid), nb::call_guard<nb::gil_scoped_release>());
    opaque.def("_blocking_renew_csid", maybe_op_unwrap(&_native::ZatcaClient::renew_csid), nb::call_guard<nb::gil_scoped_release>());
    opaque.def("_blocking_check_invoice_compliance", maybe_op_unwrap(&_native::ZatcaClient::check_invoice_compliance), nb::call_guard<nb::gil_scoped_release>());
    opaque.def("_blocking_report_simplified_invoice", maybe_op_unwrap(&_native::ZatcaClient::report_simplified_invoice), nb::call_guard<nb::gil_scoped_release>());
    opaque.def("_blocking_clear_standard_invoice", maybe_op_unwrap(&_native::ZatcaClient::clear_standard_invoice), nb::call_guard<nb::gil_scoped_release>());
    
}

} 
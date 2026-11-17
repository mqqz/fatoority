#include "diplomat_nanobind_common.hpp"


#include "FinalizedInvoice.hpp"
#include "InvoiceSigner.hpp"

namespace _native {
void add_InvoiceSigner_binding(nb::module_ mod) {
    PyType_Slot _native_InvoiceSigner_slots[] = {
        {Py_tp_free, (void *)_native::InvoiceSigner::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::InvoiceSigner> opaque(mod, "InvoiceSigner", nb::type_slots(_native_InvoiceSigner_slots));
    opaque
        .def("certificate_der", std::move(maybe_op_unwrap(&_native::InvoiceSigner::certificate_der)))
        .def("certificate_pem", std::move(maybe_op_unwrap(&_native::InvoiceSigner::certificate_pem)))
        .def_static("from_der", std::move(maybe_op_unwrap(&_native::InvoiceSigner::from_der)), "cert_der"_a, "key_der"_a)
        .def_static("from_pem", std::move(maybe_op_unwrap(&_native::InvoiceSigner::from_pem)), "cert_pem"_a, "key_pem"_a)
        .def("sign", std::move(maybe_op_unwrap(&_native::InvoiceSigner::sign)), "invoice"_a)
        .def("sign_xml", std::move(maybe_op_unwrap(&_native::InvoiceSigner::sign_xml)), "xml"_a);
}

} 
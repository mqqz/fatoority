#include "diplomat_nanobind_common.hpp"


#include "PaymentMeans.hpp"

namespace _native {
void add_PaymentMeans_binding(nb::module_ mod) {
    PyType_Slot _native_PaymentMeans_slots[] = {
        {Py_tp_free, (void *)_native::PaymentMeans::operator delete },
        {Py_tp_dealloc, (void *)diplomat_tp_dealloc},
        {0, nullptr}};
    
    nb::class_<_native::PaymentMeans> opaque(mod, "PaymentMeans", nb::type_slots(_native_PaymentMeans_slots));
    opaque
        .def("account_identifier", std::move(maybe_op_unwrap(&_native::PaymentMeans::account_identifier)))
        .def("code", std::move(maybe_op_unwrap(&_native::PaymentMeans::code)))
        .def("has_account", &_native::PaymentMeans::has_account)
        .def("instruction_note", std::move(maybe_op_unwrap(&_native::PaymentMeans::instruction_note)))
        .def("instruction_note_at", std::move(maybe_op_unwrap(&_native::PaymentMeans::instruction_note_at)), "index"_a)
        .def("instruction_notes_len", &_native::PaymentMeans::instruction_notes_len)
        .def_static("new", std::move(maybe_op_unwrap(&_native::PaymentMeans::new_)), "code"_a, "instruction"_a= nb::none())
        .def("payment_terms", std::move(maybe_op_unwrap(&_native::PaymentMeans::payment_terms)))
        .def("with_account", std::move(maybe_op_unwrap(&_native::PaymentMeans::with_account)), "id"_a= nb::none(), "terms"_a= nb::none())
        .def("with_instruction_note", std::move(maybe_op_unwrap(&_native::PaymentMeans::with_instruction_note)), "value"_a);
}

} 
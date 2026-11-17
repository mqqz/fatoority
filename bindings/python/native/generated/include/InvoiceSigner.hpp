#ifndef _NATIVE_InvoiceSigner_HPP
#define _NATIVE_InvoiceSigner_HPP

#include "InvoiceSigner.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "Bytes.hpp"
#include "FinalizedInvoice.hpp"
#include "SignedInvoice.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_InvoiceSigner_from_pem_result {union {_native::capi::InvoiceSigner* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_from_pem_result;
    fatoora_InvoiceSigner_from_pem_result fatoora_InvoiceSigner_from_pem(_native::diplomat::capi::DiplomatStringView cert_pem, _native::diplomat::capi::DiplomatStringView key_pem);

    typedef struct fatoora_InvoiceSigner_from_der_result {union {_native::capi::InvoiceSigner* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_from_der_result;
    fatoora_InvoiceSigner_from_der_result fatoora_InvoiceSigner_from_der(_native::diplomat::capi::DiplomatU8View cert_der, _native::diplomat::capi::DiplomatU8View key_der);

    typedef struct fatoora_InvoiceSigner_certificate_der_result {union {_native::capi::Bytes* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_certificate_der_result;
    fatoora_InvoiceSigner_certificate_der_result fatoora_InvoiceSigner_certificate_der(const _native::capi::InvoiceSigner* self);

    typedef struct fatoora_InvoiceSigner_certificate_pem_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_certificate_pem_result;
    fatoora_InvoiceSigner_certificate_pem_result fatoora_InvoiceSigner_certificate_pem(const _native::capi::InvoiceSigner* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceSigner_sign_result {union {_native::capi::SignedInvoice* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_sign_result;
    fatoora_InvoiceSigner_sign_result fatoora_InvoiceSigner_sign(const _native::capi::InvoiceSigner* self, _native::capi::FinalizedInvoice* invoice);

    typedef struct fatoora_InvoiceSigner_sign_xml_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_sign_xml_result;
    fatoora_InvoiceSigner_sign_xml_result fatoora_InvoiceSigner_sign_xml(const _native::capi::InvoiceSigner* self, _native::diplomat::capi::DiplomatStringView xml, _native::diplomat::capi::DiplomatWrite* write);

    void fatoora_InvoiceSigner_destroy(InvoiceSigner* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::unique_ptr<_native::InvoiceSigner>, std::unique_ptr<_native::BindingError>> _native::InvoiceSigner::from_pem(std::string_view cert_pem, std::string_view key_pem) {
    auto result = _native::capi::fatoora_InvoiceSigner_from_pem({cert_pem.data(), cert_pem.size()},
        {key_pem.data(), key_pem.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::InvoiceSigner>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::InvoiceSigner>>(std::unique_ptr<_native::InvoiceSigner>(_native::InvoiceSigner::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::InvoiceSigner>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::InvoiceSigner>, std::unique_ptr<_native::BindingError>> _native::InvoiceSigner::from_der(_native::diplomat::span<const uint8_t> cert_der, _native::diplomat::span<const uint8_t> key_der) {
    auto result = _native::capi::fatoora_InvoiceSigner_from_der({cert_der.data(), cert_der.size()},
        {key_der.data(), key_der.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::InvoiceSigner>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::InvoiceSigner>>(std::unique_ptr<_native::InvoiceSigner>(_native::InvoiceSigner::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::InvoiceSigner>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>> _native::InvoiceSigner::certificate_der() const {
    auto result = _native::capi::fatoora_InvoiceSigner_certificate_der(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Bytes>>(std::unique_ptr<_native::Bytes>(_native::Bytes::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceSigner::certificate_pem() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceSigner_certificate_pem(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceSigner::certificate_pem_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceSigner_certificate_pem(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::SignedInvoice>, std::unique_ptr<_native::BindingError>> _native::InvoiceSigner::sign(_native::FinalizedInvoice& invoice) const {
    auto result = _native::capi::fatoora_InvoiceSigner_sign(this->AsFFI(),
        invoice.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::SignedInvoice>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::SignedInvoice>>(std::unique_ptr<_native::SignedInvoice>(_native::SignedInvoice::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::SignedInvoice>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceSigner::sign_xml(std::string_view xml) const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceSigner_sign_xml(this->AsFFI(),
        {xml.data(), xml.size()},
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceSigner::sign_xml_write(std::string_view xml, W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceSigner_sign_xml(this->AsFFI(),
        {xml.data(), xml.size()},
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::InvoiceSigner* _native::InvoiceSigner::AsFFI() const {
    return reinterpret_cast<const _native::capi::InvoiceSigner*>(this);
}

inline _native::capi::InvoiceSigner* _native::InvoiceSigner::AsFFI() {
    return reinterpret_cast<_native::capi::InvoiceSigner*>(this);
}

inline const _native::InvoiceSigner* _native::InvoiceSigner::FromFFI(const _native::capi::InvoiceSigner* ptr) {
    return reinterpret_cast<const _native::InvoiceSigner*>(ptr);
}

inline _native::InvoiceSigner* _native::InvoiceSigner::FromFFI(_native::capi::InvoiceSigner* ptr) {
    return reinterpret_cast<_native::InvoiceSigner*>(ptr);
}

inline void _native::InvoiceSigner::operator delete(void* ptr) {
    _native::capi::fatoora_InvoiceSigner_destroy(reinterpret_cast<_native::capi::InvoiceSigner*>(ptr));
}


#endif // _NATIVE_InvoiceSigner_HPP

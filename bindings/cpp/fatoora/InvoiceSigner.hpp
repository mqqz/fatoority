#ifndef fatoora_InvoiceSigner_HPP
#define fatoora_InvoiceSigner_HPP

#include "InvoiceSigner.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "BindingError.hpp"
#include "Bytes.hpp"
#include "FinalizedInvoice.hpp"
#include "SignedInvoice.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_InvoiceSigner_from_pem_result {union {fatoora::capi::InvoiceSigner* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_from_pem_result;
    fatoora_InvoiceSigner_from_pem_result fatoora_InvoiceSigner_from_pem(diplomat::capi::DiplomatStringView cert_pem, diplomat::capi::DiplomatStringView key_pem);

    typedef struct fatoora_InvoiceSigner_from_der_result {union {fatoora::capi::InvoiceSigner* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_from_der_result;
    fatoora_InvoiceSigner_from_der_result fatoora_InvoiceSigner_from_der(diplomat::capi::DiplomatU8View cert_der, diplomat::capi::DiplomatU8View key_der);

    typedef struct fatoora_InvoiceSigner_certificate_der_result {union {fatoora::capi::Bytes* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_certificate_der_result;
    fatoora_InvoiceSigner_certificate_der_result fatoora_InvoiceSigner_certificate_der(const fatoora::capi::InvoiceSigner* self);

    typedef struct fatoora_InvoiceSigner_certificate_pem_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_certificate_pem_result;
    fatoora_InvoiceSigner_certificate_pem_result fatoora_InvoiceSigner_certificate_pem(const fatoora::capi::InvoiceSigner* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceSigner_sign_result {union {fatoora::capi::SignedInvoice* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_sign_result;
    fatoora_InvoiceSigner_sign_result fatoora_InvoiceSigner_sign(const fatoora::capi::InvoiceSigner* self, fatoora::capi::FinalizedInvoice* invoice);

    typedef struct fatoora_InvoiceSigner_sign_xml_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceSigner_sign_xml_result;
    fatoora_InvoiceSigner_sign_xml_result fatoora_InvoiceSigner_sign_xml(const fatoora::capi::InvoiceSigner* self, diplomat::capi::DiplomatStringView xml, diplomat::capi::DiplomatWrite* write);

    void fatoora_InvoiceSigner_destroy(InvoiceSigner* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::InvoiceSigner>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceSigner::from_pem(std::string_view cert_pem, std::string_view key_pem) {
    auto result = fatoora::capi::fatoora_InvoiceSigner_from_pem({cert_pem.data(), cert_pem.size()},
        {key_pem.data(), key_pem.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::InvoiceSigner>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::InvoiceSigner>>(std::unique_ptr<fatoora::InvoiceSigner>(fatoora::InvoiceSigner::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::InvoiceSigner>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::InvoiceSigner>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceSigner::from_der(diplomat::span<const uint8_t> cert_der, diplomat::span<const uint8_t> key_der) {
    auto result = fatoora::capi::fatoora_InvoiceSigner_from_der({cert_der.data(), cert_der.size()},
        {key_der.data(), key_der.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::InvoiceSigner>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::InvoiceSigner>>(std::unique_ptr<fatoora::InvoiceSigner>(fatoora::InvoiceSigner::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::InvoiceSigner>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceSigner::certificate_der() const {
    auto result = fatoora::capi::fatoora_InvoiceSigner_certificate_der(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Bytes>>(std::unique_ptr<fatoora::Bytes>(fatoora::Bytes::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceSigner::certificate_pem() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceSigner_certificate_pem(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceSigner::certificate_pem_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceSigner_certificate_pem(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::SignedInvoice>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceSigner::sign(fatoora::FinalizedInvoice& invoice) const {
    auto result = fatoora::capi::fatoora_InvoiceSigner_sign(this->AsFFI(),
        invoice.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::SignedInvoice>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::SignedInvoice>>(std::unique_ptr<fatoora::SignedInvoice>(fatoora::SignedInvoice::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::SignedInvoice>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceSigner::sign_xml(std::string_view xml) const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceSigner_sign_xml(this->AsFFI(),
        {xml.data(), xml.size()},
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceSigner::sign_xml_write(std::string_view xml, W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceSigner_sign_xml(this->AsFFI(),
        {xml.data(), xml.size()},
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::InvoiceSigner* fatoora::InvoiceSigner::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::InvoiceSigner*>(this);
}

inline fatoora::capi::InvoiceSigner* fatoora::InvoiceSigner::AsFFI() {
    return reinterpret_cast<fatoora::capi::InvoiceSigner*>(this);
}

inline const fatoora::InvoiceSigner* fatoora::InvoiceSigner::FromFFI(const fatoora::capi::InvoiceSigner* ptr) {
    return reinterpret_cast<const fatoora::InvoiceSigner*>(ptr);
}

inline fatoora::InvoiceSigner* fatoora::InvoiceSigner::FromFFI(fatoora::capi::InvoiceSigner* ptr) {
    return reinterpret_cast<fatoora::InvoiceSigner*>(ptr);
}

inline void fatoora::InvoiceSigner::operator delete(void* ptr) {
    fatoora::capi::fatoora_InvoiceSigner_destroy(reinterpret_cast<fatoora::capi::InvoiceSigner*>(ptr));
}


#endif // fatoora_InvoiceSigner_HPP

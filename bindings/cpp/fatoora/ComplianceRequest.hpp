#ifndef fatoora_ComplianceRequest_HPP
#define fatoora_ComplianceRequest_HPP

#include "ComplianceRequest.d.hpp"

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
#include "ComplianceCsid.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_ComplianceRequest_method_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_method_result;
    fatoora_ComplianceRequest_method_result fatoora_ComplianceRequest_method(const fatoora::capi::ComplianceRequest* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ComplianceRequest_url_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_url_result;
    fatoora_ComplianceRequest_url_result fatoora_ComplianceRequest_url(const fatoora::capi::ComplianceRequest* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ComplianceRequest_headers_json_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_headers_json_result;
    fatoora_ComplianceRequest_headers_json_result fatoora_ComplianceRequest_headers_json(const fatoora::capi::ComplianceRequest* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ComplianceRequest_body_result {union {fatoora::capi::Bytes* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_body_result;
    fatoora_ComplianceRequest_body_result fatoora_ComplianceRequest_body(const fatoora::capi::ComplianceRequest* self);

    typedef struct fatoora_ComplianceRequest_decode_result {union {fatoora::capi::ComplianceCsid* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_decode_result;
    fatoora_ComplianceRequest_decode_result fatoora_ComplianceRequest_decode(const fatoora::capi::ComplianceRequest* self, uint16_t status, diplomat::capi::DiplomatU8View body);

    void fatoora_ComplianceRequest_destroy(ComplianceRequest* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceRequest::method() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ComplianceRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceRequest::method_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ComplianceRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceRequest::url() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ComplianceRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceRequest::url_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ComplianceRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceRequest::headers_json() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ComplianceRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceRequest::headers_json_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ComplianceRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceRequest::body() const {
    auto result = fatoora::capi::fatoora_ComplianceRequest_body(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Bytes>>(std::unique_ptr<fatoora::Bytes>(fatoora::Bytes::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceRequest::decode(uint16_t status, diplomat::span<const uint8_t> body) const {
    auto result = fatoora::capi::fatoora_ComplianceRequest_decode(this->AsFFI(),
        status,
        {body.data(), body.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ComplianceCsid>>(std::unique_ptr<fatoora::ComplianceCsid>(fatoora::ComplianceCsid::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::ComplianceRequest* fatoora::ComplianceRequest::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::ComplianceRequest*>(this);
}

inline fatoora::capi::ComplianceRequest* fatoora::ComplianceRequest::AsFFI() {
    return reinterpret_cast<fatoora::capi::ComplianceRequest*>(this);
}

inline const fatoora::ComplianceRequest* fatoora::ComplianceRequest::FromFFI(const fatoora::capi::ComplianceRequest* ptr) {
    return reinterpret_cast<const fatoora::ComplianceRequest*>(ptr);
}

inline fatoora::ComplianceRequest* fatoora::ComplianceRequest::FromFFI(fatoora::capi::ComplianceRequest* ptr) {
    return reinterpret_cast<fatoora::ComplianceRequest*>(ptr);
}

inline void fatoora::ComplianceRequest::operator delete(void* ptr) {
    fatoora::capi::fatoora_ComplianceRequest_destroy(reinterpret_cast<fatoora::capi::ComplianceRequest*>(ptr));
}


#endif // fatoora_ComplianceRequest_HPP

#ifndef fatoora_ProductionRequest_HPP
#define fatoora_ProductionRequest_HPP

#include "ProductionRequest.d.hpp"

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
#include "ProductionCsid.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_ProductionRequest_method_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_method_result;
    fatoora_ProductionRequest_method_result fatoora_ProductionRequest_method(const fatoora::capi::ProductionRequest* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ProductionRequest_url_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_url_result;
    fatoora_ProductionRequest_url_result fatoora_ProductionRequest_url(const fatoora::capi::ProductionRequest* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ProductionRequest_headers_json_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_headers_json_result;
    fatoora_ProductionRequest_headers_json_result fatoora_ProductionRequest_headers_json(const fatoora::capi::ProductionRequest* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ProductionRequest_body_result {union {fatoora::capi::Bytes* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_body_result;
    fatoora_ProductionRequest_body_result fatoora_ProductionRequest_body(const fatoora::capi::ProductionRequest* self);

    typedef struct fatoora_ProductionRequest_decode_result {union {fatoora::capi::ProductionCsid* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_decode_result;
    fatoora_ProductionRequest_decode_result fatoora_ProductionRequest_decode(const fatoora::capi::ProductionRequest* self, uint16_t status, diplomat::capi::DiplomatU8View body);

    void fatoora_ProductionRequest_destroy(ProductionRequest* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionRequest::method() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ProductionRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionRequest::method_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ProductionRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionRequest::url() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ProductionRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionRequest::url_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ProductionRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionRequest::headers_json() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ProductionRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionRequest::headers_json_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ProductionRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionRequest::body() const {
    auto result = fatoora::capi::fatoora_ProductionRequest_body(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Bytes>>(std::unique_ptr<fatoora::Bytes>(fatoora::Bytes::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionRequest::decode(uint16_t status, diplomat::span<const uint8_t> body) const {
    auto result = fatoora::capi::fatoora_ProductionRequest_decode(this->AsFFI(),
        status,
        {body.data(), body.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ProductionCsid>>(std::unique_ptr<fatoora::ProductionCsid>(fatoora::ProductionCsid::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::ProductionRequest* fatoora::ProductionRequest::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::ProductionRequest*>(this);
}

inline fatoora::capi::ProductionRequest* fatoora::ProductionRequest::AsFFI() {
    return reinterpret_cast<fatoora::capi::ProductionRequest*>(this);
}

inline const fatoora::ProductionRequest* fatoora::ProductionRequest::FromFFI(const fatoora::capi::ProductionRequest* ptr) {
    return reinterpret_cast<const fatoora::ProductionRequest*>(ptr);
}

inline fatoora::ProductionRequest* fatoora::ProductionRequest::FromFFI(fatoora::capi::ProductionRequest* ptr) {
    return reinterpret_cast<fatoora::ProductionRequest*>(ptr);
}

inline void fatoora::ProductionRequest::operator delete(void* ptr) {
    fatoora::capi::fatoora_ProductionRequest_destroy(reinterpret_cast<fatoora::capi::ProductionRequest*>(ptr));
}


#endif // fatoora_ProductionRequest_HPP

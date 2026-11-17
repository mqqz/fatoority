#ifndef _NATIVE_ProductionRequest_HPP
#define _NATIVE_ProductionRequest_HPP

#include "ProductionRequest.d.hpp"

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
#include "ProductionCsid.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_ProductionRequest_method_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_method_result;
    fatoora_ProductionRequest_method_result fatoora_ProductionRequest_method(const _native::capi::ProductionRequest* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ProductionRequest_url_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_url_result;
    fatoora_ProductionRequest_url_result fatoora_ProductionRequest_url(const _native::capi::ProductionRequest* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ProductionRequest_headers_json_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_headers_json_result;
    fatoora_ProductionRequest_headers_json_result fatoora_ProductionRequest_headers_json(const _native::capi::ProductionRequest* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ProductionRequest_body_result {union {_native::capi::Bytes* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_body_result;
    fatoora_ProductionRequest_body_result fatoora_ProductionRequest_body(const _native::capi::ProductionRequest* self);

    typedef struct fatoora_ProductionRequest_decode_result {union {_native::capi::ProductionCsid* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_decode_result;
    fatoora_ProductionRequest_decode_result fatoora_ProductionRequest_decode(const _native::capi::ProductionRequest* self, uint16_t status, _native::diplomat::capi::DiplomatU8View body);

    void fatoora_ProductionRequest_destroy(ProductionRequest* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ProductionRequest::method() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ProductionRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ProductionRequest::method_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ProductionRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ProductionRequest::url() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ProductionRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ProductionRequest::url_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ProductionRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ProductionRequest::headers_json() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ProductionRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ProductionRequest::headers_json_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ProductionRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>> _native::ProductionRequest::body() const {
    auto result = _native::capi::fatoora_ProductionRequest_body(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Bytes>>(std::unique_ptr<_native::Bytes>(_native::Bytes::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>> _native::ProductionRequest::decode(uint16_t status, _native::diplomat::span<const uint8_t> body) const {
    auto result = _native::capi::fatoora_ProductionRequest_decode(this->AsFFI(),
        status,
        {body.data(), body.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ProductionCsid>>(std::unique_ptr<_native::ProductionCsid>(_native::ProductionCsid::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::ProductionRequest* _native::ProductionRequest::AsFFI() const {
    return reinterpret_cast<const _native::capi::ProductionRequest*>(this);
}

inline _native::capi::ProductionRequest* _native::ProductionRequest::AsFFI() {
    return reinterpret_cast<_native::capi::ProductionRequest*>(this);
}

inline const _native::ProductionRequest* _native::ProductionRequest::FromFFI(const _native::capi::ProductionRequest* ptr) {
    return reinterpret_cast<const _native::ProductionRequest*>(ptr);
}

inline _native::ProductionRequest* _native::ProductionRequest::FromFFI(_native::capi::ProductionRequest* ptr) {
    return reinterpret_cast<_native::ProductionRequest*>(ptr);
}

inline void _native::ProductionRequest::operator delete(void* ptr) {
    _native::capi::fatoora_ProductionRequest_destroy(reinterpret_cast<_native::capi::ProductionRequest*>(ptr));
}


#endif // _NATIVE_ProductionRequest_HPP

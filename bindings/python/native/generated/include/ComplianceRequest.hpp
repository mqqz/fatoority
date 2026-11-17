#ifndef _NATIVE_ComplianceRequest_HPP
#define _NATIVE_ComplianceRequest_HPP

#include "ComplianceRequest.d.hpp"

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
#include "ComplianceCsid.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_ComplianceRequest_method_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_method_result;
    fatoora_ComplianceRequest_method_result fatoora_ComplianceRequest_method(const _native::capi::ComplianceRequest* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ComplianceRequest_url_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_url_result;
    fatoora_ComplianceRequest_url_result fatoora_ComplianceRequest_url(const _native::capi::ComplianceRequest* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ComplianceRequest_headers_json_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_headers_json_result;
    fatoora_ComplianceRequest_headers_json_result fatoora_ComplianceRequest_headers_json(const _native::capi::ComplianceRequest* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ComplianceRequest_body_result {union {_native::capi::Bytes* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_body_result;
    fatoora_ComplianceRequest_body_result fatoora_ComplianceRequest_body(const _native::capi::ComplianceRequest* self);

    typedef struct fatoora_ComplianceRequest_decode_result {union {_native::capi::ComplianceCsid* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_decode_result;
    fatoora_ComplianceRequest_decode_result fatoora_ComplianceRequest_decode(const _native::capi::ComplianceRequest* self, uint16_t status, _native::diplomat::capi::DiplomatU8View body);

    void fatoora_ComplianceRequest_destroy(ComplianceRequest* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ComplianceRequest::method() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ComplianceRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ComplianceRequest::method_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ComplianceRequest_method(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ComplianceRequest::url() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ComplianceRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ComplianceRequest::url_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ComplianceRequest_url(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ComplianceRequest::headers_json() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ComplianceRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ComplianceRequest::headers_json_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ComplianceRequest_headers_json(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>> _native::ComplianceRequest::body() const {
    auto result = _native::capi::fatoora_ComplianceRequest_body(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Bytes>>(std::unique_ptr<_native::Bytes>(_native::Bytes::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>> _native::ComplianceRequest::decode(uint16_t status, _native::diplomat::span<const uint8_t> body) const {
    auto result = _native::capi::fatoora_ComplianceRequest_decode(this->AsFFI(),
        status,
        {body.data(), body.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ComplianceCsid>>(std::unique_ptr<_native::ComplianceCsid>(_native::ComplianceCsid::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::ComplianceRequest* _native::ComplianceRequest::AsFFI() const {
    return reinterpret_cast<const _native::capi::ComplianceRequest*>(this);
}

inline _native::capi::ComplianceRequest* _native::ComplianceRequest::AsFFI() {
    return reinterpret_cast<_native::capi::ComplianceRequest*>(this);
}

inline const _native::ComplianceRequest* _native::ComplianceRequest::FromFFI(const _native::capi::ComplianceRequest* ptr) {
    return reinterpret_cast<const _native::ComplianceRequest*>(ptr);
}

inline _native::ComplianceRequest* _native::ComplianceRequest::FromFFI(_native::capi::ComplianceRequest* ptr) {
    return reinterpret_cast<_native::ComplianceRequest*>(ptr);
}

inline void _native::ComplianceRequest::operator delete(void* ptr) {
    _native::capi::fatoora_ComplianceRequest_destroy(reinterpret_cast<_native::capi::ComplianceRequest*>(ptr));
}


#endif // _NATIVE_ComplianceRequest_HPP

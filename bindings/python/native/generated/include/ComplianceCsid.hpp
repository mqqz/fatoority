#ifndef _NATIVE_ComplianceCsid_HPP
#define _NATIVE_ComplianceCsid_HPP

#include "ComplianceCsid.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "Environment.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_ComplianceCsid_new_result {union {_native::capi::ComplianceCsid* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_new_result;
    fatoora_ComplianceCsid_new_result fatoora_ComplianceCsid_new(_native::capi::Environment environment, _native::diplomat::capi::OptionStringView request_id, _native::diplomat::capi::DiplomatStringView token, _native::diplomat::capi::DiplomatStringView secret);

    typedef struct fatoora_ComplianceCsid_env_result {union {_native::capi::Environment ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_env_result;
    fatoora_ComplianceCsid_env_result fatoora_ComplianceCsid_env(const _native::capi::ComplianceCsid* self);

    typedef struct fatoora_ComplianceCsid_request_id_result { bool is_ok;} fatoora_ComplianceCsid_request_id_result;
    fatoora_ComplianceCsid_request_id_result fatoora_ComplianceCsid_request_id(const _native::capi::ComplianceCsid* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ComplianceCsid_binary_security_token_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_binary_security_token_result;
    fatoora_ComplianceCsid_binary_security_token_result fatoora_ComplianceCsid_binary_security_token(const _native::capi::ComplianceCsid* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ComplianceCsid_secret_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_secret_result;
    fatoora_ComplianceCsid_secret_result fatoora_ComplianceCsid_secret(const _native::capi::ComplianceCsid* self, _native::diplomat::capi::DiplomatWrite* write);

    void fatoora_ComplianceCsid_destroy(ComplianceCsid* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>> _native::ComplianceCsid::new_(_native::Environment environment, std::optional<std::string_view> request_id, std::string_view token, std::string_view secret) {
    auto result = _native::capi::fatoora_ComplianceCsid_new(environment.AsFFI(),
        request_id.has_value() ? (_native::diplomat::capi::OptionStringView{ { {request_id.value().data(), request_id.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }),
        {token.data(), token.size()},
        {secret.data(), secret.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ComplianceCsid>>(std::unique_ptr<_native::ComplianceCsid>(_native::ComplianceCsid::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<_native::Environment, std::unique_ptr<_native::BindingError>> _native::ComplianceCsid::env() const {
    auto result = _native::capi::fatoora_ComplianceCsid_env(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<_native::Environment, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<_native::Environment>(_native::Environment::FromFFI(result.ok))) : _native::diplomat::result<_native::Environment, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline std::optional<std::string> _native::ComplianceCsid::request_id() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ComplianceCsid_request_id(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ComplianceCsid::request_id_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ComplianceCsid_request_id(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ComplianceCsid::binary_security_token() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ComplianceCsid_binary_security_token(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ComplianceCsid::binary_security_token_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ComplianceCsid_binary_security_token(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ComplianceCsid::secret() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ComplianceCsid_secret(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ComplianceCsid::secret_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ComplianceCsid_secret(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::ComplianceCsid* _native::ComplianceCsid::AsFFI() const {
    return reinterpret_cast<const _native::capi::ComplianceCsid*>(this);
}

inline _native::capi::ComplianceCsid* _native::ComplianceCsid::AsFFI() {
    return reinterpret_cast<_native::capi::ComplianceCsid*>(this);
}

inline const _native::ComplianceCsid* _native::ComplianceCsid::FromFFI(const _native::capi::ComplianceCsid* ptr) {
    return reinterpret_cast<const _native::ComplianceCsid*>(ptr);
}

inline _native::ComplianceCsid* _native::ComplianceCsid::FromFFI(_native::capi::ComplianceCsid* ptr) {
    return reinterpret_cast<_native::ComplianceCsid*>(ptr);
}

inline void _native::ComplianceCsid::operator delete(void* ptr) {
    _native::capi::fatoora_ComplianceCsid_destroy(reinterpret_cast<_native::capi::ComplianceCsid*>(ptr));
}


#endif // _NATIVE_ComplianceCsid_HPP

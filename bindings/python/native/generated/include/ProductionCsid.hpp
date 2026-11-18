#ifndef _NATIVE_ProductionCsid_HPP
#define _NATIVE_ProductionCsid_HPP

#include "ProductionCsid.d.hpp"

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

    typedef struct fatoora_ProductionCsid_new_result {union {_native::capi::ProductionCsid* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_new_result;
    fatoora_ProductionCsid_new_result fatoora_ProductionCsid_new(_native::capi::Environment environment, _native::diplomat::capi::OptionStringView request_id, _native::diplomat::capi::DiplomatStringView token, _native::diplomat::capi::DiplomatStringView secret);

    typedef struct fatoora_ProductionCsid_env_result {union {_native::capi::Environment ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_env_result;
    fatoora_ProductionCsid_env_result fatoora_ProductionCsid_env(const _native::capi::ProductionCsid* self);

    typedef struct fatoora_ProductionCsid_request_id_result { bool is_ok;} fatoora_ProductionCsid_request_id_result;
    fatoora_ProductionCsid_request_id_result fatoora_ProductionCsid_request_id(const _native::capi::ProductionCsid* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ProductionCsid_binary_security_token_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_binary_security_token_result;
    fatoora_ProductionCsid_binary_security_token_result fatoora_ProductionCsid_binary_security_token(const _native::capi::ProductionCsid* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ProductionCsid_secret_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_secret_result;
    fatoora_ProductionCsid_secret_result fatoora_ProductionCsid_secret(const _native::capi::ProductionCsid* self, _native::diplomat::capi::DiplomatWrite* write);

    void fatoora_ProductionCsid_destroy(ProductionCsid* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>> _native::ProductionCsid::new_(_native::Environment environment, std::optional<std::string_view> request_id, std::string_view token, std::string_view secret) {
    auto result = _native::capi::fatoora_ProductionCsid_new(environment.AsFFI(),
        request_id.has_value() ? (_native::diplomat::capi::OptionStringView{ { {request_id.value().data(), request_id.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }),
        {token.data(), token.size()},
        {secret.data(), secret.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::ProductionCsid>>(std::unique_ptr<_native::ProductionCsid>(_native::ProductionCsid::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<_native::Environment, std::unique_ptr<_native::BindingError>> _native::ProductionCsid::env() const {
    auto result = _native::capi::fatoora_ProductionCsid_env(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<_native::Environment, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<_native::Environment>(_native::Environment::FromFFI(result.ok))) : _native::diplomat::result<_native::Environment, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline std::optional<std::string> _native::ProductionCsid::request_id() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ProductionCsid_request_id(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ProductionCsid::request_id_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ProductionCsid_request_id(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ProductionCsid::binary_security_token() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ProductionCsid_binary_security_token(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ProductionCsid::binary_security_token_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ProductionCsid_binary_security_token(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ProductionCsid::secret() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ProductionCsid_secret(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ProductionCsid::secret_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ProductionCsid_secret(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::ProductionCsid* _native::ProductionCsid::AsFFI() const {
    return reinterpret_cast<const _native::capi::ProductionCsid*>(this);
}

inline _native::capi::ProductionCsid* _native::ProductionCsid::AsFFI() {
    return reinterpret_cast<_native::capi::ProductionCsid*>(this);
}

inline const _native::ProductionCsid* _native::ProductionCsid::FromFFI(const _native::capi::ProductionCsid* ptr) {
    return reinterpret_cast<const _native::ProductionCsid*>(ptr);
}

inline _native::ProductionCsid* _native::ProductionCsid::FromFFI(_native::capi::ProductionCsid* ptr) {
    return reinterpret_cast<_native::ProductionCsid*>(ptr);
}

inline void _native::ProductionCsid::operator delete(void* ptr) {
    _native::capi::fatoora_ProductionCsid_destroy(reinterpret_cast<_native::capi::ProductionCsid*>(ptr));
}


#endif // _NATIVE_ProductionCsid_HPP

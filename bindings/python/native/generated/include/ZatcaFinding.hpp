#ifndef _NATIVE_ZatcaFinding_HPP
#define _NATIVE_ZatcaFinding_HPP

#include "ZatcaFinding.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "Severity.hpp"
#include "Text.hpp"
#include "ValidationLayer.hpp"
#include "ValidationLocationKind.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_ZatcaFinding_assertion_site_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaFinding_assertion_site_result;
    fatoora_ZatcaFinding_assertion_site_result fatoora_ZatcaFinding_assertion_site(const _native::capi::ZatcaFinding* self);

    _native::capi::ValidationLayer fatoora_ZatcaFinding_layer(const _native::capi::ZatcaFinding* self);

    typedef struct fatoora_ZatcaFinding_code_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaFinding_code_result;
    fatoora_ZatcaFinding_code_result fatoora_ZatcaFinding_code(const _native::capi::ZatcaFinding* self, _native::diplomat::capi::DiplomatWrite* write);

    _native::capi::Severity fatoora_ZatcaFinding_severity(const _native::capi::ZatcaFinding* self);

    typedef struct fatoora_ZatcaFinding_message_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaFinding_message_result;
    fatoora_ZatcaFinding_message_result fatoora_ZatcaFinding_message(const _native::capi::ZatcaFinding* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ZatcaFinding_location_kind_result {union {_native::capi::ValidationLocationKind ok; }; bool is_ok;} fatoora_ZatcaFinding_location_kind_result;
    fatoora_ZatcaFinding_location_kind_result fatoora_ZatcaFinding_location_kind(const _native::capi::ZatcaFinding* self);

    typedef struct fatoora_ZatcaFinding_location_path_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaFinding_location_path_result;
    fatoora_ZatcaFinding_location_path_result fatoora_ZatcaFinding_location_path(const _native::capi::ZatcaFinding* self);

    typedef struct fatoora_ZatcaFinding_location_line_result {union {uint32_t ok; }; bool is_ok;} fatoora_ZatcaFinding_location_line_result;
    fatoora_ZatcaFinding_location_line_result fatoora_ZatcaFinding_location_line(const _native::capi::ZatcaFinding* self);

    typedef struct fatoora_ZatcaFinding_location_column_result {union {uint32_t ok; }; bool is_ok;} fatoora_ZatcaFinding_location_column_result;
    fatoora_ZatcaFinding_location_column_result fatoora_ZatcaFinding_location_column(const _native::capi::ZatcaFinding* self);

    void fatoora_ZatcaFinding_destroy(ZatcaFinding* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::ZatcaFinding::assertion_site() const {
    auto result = _native::capi::fatoora_ZatcaFinding_assertion_site(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::ValidationLayer _native::ZatcaFinding::layer() const {
    auto result = _native::capi::fatoora_ZatcaFinding_layer(this->AsFFI());
    return _native::ValidationLayer::FromFFI(result);
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ZatcaFinding::code() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ZatcaFinding_code(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ZatcaFinding::code_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ZatcaFinding_code(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::Severity _native::ZatcaFinding::severity() const {
    auto result = _native::capi::fatoora_ZatcaFinding_severity(this->AsFFI());
    return _native::Severity::FromFFI(result);
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ZatcaFinding::message() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ZatcaFinding_message(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ZatcaFinding::message_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ZatcaFinding_message(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline std::optional<_native::ValidationLocationKind> _native::ZatcaFinding::location_kind() const {
    auto result = _native::capi::fatoora_ZatcaFinding_location_kind(this->AsFFI());
    return result.is_ok ? std::optional<_native::ValidationLocationKind>(_native::ValidationLocationKind::FromFFI(result.ok)) : std::nullopt;
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::ZatcaFinding::location_path() const {
    auto result = _native::capi::fatoora_ZatcaFinding_location_path(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline std::optional<uint32_t> _native::ZatcaFinding::location_line() const {
    auto result = _native::capi::fatoora_ZatcaFinding_location_line(this->AsFFI());
    return result.is_ok ? std::optional<uint32_t>(result.ok) : std::nullopt;
}

inline std::optional<uint32_t> _native::ZatcaFinding::location_column() const {
    auto result = _native::capi::fatoora_ZatcaFinding_location_column(this->AsFFI());
    return result.is_ok ? std::optional<uint32_t>(result.ok) : std::nullopt;
}

inline const _native::capi::ZatcaFinding* _native::ZatcaFinding::AsFFI() const {
    return reinterpret_cast<const _native::capi::ZatcaFinding*>(this);
}

inline _native::capi::ZatcaFinding* _native::ZatcaFinding::AsFFI() {
    return reinterpret_cast<_native::capi::ZatcaFinding*>(this);
}

inline const _native::ZatcaFinding* _native::ZatcaFinding::FromFFI(const _native::capi::ZatcaFinding* ptr) {
    return reinterpret_cast<const _native::ZatcaFinding*>(ptr);
}

inline _native::ZatcaFinding* _native::ZatcaFinding::FromFFI(_native::capi::ZatcaFinding* ptr) {
    return reinterpret_cast<_native::ZatcaFinding*>(ptr);
}

inline void _native::ZatcaFinding::operator delete(void* ptr) {
    _native::capi::fatoora_ZatcaFinding_destroy(reinterpret_cast<_native::capi::ZatcaFinding*>(ptr));
}


#endif // _NATIVE_ZatcaFinding_HPP

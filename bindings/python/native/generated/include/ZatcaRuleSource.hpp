#ifndef _NATIVE_ZatcaRuleSource_HPP
#define _NATIVE_ZatcaRuleSource_HPP

#include "ZatcaRuleSource.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_ZatcaRuleSource_stylesheet_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaRuleSource_stylesheet_result;
    fatoora_ZatcaRuleSource_stylesheet_result fatoora_ZatcaRuleSource_stylesheet(const _native::capi::ZatcaRuleSource* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ZatcaRuleSource_sha256_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaRuleSource_sha256_result;
    fatoora_ZatcaRuleSource_sha256_result fatoora_ZatcaRuleSource_sha256(const _native::capi::ZatcaRuleSource* self, _native::diplomat::capi::DiplomatWrite* write);

    void fatoora_ZatcaRuleSource_destroy(ZatcaRuleSource* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ZatcaRuleSource::stylesheet() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ZatcaRuleSource_stylesheet(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ZatcaRuleSource::stylesheet_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ZatcaRuleSource_stylesheet(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::ZatcaRuleSource::sha256() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ZatcaRuleSource_sha256(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::ZatcaRuleSource::sha256_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ZatcaRuleSource_sha256(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::ZatcaRuleSource* _native::ZatcaRuleSource::AsFFI() const {
    return reinterpret_cast<const _native::capi::ZatcaRuleSource*>(this);
}

inline _native::capi::ZatcaRuleSource* _native::ZatcaRuleSource::AsFFI() {
    return reinterpret_cast<_native::capi::ZatcaRuleSource*>(this);
}

inline const _native::ZatcaRuleSource* _native::ZatcaRuleSource::FromFFI(const _native::capi::ZatcaRuleSource* ptr) {
    return reinterpret_cast<const _native::ZatcaRuleSource*>(ptr);
}

inline _native::ZatcaRuleSource* _native::ZatcaRuleSource::FromFFI(_native::capi::ZatcaRuleSource* ptr) {
    return reinterpret_cast<_native::ZatcaRuleSource*>(ptr);
}

inline void _native::ZatcaRuleSource::operator delete(void* ptr) {
    _native::capi::fatoora_ZatcaRuleSource_destroy(reinterpret_cast<_native::capi::ZatcaRuleSource*>(ptr));
}


#endif // _NATIVE_ZatcaRuleSource_HPP

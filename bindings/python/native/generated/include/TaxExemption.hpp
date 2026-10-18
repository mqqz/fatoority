#ifndef _NATIVE_TaxExemption_HPP
#define _NATIVE_TaxExemption_HPP

#include "TaxExemption.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "Text.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_TaxExemption_new_result {union {_native::capi::TaxExemption* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_TaxExemption_new_result;
    fatoora_TaxExemption_new_result fatoora_TaxExemption_new(_native::diplomat::capi::OptionStringView code, _native::diplomat::capi::OptionStringView reason);

    typedef struct fatoora_TaxExemption_reason_code_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_TaxExemption_reason_code_result;
    fatoora_TaxExemption_reason_code_result fatoora_TaxExemption_reason_code(const _native::capi::TaxExemption* self);

    typedef struct fatoora_TaxExemption_reason_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_TaxExemption_reason_result;
    fatoora_TaxExemption_reason_result fatoora_TaxExemption_reason(const _native::capi::TaxExemption* self);

    void fatoora_TaxExemption_destroy(TaxExemption* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>> _native::TaxExemption::new_(std::optional<std::string_view> code, std::optional<std::string_view> reason) {
    auto result = _native::capi::fatoora_TaxExemption_new(code.has_value() ? (_native::diplomat::capi::OptionStringView{ { {code.value().data(), code.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }),
        reason.has_value() ? (_native::diplomat::capi::OptionStringView{ { {reason.value().data(), reason.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::TaxExemption>>(std::unique_ptr<_native::TaxExemption>(_native::TaxExemption::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::TaxExemption::reason_code() const {
    auto result = _native::capi::fatoora_TaxExemption_reason_code(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::TaxExemption::reason() const {
    auto result = _native::capi::fatoora_TaxExemption_reason(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::TaxExemption* _native::TaxExemption::AsFFI() const {
    return reinterpret_cast<const _native::capi::TaxExemption*>(this);
}

inline _native::capi::TaxExemption* _native::TaxExemption::AsFFI() {
    return reinterpret_cast<_native::capi::TaxExemption*>(this);
}

inline const _native::TaxExemption* _native::TaxExemption::FromFFI(const _native::capi::TaxExemption* ptr) {
    return reinterpret_cast<const _native::TaxExemption*>(ptr);
}

inline _native::TaxExemption* _native::TaxExemption::FromFFI(_native::capi::TaxExemption* ptr) {
    return reinterpret_cast<_native::TaxExemption*>(ptr);
}

inline void _native::TaxExemption::operator delete(void* ptr) {
    _native::capi::fatoora_TaxExemption_destroy(reinterpret_cast<_native::capi::TaxExemption*>(ptr));
}


#endif // _NATIVE_TaxExemption_HPP

#ifndef _NATIVE_Adjustment_HPP
#define _NATIVE_Adjustment_HPP

#include "Adjustment.d.hpp"

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

    typedef struct fatoora_Adjustment_amount_result {union {_native::capi::Adjustment* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_amount_result;
    fatoora_Adjustment_amount_result fatoora_Adjustment_amount(_native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_Adjustment_percentage_result {union {_native::capi::Adjustment* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_percentage_result;
    fatoora_Adjustment_percentage_result fatoora_Adjustment_percentage(_native::diplomat::capi::DiplomatStringView base, _native::diplomat::capi::DiplomatStringView percent);

    typedef struct fatoora_Adjustment_with_supplied_amount_result {union {_native::capi::Adjustment* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_with_supplied_amount_result;
    fatoora_Adjustment_with_supplied_amount_result fatoora_Adjustment_with_supplied_amount(const _native::capi::Adjustment* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_Adjustment_with_reason_result {union {_native::capi::Adjustment* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_with_reason_result;
    fatoora_Adjustment_with_reason_result fatoora_Adjustment_with_reason(const _native::capi::Adjustment* self, _native::diplomat::capi::OptionStringView reason, _native::diplomat::capi::OptionStringView code);

    typedef struct fatoora_Adjustment_value_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_value_result;
    fatoora_Adjustment_value_result fatoora_Adjustment_value(const _native::capi::Adjustment* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_Adjustment_base_amount_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_base_amount_result;
    fatoora_Adjustment_base_amount_result fatoora_Adjustment_base_amount(const _native::capi::Adjustment* self);

    typedef struct fatoora_Adjustment_percent_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_percent_result;
    fatoora_Adjustment_percent_result fatoora_Adjustment_percent(const _native::capi::Adjustment* self);

    typedef struct fatoora_Adjustment_reason_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_reason_result;
    fatoora_Adjustment_reason_result fatoora_Adjustment_reason(const _native::capi::Adjustment* self);

    typedef struct fatoora_Adjustment_reason_code_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_reason_code_result;
    fatoora_Adjustment_reason_code_result fatoora_Adjustment_reason_code(const _native::capi::Adjustment* self);

    void fatoora_Adjustment_destroy(Adjustment* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>> _native::Adjustment::amount(std::string_view value) {
    auto result = _native::capi::fatoora_Adjustment_amount({value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Adjustment>>(std::unique_ptr<_native::Adjustment>(_native::Adjustment::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>> _native::Adjustment::percentage(std::string_view base, std::string_view percent) {
    auto result = _native::capi::fatoora_Adjustment_percentage({base.data(), base.size()},
        {percent.data(), percent.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Adjustment>>(std::unique_ptr<_native::Adjustment>(_native::Adjustment::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>> _native::Adjustment::with_supplied_amount(std::string_view value) const {
    auto result = _native::capi::fatoora_Adjustment_with_supplied_amount(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Adjustment>>(std::unique_ptr<_native::Adjustment>(_native::Adjustment::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>> _native::Adjustment::with_reason(std::optional<std::string_view> reason, std::optional<std::string_view> code) const {
    auto result = _native::capi::fatoora_Adjustment_with_reason(this->AsFFI(),
        reason.has_value() ? (_native::diplomat::capi::OptionStringView{ { {reason.value().data(), reason.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }),
        code.has_value() ? (_native::diplomat::capi::OptionStringView{ { {code.value().data(), code.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Adjustment>>(std::unique_ptr<_native::Adjustment>(_native::Adjustment::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::Adjustment::value() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_Adjustment_value(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::Adjustment::value_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_Adjustment_value(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::Adjustment::base_amount() const {
    auto result = _native::capi::fatoora_Adjustment_base_amount(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::Adjustment::percent() const {
    auto result = _native::capi::fatoora_Adjustment_percent(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::Adjustment::reason() const {
    auto result = _native::capi::fatoora_Adjustment_reason(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::Adjustment::reason_code() const {
    auto result = _native::capi::fatoora_Adjustment_reason_code(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::Adjustment* _native::Adjustment::AsFFI() const {
    return reinterpret_cast<const _native::capi::Adjustment*>(this);
}

inline _native::capi::Adjustment* _native::Adjustment::AsFFI() {
    return reinterpret_cast<_native::capi::Adjustment*>(this);
}

inline const _native::Adjustment* _native::Adjustment::FromFFI(const _native::capi::Adjustment* ptr) {
    return reinterpret_cast<const _native::Adjustment*>(ptr);
}

inline _native::Adjustment* _native::Adjustment::FromFFI(_native::capi::Adjustment* ptr) {
    return reinterpret_cast<_native::Adjustment*>(ptr);
}

inline void _native::Adjustment::operator delete(void* ptr) {
    _native::capi::fatoora_Adjustment_destroy(reinterpret_cast<_native::capi::Adjustment*>(ptr));
}


#endif // _NATIVE_Adjustment_HPP

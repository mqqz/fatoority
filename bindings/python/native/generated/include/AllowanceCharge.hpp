#ifndef _NATIVE_AllowanceCharge_HPP
#define _NATIVE_AllowanceCharge_HPP

#include "AllowanceCharge.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "TaxExemption.hpp"
#include "Text.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    bool fatoora_AllowanceCharge_has_vat_rate(const _native::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_without_vat_rate_result {union {_native::capi::AllowanceCharge* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_without_vat_rate_result;
    fatoora_AllowanceCharge_without_vat_rate_result fatoora_AllowanceCharge_without_vat_rate(const _native::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_with_percentage_result {union {_native::capi::AllowanceCharge* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_with_percentage_result;
    fatoora_AllowanceCharge_with_percentage_result fatoora_AllowanceCharge_with_percentage(const _native::capi::AllowanceCharge* self, _native::diplomat::capi::DiplomatStringView base, _native::diplomat::capi::DiplomatStringView percent);

    typedef struct fatoora_AllowanceCharge_base_amount_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_base_amount_result;
    fatoora_AllowanceCharge_base_amount_result fatoora_AllowanceCharge_base_amount(const _native::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_percentage_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_percentage_result;
    fatoora_AllowanceCharge_percentage_result fatoora_AllowanceCharge_percentage(const _native::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_new_result {union {_native::capi::AllowanceCharge* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_new_result;
    fatoora_AllowanceCharge_new_result fatoora_AllowanceCharge_new(bool charge, _native::diplomat::capi::DiplomatStringView amount, _native::diplomat::capi::OptionStringView reason, _native::diplomat::capi::OptionStringView reason_code, uint8_t category, _native::diplomat::capi::DiplomatStringView rate, const _native::capi::TaxExemption* exemption);

    bool fatoora_AllowanceCharge_is_charge(const _native::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_amount_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_amount_result;
    fatoora_AllowanceCharge_amount_result fatoora_AllowanceCharge_amount(const _native::capi::AllowanceCharge* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_AllowanceCharge_rate_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_rate_result;
    fatoora_AllowanceCharge_rate_result fatoora_AllowanceCharge_rate(const _native::capi::AllowanceCharge* self, _native::diplomat::capi::DiplomatWrite* write);

    uint8_t fatoora_AllowanceCharge_category(const _native::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_with_tax_category_count_result {union {_native::capi::AllowanceCharge* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_with_tax_category_count_result;
    fatoora_AllowanceCharge_with_tax_category_count_result fatoora_AllowanceCharge_with_tax_category_count(const _native::capi::AllowanceCharge* self, size_t count);

    size_t fatoora_AllowanceCharge_tax_category_count(const _native::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_reason_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_reason_result;
    fatoora_AllowanceCharge_reason_result fatoora_AllowanceCharge_reason(const _native::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_reason_code_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_reason_code_result;
    fatoora_AllowanceCharge_reason_code_result fatoora_AllowanceCharge_reason_code(const _native::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_exemption_result {union {_native::capi::TaxExemption* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_exemption_result;
    fatoora_AllowanceCharge_exemption_result fatoora_AllowanceCharge_exemption(const _native::capi::AllowanceCharge* self);

    void fatoora_AllowanceCharge_destroy(AllowanceCharge* self);

    } // extern "C"
} // namespace capi
} // namespace

inline bool _native::AllowanceCharge::has_vat_rate() const {
    auto result = _native::capi::fatoora_AllowanceCharge_has_vat_rate(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::without_vat_rate() const {
    auto result = _native::capi::fatoora_AllowanceCharge_without_vat_rate(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::AllowanceCharge>>(std::unique_ptr<_native::AllowanceCharge>(_native::AllowanceCharge::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::with_percentage(std::string_view base, std::string_view percent) const {
    auto result = _native::capi::fatoora_AllowanceCharge_with_percentage(this->AsFFI(),
        {base.data(), base.size()},
        {percent.data(), percent.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::AllowanceCharge>>(std::unique_ptr<_native::AllowanceCharge>(_native::AllowanceCharge::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::base_amount() const {
    auto result = _native::capi::fatoora_AllowanceCharge_base_amount(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::percentage() const {
    auto result = _native::capi::fatoora_AllowanceCharge_percentage(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::new_(bool charge, std::string_view amount, std::optional<std::string_view> reason, std::optional<std::string_view> reason_code, uint8_t category, std::string_view rate, const _native::TaxExemption& exemption) {
    auto result = _native::capi::fatoora_AllowanceCharge_new(charge,
        {amount.data(), amount.size()},
        reason.has_value() ? (_native::diplomat::capi::OptionStringView{ { {reason.value().data(), reason.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }),
        reason_code.has_value() ? (_native::diplomat::capi::OptionStringView{ { {reason_code.value().data(), reason_code.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }),
        category,
        {rate.data(), rate.size()},
        exemption.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::AllowanceCharge>>(std::unique_ptr<_native::AllowanceCharge>(_native::AllowanceCharge::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline bool _native::AllowanceCharge::is_charge() const {
    auto result = _native::capi::fatoora_AllowanceCharge_is_charge(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::amount() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_AllowanceCharge_amount(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::amount_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_AllowanceCharge_amount(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::rate() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_AllowanceCharge_rate(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::rate_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_AllowanceCharge_rate(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline uint8_t _native::AllowanceCharge::category() const {
    auto result = _native::capi::fatoora_AllowanceCharge_category(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::with_tax_category_count(size_t count) const {
    auto result = _native::capi::fatoora_AllowanceCharge_with_tax_category_count(this->AsFFI(),
        count);
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::AllowanceCharge>>(std::unique_ptr<_native::AllowanceCharge>(_native::AllowanceCharge::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline size_t _native::AllowanceCharge::tax_category_count() const {
    auto result = _native::capi::fatoora_AllowanceCharge_tax_category_count(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::reason() const {
    auto result = _native::capi::fatoora_AllowanceCharge_reason(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::reason_code() const {
    auto result = _native::capi::fatoora_AllowanceCharge_reason_code(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>> _native::AllowanceCharge::exemption() const {
    auto result = _native::capi::fatoora_AllowanceCharge_exemption(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::TaxExemption>>(std::unique_ptr<_native::TaxExemption>(_native::TaxExemption::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::AllowanceCharge* _native::AllowanceCharge::AsFFI() const {
    return reinterpret_cast<const _native::capi::AllowanceCharge*>(this);
}

inline _native::capi::AllowanceCharge* _native::AllowanceCharge::AsFFI() {
    return reinterpret_cast<_native::capi::AllowanceCharge*>(this);
}

inline const _native::AllowanceCharge* _native::AllowanceCharge::FromFFI(const _native::capi::AllowanceCharge* ptr) {
    return reinterpret_cast<const _native::AllowanceCharge*>(ptr);
}

inline _native::AllowanceCharge* _native::AllowanceCharge::FromFFI(_native::capi::AllowanceCharge* ptr) {
    return reinterpret_cast<_native::AllowanceCharge*>(ptr);
}

inline void _native::AllowanceCharge::operator delete(void* ptr) {
    _native::capi::fatoora_AllowanceCharge_destroy(reinterpret_cast<_native::capi::AllowanceCharge*>(ptr));
}


#endif // _NATIVE_AllowanceCharge_HPP

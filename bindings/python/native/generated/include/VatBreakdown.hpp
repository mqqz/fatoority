#ifndef _NATIVE_VatBreakdown_HPP
#define _NATIVE_VatBreakdown_HPP

#include "VatBreakdown.d.hpp"

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
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_VatBreakdown_new_result {union {_native::capi::VatBreakdown* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_new_result;
    fatoora_VatBreakdown_new_result fatoora_VatBreakdown_new(uint8_t category, _native::diplomat::capi::DiplomatStringView rate, _native::diplomat::capi::DiplomatStringView taxable_amount, _native::diplomat::capi::DiplomatStringView tax_amount, const _native::capi::TaxExemption* exemption);

    uint8_t fatoora_VatBreakdown_category(const _native::capi::VatBreakdown* self);

    typedef struct fatoora_VatBreakdown_rate_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_rate_result;
    fatoora_VatBreakdown_rate_result fatoora_VatBreakdown_rate(const _native::capi::VatBreakdown* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_VatBreakdown_taxable_amount_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_taxable_amount_result;
    fatoora_VatBreakdown_taxable_amount_result fatoora_VatBreakdown_taxable_amount(const _native::capi::VatBreakdown* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_VatBreakdown_tax_amount_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_tax_amount_result;
    fatoora_VatBreakdown_tax_amount_result fatoora_VatBreakdown_tax_amount(const _native::capi::VatBreakdown* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_VatBreakdown_exemption_result {union {_native::capi::TaxExemption* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_exemption_result;
    fatoora_VatBreakdown_exemption_result fatoora_VatBreakdown_exemption(const _native::capi::VatBreakdown* self);

    void fatoora_VatBreakdown_destroy(VatBreakdown* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::unique_ptr<_native::VatBreakdown>, std::unique_ptr<_native::BindingError>> _native::VatBreakdown::new_(uint8_t category, std::string_view rate, std::string_view taxable_amount, std::string_view tax_amount, const _native::TaxExemption& exemption) {
    auto result = _native::capi::fatoora_VatBreakdown_new(category,
        {rate.data(), rate.size()},
        {taxable_amount.data(), taxable_amount.size()},
        {tax_amount.data(), tax_amount.size()},
        exemption.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::VatBreakdown>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::VatBreakdown>>(std::unique_ptr<_native::VatBreakdown>(_native::VatBreakdown::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::VatBreakdown>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline uint8_t _native::VatBreakdown::category() const {
    auto result = _native::capi::fatoora_VatBreakdown_category(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::VatBreakdown::rate() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_VatBreakdown_rate(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::VatBreakdown::rate_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_VatBreakdown_rate(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::VatBreakdown::taxable_amount() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_VatBreakdown_taxable_amount(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::VatBreakdown::taxable_amount_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_VatBreakdown_taxable_amount(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::VatBreakdown::tax_amount() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_VatBreakdown_tax_amount(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::VatBreakdown::tax_amount_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_VatBreakdown_tax_amount(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>> _native::VatBreakdown::exemption() const {
    auto result = _native::capi::fatoora_VatBreakdown_exemption(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::TaxExemption>>(std::unique_ptr<_native::TaxExemption>(_native::TaxExemption::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::VatBreakdown* _native::VatBreakdown::AsFFI() const {
    return reinterpret_cast<const _native::capi::VatBreakdown*>(this);
}

inline _native::capi::VatBreakdown* _native::VatBreakdown::AsFFI() {
    return reinterpret_cast<_native::capi::VatBreakdown*>(this);
}

inline const _native::VatBreakdown* _native::VatBreakdown::FromFFI(const _native::capi::VatBreakdown* ptr) {
    return reinterpret_cast<const _native::VatBreakdown*>(ptr);
}

inline _native::VatBreakdown* _native::VatBreakdown::FromFFI(_native::capi::VatBreakdown* ptr) {
    return reinterpret_cast<_native::VatBreakdown*>(ptr);
}

inline void _native::VatBreakdown::operator delete(void* ptr) {
    _native::capi::fatoora_VatBreakdown_destroy(reinterpret_cast<_native::capi::VatBreakdown*>(ptr));
}


#endif // _NATIVE_VatBreakdown_HPP

#ifndef fatoora_VatBreakdown_HPP
#define fatoora_VatBreakdown_HPP

#include "VatBreakdown.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "BindingError.hpp"
#include "TaxExemption.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_VatBreakdown_new_result {union {fatoora::capi::VatBreakdown* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_new_result;
    fatoora_VatBreakdown_new_result fatoora_VatBreakdown_new(uint8_t category, diplomat::capi::DiplomatStringView rate, diplomat::capi::DiplomatStringView taxable_amount, diplomat::capi::DiplomatStringView tax_amount, const fatoora::capi::TaxExemption* exemption);

    uint8_t fatoora_VatBreakdown_category(const fatoora::capi::VatBreakdown* self);

    typedef struct fatoora_VatBreakdown_rate_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_rate_result;
    fatoora_VatBreakdown_rate_result fatoora_VatBreakdown_rate(const fatoora::capi::VatBreakdown* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_VatBreakdown_taxable_amount_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_taxable_amount_result;
    fatoora_VatBreakdown_taxable_amount_result fatoora_VatBreakdown_taxable_amount(const fatoora::capi::VatBreakdown* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_VatBreakdown_tax_amount_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_tax_amount_result;
    fatoora_VatBreakdown_tax_amount_result fatoora_VatBreakdown_tax_amount(const fatoora::capi::VatBreakdown* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_VatBreakdown_exemption_result {union {fatoora::capi::TaxExemption* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_exemption_result;
    fatoora_VatBreakdown_exemption_result fatoora_VatBreakdown_exemption(const fatoora::capi::VatBreakdown* self);

    void fatoora_VatBreakdown_destroy(VatBreakdown* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::VatBreakdown>, std::unique_ptr<fatoora::BindingError>> fatoora::VatBreakdown::new_(uint8_t category, std::string_view rate, std::string_view taxable_amount, std::string_view tax_amount, const fatoora::TaxExemption& exemption) {
    auto result = fatoora::capi::fatoora_VatBreakdown_new(category,
        {rate.data(), rate.size()},
        {taxable_amount.data(), taxable_amount.size()},
        {tax_amount.data(), tax_amount.size()},
        exemption.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::VatBreakdown>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::VatBreakdown>>(std::unique_ptr<fatoora::VatBreakdown>(fatoora::VatBreakdown::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::VatBreakdown>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline uint8_t fatoora::VatBreakdown::category() const {
    auto result = fatoora::capi::fatoora_VatBreakdown_category(this->AsFFI());
    return result;
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::VatBreakdown::rate() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_VatBreakdown_rate(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::VatBreakdown::rate_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_VatBreakdown_rate(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::VatBreakdown::taxable_amount() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_VatBreakdown_taxable_amount(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::VatBreakdown::taxable_amount_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_VatBreakdown_taxable_amount(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::VatBreakdown::tax_amount() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_VatBreakdown_tax_amount(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::VatBreakdown::tax_amount_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_VatBreakdown_tax_amount(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>> fatoora::VatBreakdown::exemption() const {
    auto result = fatoora::capi::fatoora_VatBreakdown_exemption(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::TaxExemption>>(std::unique_ptr<fatoora::TaxExemption>(fatoora::TaxExemption::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::VatBreakdown* fatoora::VatBreakdown::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::VatBreakdown*>(this);
}

inline fatoora::capi::VatBreakdown* fatoora::VatBreakdown::AsFFI() {
    return reinterpret_cast<fatoora::capi::VatBreakdown*>(this);
}

inline const fatoora::VatBreakdown* fatoora::VatBreakdown::FromFFI(const fatoora::capi::VatBreakdown* ptr) {
    return reinterpret_cast<const fatoora::VatBreakdown*>(ptr);
}

inline fatoora::VatBreakdown* fatoora::VatBreakdown::FromFFI(fatoora::capi::VatBreakdown* ptr) {
    return reinterpret_cast<fatoora::VatBreakdown*>(ptr);
}

inline void fatoora::VatBreakdown::operator delete(void* ptr) {
    fatoora::capi::fatoora_VatBreakdown_destroy(reinterpret_cast<fatoora::capi::VatBreakdown*>(ptr));
}


#endif // fatoora_VatBreakdown_HPP

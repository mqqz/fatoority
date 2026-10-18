#ifndef fatoora_AllowanceCharge_HPP
#define fatoora_AllowanceCharge_HPP

#include "AllowanceCharge.d.hpp"

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
#include "Text.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_AllowanceCharge_new_result {union {fatoora::capi::AllowanceCharge* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_new_result;
    fatoora_AllowanceCharge_new_result fatoora_AllowanceCharge_new(bool charge, diplomat::capi::DiplomatStringView amount, diplomat::capi::OptionStringView reason, diplomat::capi::OptionStringView reason_code, uint8_t category, diplomat::capi::DiplomatStringView rate, const fatoora::capi::TaxExemption* exemption);

    bool fatoora_AllowanceCharge_is_charge(const fatoora::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_amount_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_amount_result;
    fatoora_AllowanceCharge_amount_result fatoora_AllowanceCharge_amount(const fatoora::capi::AllowanceCharge* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_AllowanceCharge_rate_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_rate_result;
    fatoora_AllowanceCharge_rate_result fatoora_AllowanceCharge_rate(const fatoora::capi::AllowanceCharge* self, diplomat::capi::DiplomatWrite* write);

    uint8_t fatoora_AllowanceCharge_category(const fatoora::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_with_tax_category_count_result {union {fatoora::capi::AllowanceCharge* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_with_tax_category_count_result;
    fatoora_AllowanceCharge_with_tax_category_count_result fatoora_AllowanceCharge_with_tax_category_count(const fatoora::capi::AllowanceCharge* self, size_t count);

    size_t fatoora_AllowanceCharge_tax_category_count(const fatoora::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_reason_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_reason_result;
    fatoora_AllowanceCharge_reason_result fatoora_AllowanceCharge_reason(const fatoora::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_reason_code_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_reason_code_result;
    fatoora_AllowanceCharge_reason_code_result fatoora_AllowanceCharge_reason_code(const fatoora::capi::AllowanceCharge* self);

    typedef struct fatoora_AllowanceCharge_exemption_result {union {fatoora::capi::TaxExemption* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_exemption_result;
    fatoora_AllowanceCharge_exemption_result fatoora_AllowanceCharge_exemption(const fatoora::capi::AllowanceCharge* self);

    void fatoora_AllowanceCharge_destroy(AllowanceCharge* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::AllowanceCharge>, std::unique_ptr<fatoora::BindingError>> fatoora::AllowanceCharge::new_(bool charge, std::string_view amount, std::optional<std::string_view> reason, std::optional<std::string_view> reason_code, uint8_t category, std::string_view rate, const fatoora::TaxExemption& exemption) {
    auto result = fatoora::capi::fatoora_AllowanceCharge_new(charge,
        {amount.data(), amount.size()},
        reason.has_value() ? (diplomat::capi::OptionStringView{ { {reason.value().data(), reason.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }),
        reason_code.has_value() ? (diplomat::capi::OptionStringView{ { {reason_code.value().data(), reason_code.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }),
        category,
        {rate.data(), rate.size()},
        exemption.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::AllowanceCharge>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::AllowanceCharge>>(std::unique_ptr<fatoora::AllowanceCharge>(fatoora::AllowanceCharge::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::AllowanceCharge>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline bool fatoora::AllowanceCharge::is_charge() const {
    auto result = fatoora::capi::fatoora_AllowanceCharge_is_charge(this->AsFFI());
    return result;
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::AllowanceCharge::amount() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_AllowanceCharge_amount(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::AllowanceCharge::amount_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_AllowanceCharge_amount(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::AllowanceCharge::rate() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_AllowanceCharge_rate(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::AllowanceCharge::rate_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_AllowanceCharge_rate(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline uint8_t fatoora::AllowanceCharge::category() const {
    auto result = fatoora::capi::fatoora_AllowanceCharge_category(this->AsFFI());
    return result;
}

inline diplomat::result<std::unique_ptr<fatoora::AllowanceCharge>, std::unique_ptr<fatoora::BindingError>> fatoora::AllowanceCharge::with_tax_category_count(size_t count) const {
    auto result = fatoora::capi::fatoora_AllowanceCharge_with_tax_category_count(this->AsFFI(),
        count);
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::AllowanceCharge>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::AllowanceCharge>>(std::unique_ptr<fatoora::AllowanceCharge>(fatoora::AllowanceCharge::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::AllowanceCharge>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline size_t fatoora::AllowanceCharge::tax_category_count() const {
    auto result = fatoora::capi::fatoora_AllowanceCharge_tax_category_count(this->AsFFI());
    return result;
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::AllowanceCharge::reason() const {
    auto result = fatoora::capi::fatoora_AllowanceCharge_reason(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::AllowanceCharge::reason_code() const {
    auto result = fatoora::capi::fatoora_AllowanceCharge_reason_code(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>> fatoora::AllowanceCharge::exemption() const {
    auto result = fatoora::capi::fatoora_AllowanceCharge_exemption(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::TaxExemption>>(std::unique_ptr<fatoora::TaxExemption>(fatoora::TaxExemption::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::AllowanceCharge* fatoora::AllowanceCharge::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::AllowanceCharge*>(this);
}

inline fatoora::capi::AllowanceCharge* fatoora::AllowanceCharge::AsFFI() {
    return reinterpret_cast<fatoora::capi::AllowanceCharge*>(this);
}

inline const fatoora::AllowanceCharge* fatoora::AllowanceCharge::FromFFI(const fatoora::capi::AllowanceCharge* ptr) {
    return reinterpret_cast<const fatoora::AllowanceCharge*>(ptr);
}

inline fatoora::AllowanceCharge* fatoora::AllowanceCharge::FromFFI(fatoora::capi::AllowanceCharge* ptr) {
    return reinterpret_cast<fatoora::AllowanceCharge*>(ptr);
}

inline void fatoora::AllowanceCharge::operator delete(void* ptr) {
    fatoora::capi::fatoora_AllowanceCharge_destroy(reinterpret_cast<fatoora::capi::AllowanceCharge*>(ptr));
}


#endif // fatoora_AllowanceCharge_HPP

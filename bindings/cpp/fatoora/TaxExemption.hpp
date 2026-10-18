#ifndef fatoora_TaxExemption_HPP
#define fatoora_TaxExemption_HPP

#include "TaxExemption.d.hpp"

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
#include "Text.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_TaxExemption_new_result {union {fatoora::capi::TaxExemption* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_TaxExemption_new_result;
    fatoora_TaxExemption_new_result fatoora_TaxExemption_new(diplomat::capi::OptionStringView code, diplomat::capi::OptionStringView reason);

    typedef struct fatoora_TaxExemption_reason_code_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_TaxExemption_reason_code_result;
    fatoora_TaxExemption_reason_code_result fatoora_TaxExemption_reason_code(const fatoora::capi::TaxExemption* self);

    typedef struct fatoora_TaxExemption_reason_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_TaxExemption_reason_result;
    fatoora_TaxExemption_reason_result fatoora_TaxExemption_reason(const fatoora::capi::TaxExemption* self);

    void fatoora_TaxExemption_destroy(TaxExemption* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>> fatoora::TaxExemption::new_(std::optional<std::string_view> code, std::optional<std::string_view> reason) {
    auto result = fatoora::capi::fatoora_TaxExemption_new(code.has_value() ? (diplomat::capi::OptionStringView{ { {code.value().data(), code.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }),
        reason.has_value() ? (diplomat::capi::OptionStringView{ { {reason.value().data(), reason.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::TaxExemption>>(std::unique_ptr<fatoora::TaxExemption>(fatoora::TaxExemption::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::TaxExemption::reason_code() const {
    auto result = fatoora::capi::fatoora_TaxExemption_reason_code(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::TaxExemption::reason() const {
    auto result = fatoora::capi::fatoora_TaxExemption_reason(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::TaxExemption* fatoora::TaxExemption::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::TaxExemption*>(this);
}

inline fatoora::capi::TaxExemption* fatoora::TaxExemption::AsFFI() {
    return reinterpret_cast<fatoora::capi::TaxExemption*>(this);
}

inline const fatoora::TaxExemption* fatoora::TaxExemption::FromFFI(const fatoora::capi::TaxExemption* ptr) {
    return reinterpret_cast<const fatoora::TaxExemption*>(ptr);
}

inline fatoora::TaxExemption* fatoora::TaxExemption::FromFFI(fatoora::capi::TaxExemption* ptr) {
    return reinterpret_cast<fatoora::TaxExemption*>(ptr);
}

inline void fatoora::TaxExemption::operator delete(void* ptr) {
    fatoora::capi::fatoora_TaxExemption_destroy(reinterpret_cast<fatoora::capi::TaxExemption*>(ptr));
}


#endif // fatoora_TaxExemption_HPP

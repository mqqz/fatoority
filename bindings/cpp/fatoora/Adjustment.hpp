#ifndef fatoora_Adjustment_HPP
#define fatoora_Adjustment_HPP

#include "Adjustment.d.hpp"

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

    typedef struct fatoora_Adjustment_amount_result {union {fatoora::capi::Adjustment* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_amount_result;
    fatoora_Adjustment_amount_result fatoora_Adjustment_amount(diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_Adjustment_percentage_result {union {fatoora::capi::Adjustment* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_percentage_result;
    fatoora_Adjustment_percentage_result fatoora_Adjustment_percentage(diplomat::capi::DiplomatStringView base, diplomat::capi::DiplomatStringView percent);

    typedef struct fatoora_Adjustment_with_supplied_amount_result {union {fatoora::capi::Adjustment* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_with_supplied_amount_result;
    fatoora_Adjustment_with_supplied_amount_result fatoora_Adjustment_with_supplied_amount(const fatoora::capi::Adjustment* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_Adjustment_with_reason_result {union {fatoora::capi::Adjustment* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_with_reason_result;
    fatoora_Adjustment_with_reason_result fatoora_Adjustment_with_reason(const fatoora::capi::Adjustment* self, diplomat::capi::OptionStringView reason, diplomat::capi::OptionStringView code);

    typedef struct fatoora_Adjustment_value_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_value_result;
    fatoora_Adjustment_value_result fatoora_Adjustment_value(const fatoora::capi::Adjustment* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_Adjustment_base_amount_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_base_amount_result;
    fatoora_Adjustment_base_amount_result fatoora_Adjustment_base_amount(const fatoora::capi::Adjustment* self);

    typedef struct fatoora_Adjustment_percent_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_percent_result;
    fatoora_Adjustment_percent_result fatoora_Adjustment_percent(const fatoora::capi::Adjustment* self);

    typedef struct fatoora_Adjustment_reason_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_reason_result;
    fatoora_Adjustment_reason_result fatoora_Adjustment_reason(const fatoora::capi::Adjustment* self);

    typedef struct fatoora_Adjustment_reason_code_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_Adjustment_reason_code_result;
    fatoora_Adjustment_reason_code_result fatoora_Adjustment_reason_code(const fatoora::capi::Adjustment* self);

    void fatoora_Adjustment_destroy(Adjustment* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>> fatoora::Adjustment::amount(std::string_view value) {
    auto result = fatoora::capi::fatoora_Adjustment_amount({value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Adjustment>>(std::unique_ptr<fatoora::Adjustment>(fatoora::Adjustment::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>> fatoora::Adjustment::percentage(std::string_view base, std::string_view percent) {
    auto result = fatoora::capi::fatoora_Adjustment_percentage({base.data(), base.size()},
        {percent.data(), percent.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Adjustment>>(std::unique_ptr<fatoora::Adjustment>(fatoora::Adjustment::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>> fatoora::Adjustment::with_supplied_amount(std::string_view value) const {
    auto result = fatoora::capi::fatoora_Adjustment_with_supplied_amount(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Adjustment>>(std::unique_ptr<fatoora::Adjustment>(fatoora::Adjustment::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>> fatoora::Adjustment::with_reason(std::optional<std::string_view> reason, std::optional<std::string_view> code) const {
    auto result = fatoora::capi::fatoora_Adjustment_with_reason(this->AsFFI(),
        reason.has_value() ? (diplomat::capi::OptionStringView{ { {reason.value().data(), reason.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }),
        code.has_value() ? (diplomat::capi::OptionStringView{ { {code.value().data(), code.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Adjustment>>(std::unique_ptr<fatoora::Adjustment>(fatoora::Adjustment::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::Adjustment::value() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_Adjustment_value(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::Adjustment::value_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_Adjustment_value(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::Adjustment::base_amount() const {
    auto result = fatoora::capi::fatoora_Adjustment_base_amount(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::Adjustment::percent() const {
    auto result = fatoora::capi::fatoora_Adjustment_percent(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::Adjustment::reason() const {
    auto result = fatoora::capi::fatoora_Adjustment_reason(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::Adjustment::reason_code() const {
    auto result = fatoora::capi::fatoora_Adjustment_reason_code(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::Adjustment* fatoora::Adjustment::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::Adjustment*>(this);
}

inline fatoora::capi::Adjustment* fatoora::Adjustment::AsFFI() {
    return reinterpret_cast<fatoora::capi::Adjustment*>(this);
}

inline const fatoora::Adjustment* fatoora::Adjustment::FromFFI(const fatoora::capi::Adjustment* ptr) {
    return reinterpret_cast<const fatoora::Adjustment*>(ptr);
}

inline fatoora::Adjustment* fatoora::Adjustment::FromFFI(fatoora::capi::Adjustment* ptr) {
    return reinterpret_cast<fatoora::Adjustment*>(ptr);
}

inline void fatoora::Adjustment::operator delete(void* ptr) {
    fatoora::capi::fatoora_Adjustment_destroy(reinterpret_cast<fatoora::capi::Adjustment*>(ptr));
}


#endif // fatoora_Adjustment_HPP

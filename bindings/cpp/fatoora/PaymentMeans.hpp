#ifndef fatoora_PaymentMeans_HPP
#define fatoora_PaymentMeans_HPP

#include "PaymentMeans.d.hpp"

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


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_PaymentMeans_with_instruction_note_result {union {fatoora::capi::PaymentMeans* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_with_instruction_note_result;
    fatoora_PaymentMeans_with_instruction_note_result fatoora_PaymentMeans_with_instruction_note(const fatoora::capi::PaymentMeans* self, diplomat::capi::DiplomatStringView value);

    size_t fatoora_PaymentMeans_instruction_notes_len(const fatoora::capi::PaymentMeans* self);

    typedef struct fatoora_PaymentMeans_instruction_note_at_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_instruction_note_at_result;
    fatoora_PaymentMeans_instruction_note_at_result fatoora_PaymentMeans_instruction_note_at(const fatoora::capi::PaymentMeans* self, size_t index, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_PaymentMeans_new_result {union {fatoora::capi::PaymentMeans* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_new_result;
    fatoora_PaymentMeans_new_result fatoora_PaymentMeans_new(diplomat::capi::DiplomatStringView code, diplomat::capi::OptionStringView instruction);

    typedef struct fatoora_PaymentMeans_with_account_result {union {fatoora::capi::PaymentMeans* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_with_account_result;
    fatoora_PaymentMeans_with_account_result fatoora_PaymentMeans_with_account(const fatoora::capi::PaymentMeans* self, diplomat::capi::OptionStringView id, diplomat::capi::OptionStringView terms);

    typedef struct fatoora_PaymentMeans_code_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_code_result;
    fatoora_PaymentMeans_code_result fatoora_PaymentMeans_code(const fatoora::capi::PaymentMeans* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_PaymentMeans_instruction_note_result { bool is_ok;} fatoora_PaymentMeans_instruction_note_result;
    fatoora_PaymentMeans_instruction_note_result fatoora_PaymentMeans_instruction_note(const fatoora::capi::PaymentMeans* self, diplomat::capi::DiplomatWrite* write);

    bool fatoora_PaymentMeans_has_account(const fatoora::capi::PaymentMeans* self);

    typedef struct fatoora_PaymentMeans_account_identifier_result { bool is_ok;} fatoora_PaymentMeans_account_identifier_result;
    fatoora_PaymentMeans_account_identifier_result fatoora_PaymentMeans_account_identifier(const fatoora::capi::PaymentMeans* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_PaymentMeans_payment_terms_result { bool is_ok;} fatoora_PaymentMeans_payment_terms_result;
    fatoora_PaymentMeans_payment_terms_result fatoora_PaymentMeans_payment_terms(const fatoora::capi::PaymentMeans* self, diplomat::capi::DiplomatWrite* write);

    void fatoora_PaymentMeans_destroy(PaymentMeans* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>> fatoora::PaymentMeans::with_instruction_note(std::string_view value) const {
    auto result = fatoora::capi::fatoora_PaymentMeans_with_instruction_note(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::PaymentMeans>>(std::unique_ptr<fatoora::PaymentMeans>(fatoora::PaymentMeans::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline size_t fatoora::PaymentMeans::instruction_notes_len() const {
    auto result = fatoora::capi::fatoora_PaymentMeans_instruction_notes_len(this->AsFFI());
    return result;
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::PaymentMeans::instruction_note_at(size_t index) const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_PaymentMeans_instruction_note_at(this->AsFFI(),
        index,
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::PaymentMeans::instruction_note_at_write(size_t index, W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_PaymentMeans_instruction_note_at(this->AsFFI(),
        index,
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>> fatoora::PaymentMeans::new_(std::string_view code, std::optional<std::string_view> instruction) {
    auto result = fatoora::capi::fatoora_PaymentMeans_new({code.data(), code.size()},
        instruction.has_value() ? (diplomat::capi::OptionStringView{ { {instruction.value().data(), instruction.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::PaymentMeans>>(std::unique_ptr<fatoora::PaymentMeans>(fatoora::PaymentMeans::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>> fatoora::PaymentMeans::with_account(std::optional<std::string_view> id, std::optional<std::string_view> terms) const {
    auto result = fatoora::capi::fatoora_PaymentMeans_with_account(this->AsFFI(),
        id.has_value() ? (diplomat::capi::OptionStringView{ { {id.value().data(), id.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }),
        terms.has_value() ? (diplomat::capi::OptionStringView{ { {terms.value().data(), terms.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::PaymentMeans>>(std::unique_ptr<fatoora::PaymentMeans>(fatoora::PaymentMeans::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::PaymentMeans::code() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_PaymentMeans_code(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::PaymentMeans::code_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_PaymentMeans_code(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline std::optional<std::string> fatoora::PaymentMeans::instruction_note() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_PaymentMeans_instruction_note(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> fatoora::PaymentMeans::instruction_note_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_PaymentMeans_instruction_note(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline bool fatoora::PaymentMeans::has_account() const {
    auto result = fatoora::capi::fatoora_PaymentMeans_has_account(this->AsFFI());
    return result;
}

inline std::optional<std::string> fatoora::PaymentMeans::account_identifier() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_PaymentMeans_account_identifier(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> fatoora::PaymentMeans::account_identifier_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_PaymentMeans_account_identifier(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> fatoora::PaymentMeans::payment_terms() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_PaymentMeans_payment_terms(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> fatoora::PaymentMeans::payment_terms_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_PaymentMeans_payment_terms(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline const fatoora::capi::PaymentMeans* fatoora::PaymentMeans::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::PaymentMeans*>(this);
}

inline fatoora::capi::PaymentMeans* fatoora::PaymentMeans::AsFFI() {
    return reinterpret_cast<fatoora::capi::PaymentMeans*>(this);
}

inline const fatoora::PaymentMeans* fatoora::PaymentMeans::FromFFI(const fatoora::capi::PaymentMeans* ptr) {
    return reinterpret_cast<const fatoora::PaymentMeans*>(ptr);
}

inline fatoora::PaymentMeans* fatoora::PaymentMeans::FromFFI(fatoora::capi::PaymentMeans* ptr) {
    return reinterpret_cast<fatoora::PaymentMeans*>(ptr);
}

inline void fatoora::PaymentMeans::operator delete(void* ptr) {
    fatoora::capi::fatoora_PaymentMeans_destroy(reinterpret_cast<fatoora::capi::PaymentMeans*>(ptr));
}


#endif // fatoora_PaymentMeans_HPP

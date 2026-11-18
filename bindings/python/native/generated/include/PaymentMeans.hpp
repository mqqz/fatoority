#ifndef _NATIVE_PaymentMeans_HPP
#define _NATIVE_PaymentMeans_HPP

#include "PaymentMeans.d.hpp"

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

    typedef struct fatoora_PaymentMeans_with_instruction_note_result {union {_native::capi::PaymentMeans* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_with_instruction_note_result;
    fatoora_PaymentMeans_with_instruction_note_result fatoora_PaymentMeans_with_instruction_note(const _native::capi::PaymentMeans* self, _native::diplomat::capi::DiplomatStringView value);

    size_t fatoora_PaymentMeans_instruction_notes_len(const _native::capi::PaymentMeans* self);

    typedef struct fatoora_PaymentMeans_instruction_note_at_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_instruction_note_at_result;
    fatoora_PaymentMeans_instruction_note_at_result fatoora_PaymentMeans_instruction_note_at(const _native::capi::PaymentMeans* self, size_t index, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_PaymentMeans_new_result {union {_native::capi::PaymentMeans* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_new_result;
    fatoora_PaymentMeans_new_result fatoora_PaymentMeans_new(_native::diplomat::capi::DiplomatStringView code, _native::diplomat::capi::OptionStringView instruction);

    typedef struct fatoora_PaymentMeans_with_account_result {union {_native::capi::PaymentMeans* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_with_account_result;
    fatoora_PaymentMeans_with_account_result fatoora_PaymentMeans_with_account(const _native::capi::PaymentMeans* self, _native::diplomat::capi::OptionStringView id, _native::diplomat::capi::OptionStringView terms);

    typedef struct fatoora_PaymentMeans_code_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_PaymentMeans_code_result;
    fatoora_PaymentMeans_code_result fatoora_PaymentMeans_code(const _native::capi::PaymentMeans* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_PaymentMeans_instruction_note_result { bool is_ok;} fatoora_PaymentMeans_instruction_note_result;
    fatoora_PaymentMeans_instruction_note_result fatoora_PaymentMeans_instruction_note(const _native::capi::PaymentMeans* self, _native::diplomat::capi::DiplomatWrite* write);

    bool fatoora_PaymentMeans_has_account(const _native::capi::PaymentMeans* self);

    typedef struct fatoora_PaymentMeans_account_identifier_result { bool is_ok;} fatoora_PaymentMeans_account_identifier_result;
    fatoora_PaymentMeans_account_identifier_result fatoora_PaymentMeans_account_identifier(const _native::capi::PaymentMeans* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_PaymentMeans_payment_terms_result { bool is_ok;} fatoora_PaymentMeans_payment_terms_result;
    fatoora_PaymentMeans_payment_terms_result fatoora_PaymentMeans_payment_terms(const _native::capi::PaymentMeans* self, _native::diplomat::capi::DiplomatWrite* write);

    void fatoora_PaymentMeans_destroy(PaymentMeans* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>> _native::PaymentMeans::with_instruction_note(std::string_view value) const {
    auto result = _native::capi::fatoora_PaymentMeans_with_instruction_note(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::PaymentMeans>>(std::unique_ptr<_native::PaymentMeans>(_native::PaymentMeans::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline size_t _native::PaymentMeans::instruction_notes_len() const {
    auto result = _native::capi::fatoora_PaymentMeans_instruction_notes_len(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::PaymentMeans::instruction_note_at(size_t index) const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_PaymentMeans_instruction_note_at(this->AsFFI(),
        index,
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::PaymentMeans::instruction_note_at_write(size_t index, W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_PaymentMeans_instruction_note_at(this->AsFFI(),
        index,
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>> _native::PaymentMeans::new_(std::string_view code, std::optional<std::string_view> instruction) {
    auto result = _native::capi::fatoora_PaymentMeans_new({code.data(), code.size()},
        instruction.has_value() ? (_native::diplomat::capi::OptionStringView{ { {instruction.value().data(), instruction.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::PaymentMeans>>(std::unique_ptr<_native::PaymentMeans>(_native::PaymentMeans::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>> _native::PaymentMeans::with_account(std::optional<std::string_view> id, std::optional<std::string_view> terms) const {
    auto result = _native::capi::fatoora_PaymentMeans_with_account(this->AsFFI(),
        id.has_value() ? (_native::diplomat::capi::OptionStringView{ { {id.value().data(), id.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }),
        terms.has_value() ? (_native::diplomat::capi::OptionStringView{ { {terms.value().data(), terms.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::PaymentMeans>>(std::unique_ptr<_native::PaymentMeans>(_native::PaymentMeans::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::PaymentMeans::code() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_PaymentMeans_code(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::PaymentMeans::code_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_PaymentMeans_code(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline std::optional<std::string> _native::PaymentMeans::instruction_note() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_PaymentMeans_instruction_note(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::PaymentMeans::instruction_note_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_PaymentMeans_instruction_note(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline bool _native::PaymentMeans::has_account() const {
    auto result = _native::capi::fatoora_PaymentMeans_has_account(this->AsFFI());
    return result;
}

inline std::optional<std::string> _native::PaymentMeans::account_identifier() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_PaymentMeans_account_identifier(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::PaymentMeans::account_identifier_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_PaymentMeans_account_identifier(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> _native::PaymentMeans::payment_terms() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_PaymentMeans_payment_terms(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::PaymentMeans::payment_terms_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_PaymentMeans_payment_terms(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline const _native::capi::PaymentMeans* _native::PaymentMeans::AsFFI() const {
    return reinterpret_cast<const _native::capi::PaymentMeans*>(this);
}

inline _native::capi::PaymentMeans* _native::PaymentMeans::AsFFI() {
    return reinterpret_cast<_native::capi::PaymentMeans*>(this);
}

inline const _native::PaymentMeans* _native::PaymentMeans::FromFFI(const _native::capi::PaymentMeans* ptr) {
    return reinterpret_cast<const _native::PaymentMeans*>(ptr);
}

inline _native::PaymentMeans* _native::PaymentMeans::FromFFI(_native::capi::PaymentMeans* ptr) {
    return reinterpret_cast<_native::PaymentMeans*>(ptr);
}

inline void _native::PaymentMeans::operator delete(void* ptr) {
    _native::capi::fatoora_PaymentMeans_destroy(reinterpret_cast<_native::capi::PaymentMeans*>(ptr));
}


#endif // _NATIVE_PaymentMeans_HPP

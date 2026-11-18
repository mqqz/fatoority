#ifndef _NATIVE_ValidationMessage_HPP
#define _NATIVE_ValidationMessage_HPP

#include "ValidationMessage.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "MessageType.hpp"
#include "ValidationStatus.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_ValidationMessage_message_type_result {union {_native::capi::MessageType ok; }; bool is_ok;} fatoora_ValidationMessage_message_type_result;
    fatoora_ValidationMessage_message_type_result fatoora_ValidationMessage_message_type(const _native::capi::ValidationMessage* self);

    typedef struct fatoora_ValidationMessage_message_type_raw_result { bool is_ok;} fatoora_ValidationMessage_message_type_raw_result;
    fatoora_ValidationMessage_message_type_raw_result fatoora_ValidationMessage_message_type_raw(const _native::capi::ValidationMessage* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationMessage_code_result { bool is_ok;} fatoora_ValidationMessage_code_result;
    fatoora_ValidationMessage_code_result fatoora_ValidationMessage_code(const _native::capi::ValidationMessage* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationMessage_category_result { bool is_ok;} fatoora_ValidationMessage_category_result;
    fatoora_ValidationMessage_category_result fatoora_ValidationMessage_category(const _native::capi::ValidationMessage* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationMessage_message_result { bool is_ok;} fatoora_ValidationMessage_message_result;
    fatoora_ValidationMessage_message_result fatoora_ValidationMessage_message(const _native::capi::ValidationMessage* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationMessage_status_result {union {_native::capi::ValidationStatus ok; }; bool is_ok;} fatoora_ValidationMessage_status_result;
    fatoora_ValidationMessage_status_result fatoora_ValidationMessage_status(const _native::capi::ValidationMessage* self);

    typedef struct fatoora_ValidationMessage_status_raw_result { bool is_ok;} fatoora_ValidationMessage_status_raw_result;
    fatoora_ValidationMessage_status_raw_result fatoora_ValidationMessage_status_raw(const _native::capi::ValidationMessage* self, _native::diplomat::capi::DiplomatWrite* write);

    void fatoora_ValidationMessage_destroy(ValidationMessage* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::optional<_native::MessageType> _native::ValidationMessage::message_type() const {
    auto result = _native::capi::fatoora_ValidationMessage_message_type(this->AsFFI());
    return result.is_ok ? std::optional<_native::MessageType>(_native::MessageType::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<std::string> _native::ValidationMessage::message_type_raw() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ValidationMessage_message_type_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ValidationMessage::message_type_raw_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ValidationMessage_message_type_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> _native::ValidationMessage::code() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ValidationMessage_code(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ValidationMessage::code_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ValidationMessage_code(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> _native::ValidationMessage::category() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ValidationMessage_category(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ValidationMessage::category_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ValidationMessage_category(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> _native::ValidationMessage::message() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ValidationMessage_message(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ValidationMessage::message_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ValidationMessage_message(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<_native::ValidationStatus> _native::ValidationMessage::status() const {
    auto result = _native::capi::fatoora_ValidationMessage_status(this->AsFFI());
    return result.is_ok ? std::optional<_native::ValidationStatus>(_native::ValidationStatus::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<std::string> _native::ValidationMessage::status_raw() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_ValidationMessage_status_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> _native::ValidationMessage::status_raw_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_ValidationMessage_status_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline const _native::capi::ValidationMessage* _native::ValidationMessage::AsFFI() const {
    return reinterpret_cast<const _native::capi::ValidationMessage*>(this);
}

inline _native::capi::ValidationMessage* _native::ValidationMessage::AsFFI() {
    return reinterpret_cast<_native::capi::ValidationMessage*>(this);
}

inline const _native::ValidationMessage* _native::ValidationMessage::FromFFI(const _native::capi::ValidationMessage* ptr) {
    return reinterpret_cast<const _native::ValidationMessage*>(ptr);
}

inline _native::ValidationMessage* _native::ValidationMessage::FromFFI(_native::capi::ValidationMessage* ptr) {
    return reinterpret_cast<_native::ValidationMessage*>(ptr);
}

inline void _native::ValidationMessage::operator delete(void* ptr) {
    _native::capi::fatoora_ValidationMessage_destroy(reinterpret_cast<_native::capi::ValidationMessage*>(ptr));
}


#endif // _NATIVE_ValidationMessage_HPP

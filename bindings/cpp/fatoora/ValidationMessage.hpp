#ifndef fatoora_ValidationMessage_HPP
#define fatoora_ValidationMessage_HPP

#include "ValidationMessage.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "MessageType.hpp"
#include "ValidationStatus.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_ValidationMessage_message_type_result {union {fatoora::capi::MessageType ok; }; bool is_ok;} fatoora_ValidationMessage_message_type_result;
    fatoora_ValidationMessage_message_type_result fatoora_ValidationMessage_message_type(const fatoora::capi::ValidationMessage* self);

    typedef struct fatoora_ValidationMessage_message_type_raw_result { bool is_ok;} fatoora_ValidationMessage_message_type_raw_result;
    fatoora_ValidationMessage_message_type_raw_result fatoora_ValidationMessage_message_type_raw(const fatoora::capi::ValidationMessage* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationMessage_code_result { bool is_ok;} fatoora_ValidationMessage_code_result;
    fatoora_ValidationMessage_code_result fatoora_ValidationMessage_code(const fatoora::capi::ValidationMessage* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationMessage_category_result { bool is_ok;} fatoora_ValidationMessage_category_result;
    fatoora_ValidationMessage_category_result fatoora_ValidationMessage_category(const fatoora::capi::ValidationMessage* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationMessage_message_result { bool is_ok;} fatoora_ValidationMessage_message_result;
    fatoora_ValidationMessage_message_result fatoora_ValidationMessage_message(const fatoora::capi::ValidationMessage* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ValidationMessage_status_result {union {fatoora::capi::ValidationStatus ok; }; bool is_ok;} fatoora_ValidationMessage_status_result;
    fatoora_ValidationMessage_status_result fatoora_ValidationMessage_status(const fatoora::capi::ValidationMessage* self);

    typedef struct fatoora_ValidationMessage_status_raw_result { bool is_ok;} fatoora_ValidationMessage_status_raw_result;
    fatoora_ValidationMessage_status_raw_result fatoora_ValidationMessage_status_raw(const fatoora::capi::ValidationMessage* self, diplomat::capi::DiplomatWrite* write);

    void fatoora_ValidationMessage_destroy(ValidationMessage* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::optional<fatoora::MessageType> fatoora::ValidationMessage::message_type() const {
    auto result = fatoora::capi::fatoora_ValidationMessage_message_type(this->AsFFI());
    return result.is_ok ? std::optional<fatoora::MessageType>(fatoora::MessageType::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<std::string> fatoora::ValidationMessage::message_type_raw() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ValidationMessage_message_type_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> fatoora::ValidationMessage::message_type_raw_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ValidationMessage_message_type_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> fatoora::ValidationMessage::code() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ValidationMessage_code(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> fatoora::ValidationMessage::code_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ValidationMessage_code(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> fatoora::ValidationMessage::category() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ValidationMessage_category(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> fatoora::ValidationMessage::category_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ValidationMessage_category(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<std::string> fatoora::ValidationMessage::message() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ValidationMessage_message(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> fatoora::ValidationMessage::message_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ValidationMessage_message(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline std::optional<fatoora::ValidationStatus> fatoora::ValidationMessage::status() const {
    auto result = fatoora::capi::fatoora_ValidationMessage_status(this->AsFFI());
    return result.is_ok ? std::optional<fatoora::ValidationStatus>(fatoora::ValidationStatus::FromFFI(result.ok)) : std::nullopt;
}

inline std::optional<std::string> fatoora::ValidationMessage::status_raw() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ValidationMessage_status_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> fatoora::ValidationMessage::status_raw_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ValidationMessage_status_raw(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline const fatoora::capi::ValidationMessage* fatoora::ValidationMessage::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::ValidationMessage*>(this);
}

inline fatoora::capi::ValidationMessage* fatoora::ValidationMessage::AsFFI() {
    return reinterpret_cast<fatoora::capi::ValidationMessage*>(this);
}

inline const fatoora::ValidationMessage* fatoora::ValidationMessage::FromFFI(const fatoora::capi::ValidationMessage* ptr) {
    return reinterpret_cast<const fatoora::ValidationMessage*>(ptr);
}

inline fatoora::ValidationMessage* fatoora::ValidationMessage::FromFFI(fatoora::capi::ValidationMessage* ptr) {
    return reinterpret_cast<fatoora::ValidationMessage*>(ptr);
}

inline void fatoora::ValidationMessage::operator delete(void* ptr) {
    fatoora::capi::fatoora_ValidationMessage_destroy(reinterpret_cast<fatoora::capi::ValidationMessage*>(ptr));
}


#endif // fatoora_ValidationMessage_HPP

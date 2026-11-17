#ifndef fatoora_ZatcaFinding_HPP
#define fatoora_ZatcaFinding_HPP

#include "ZatcaFinding.d.hpp"

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
#include "Severity.hpp"
#include "Text.hpp"
#include "ValidationLayer.hpp"
#include "ValidationLocationKind.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_ZatcaFinding_assertion_site_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaFinding_assertion_site_result;
    fatoora_ZatcaFinding_assertion_site_result fatoora_ZatcaFinding_assertion_site(const fatoora::capi::ZatcaFinding* self);

    fatoora::capi::ValidationLayer fatoora_ZatcaFinding_layer(const fatoora::capi::ZatcaFinding* self);

    typedef struct fatoora_ZatcaFinding_code_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaFinding_code_result;
    fatoora_ZatcaFinding_code_result fatoora_ZatcaFinding_code(const fatoora::capi::ZatcaFinding* self, diplomat::capi::DiplomatWrite* write);

    fatoora::capi::Severity fatoora_ZatcaFinding_severity(const fatoora::capi::ZatcaFinding* self);

    typedef struct fatoora_ZatcaFinding_message_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaFinding_message_result;
    fatoora_ZatcaFinding_message_result fatoora_ZatcaFinding_message(const fatoora::capi::ZatcaFinding* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ZatcaFinding_location_kind_result {union {fatoora::capi::ValidationLocationKind ok; }; bool is_ok;} fatoora_ZatcaFinding_location_kind_result;
    fatoora_ZatcaFinding_location_kind_result fatoora_ZatcaFinding_location_kind(const fatoora::capi::ZatcaFinding* self);

    typedef struct fatoora_ZatcaFinding_location_path_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaFinding_location_path_result;
    fatoora_ZatcaFinding_location_path_result fatoora_ZatcaFinding_location_path(const fatoora::capi::ZatcaFinding* self);

    typedef struct fatoora_ZatcaFinding_location_line_result {union {uint32_t ok; }; bool is_ok;} fatoora_ZatcaFinding_location_line_result;
    fatoora_ZatcaFinding_location_line_result fatoora_ZatcaFinding_location_line(const fatoora::capi::ZatcaFinding* self);

    typedef struct fatoora_ZatcaFinding_location_column_result {union {uint32_t ok; }; bool is_ok;} fatoora_ZatcaFinding_location_column_result;
    fatoora_ZatcaFinding_location_column_result fatoora_ZatcaFinding_location_column(const fatoora::capi::ZatcaFinding* self);

    void fatoora_ZatcaFinding_destroy(ZatcaFinding* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaFinding::assertion_site() const {
    auto result = fatoora::capi::fatoora_ZatcaFinding_assertion_site(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline fatoora::ValidationLayer fatoora::ZatcaFinding::layer() const {
    auto result = fatoora::capi::fatoora_ZatcaFinding_layer(this->AsFFI());
    return fatoora::ValidationLayer::FromFFI(result);
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaFinding::code() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ZatcaFinding_code(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaFinding::code_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ZatcaFinding_code(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline fatoora::Severity fatoora::ZatcaFinding::severity() const {
    auto result = fatoora::capi::fatoora_ZatcaFinding_severity(this->AsFFI());
    return fatoora::Severity::FromFFI(result);
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaFinding::message() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ZatcaFinding_message(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaFinding::message_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ZatcaFinding_message(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline std::optional<fatoora::ValidationLocationKind> fatoora::ZatcaFinding::location_kind() const {
    auto result = fatoora::capi::fatoora_ZatcaFinding_location_kind(this->AsFFI());
    return result.is_ok ? std::optional<fatoora::ValidationLocationKind>(fatoora::ValidationLocationKind::FromFFI(result.ok)) : std::nullopt;
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaFinding::location_path() const {
    auto result = fatoora::capi::fatoora_ZatcaFinding_location_path(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline std::optional<uint32_t> fatoora::ZatcaFinding::location_line() const {
    auto result = fatoora::capi::fatoora_ZatcaFinding_location_line(this->AsFFI());
    return result.is_ok ? std::optional<uint32_t>(result.ok) : std::nullopt;
}

inline std::optional<uint32_t> fatoora::ZatcaFinding::location_column() const {
    auto result = fatoora::capi::fatoora_ZatcaFinding_location_column(this->AsFFI());
    return result.is_ok ? std::optional<uint32_t>(result.ok) : std::nullopt;
}

inline const fatoora::capi::ZatcaFinding* fatoora::ZatcaFinding::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::ZatcaFinding*>(this);
}

inline fatoora::capi::ZatcaFinding* fatoora::ZatcaFinding::AsFFI() {
    return reinterpret_cast<fatoora::capi::ZatcaFinding*>(this);
}

inline const fatoora::ZatcaFinding* fatoora::ZatcaFinding::FromFFI(const fatoora::capi::ZatcaFinding* ptr) {
    return reinterpret_cast<const fatoora::ZatcaFinding*>(ptr);
}

inline fatoora::ZatcaFinding* fatoora::ZatcaFinding::FromFFI(fatoora::capi::ZatcaFinding* ptr) {
    return reinterpret_cast<fatoora::ZatcaFinding*>(ptr);
}

inline void fatoora::ZatcaFinding::operator delete(void* ptr) {
    fatoora::capi::fatoora_ZatcaFinding_destroy(reinterpret_cast<fatoora::capi::ZatcaFinding*>(ptr));
}


#endif // fatoora_ZatcaFinding_HPP

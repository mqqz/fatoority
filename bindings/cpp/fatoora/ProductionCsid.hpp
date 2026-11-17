#ifndef fatoora_ProductionCsid_HPP
#define fatoora_ProductionCsid_HPP

#include "ProductionCsid.d.hpp"

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
#include "Environment.hpp"
#include "Text.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_ProductionCsid_create_result {union {fatoora::capi::ProductionCsid* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_create_result;
    fatoora_ProductionCsid_create_result fatoora_ProductionCsid_create(fatoora::capi::Environment environment, diplomat::capi::OptionStringView request_id, diplomat::capi::DiplomatStringView token, diplomat::capi::DiplomatStringView secret);

    typedef struct fatoora_ProductionCsid_env_result {union {fatoora::capi::Environment ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_env_result;
    fatoora_ProductionCsid_env_result fatoora_ProductionCsid_env(const fatoora::capi::ProductionCsid* self);

    typedef struct fatoora_ProductionCsid_request_id_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_request_id_result;
    fatoora_ProductionCsid_request_id_result fatoora_ProductionCsid_request_id(const fatoora::capi::ProductionCsid* self);

    typedef struct fatoora_ProductionCsid_binary_security_token_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_binary_security_token_result;
    fatoora_ProductionCsid_binary_security_token_result fatoora_ProductionCsid_binary_security_token(const fatoora::capi::ProductionCsid* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ProductionCsid_secret_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_secret_result;
    fatoora_ProductionCsid_secret_result fatoora_ProductionCsid_secret(const fatoora::capi::ProductionCsid* self, diplomat::capi::DiplomatWrite* write);

    void fatoora_ProductionCsid_destroy(ProductionCsid* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionCsid::create(fatoora::Environment environment, std::optional<std::string_view> request_id, std::string_view token, std::string_view secret) {
    auto result = fatoora::capi::fatoora_ProductionCsid_create(environment.AsFFI(),
        request_id.has_value() ? (diplomat::capi::OptionStringView{ { {request_id.value().data(), request_id.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }),
        {token.data(), token.size()},
        {secret.data(), secret.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ProductionCsid>>(std::unique_ptr<fatoora::ProductionCsid>(fatoora::ProductionCsid::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<fatoora::Environment, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionCsid::env() const {
    auto result = fatoora::capi::fatoora_ProductionCsid_env(this->AsFFI());
    return result.is_ok ? diplomat::result<fatoora::Environment, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<fatoora::Environment>(fatoora::Environment::FromFFI(result.ok))) : diplomat::result<fatoora::Environment, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionCsid::request_id() const {
    auto result = fatoora::capi::fatoora_ProductionCsid_request_id(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionCsid::binary_security_token() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ProductionCsid_binary_security_token(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionCsid::binary_security_token_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ProductionCsid_binary_security_token(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionCsid::secret() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ProductionCsid_secret(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ProductionCsid::secret_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ProductionCsid_secret(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::ProductionCsid* fatoora::ProductionCsid::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::ProductionCsid*>(this);
}

inline fatoora::capi::ProductionCsid* fatoora::ProductionCsid::AsFFI() {
    return reinterpret_cast<fatoora::capi::ProductionCsid*>(this);
}

inline const fatoora::ProductionCsid* fatoora::ProductionCsid::FromFFI(const fatoora::capi::ProductionCsid* ptr) {
    return reinterpret_cast<const fatoora::ProductionCsid*>(ptr);
}

inline fatoora::ProductionCsid* fatoora::ProductionCsid::FromFFI(fatoora::capi::ProductionCsid* ptr) {
    return reinterpret_cast<fatoora::ProductionCsid*>(ptr);
}

inline void fatoora::ProductionCsid::operator delete(void* ptr) {
    fatoora::capi::fatoora_ProductionCsid_destroy(reinterpret_cast<fatoora::capi::ProductionCsid*>(ptr));
}


#endif // fatoora_ProductionCsid_HPP

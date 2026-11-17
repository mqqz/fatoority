#ifndef fatoora_ComplianceCsid_HPP
#define fatoora_ComplianceCsid_HPP

#include "ComplianceCsid.d.hpp"

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

    typedef struct fatoora_ComplianceCsid_create_result {union {fatoora::capi::ComplianceCsid* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_create_result;
    fatoora_ComplianceCsid_create_result fatoora_ComplianceCsid_create(fatoora::capi::Environment environment, diplomat::capi::OptionStringView request_id, diplomat::capi::DiplomatStringView token, diplomat::capi::DiplomatStringView secret);

    typedef struct fatoora_ComplianceCsid_env_result {union {fatoora::capi::Environment ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_env_result;
    fatoora_ComplianceCsid_env_result fatoora_ComplianceCsid_env(const fatoora::capi::ComplianceCsid* self);

    typedef struct fatoora_ComplianceCsid_request_id_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_request_id_result;
    fatoora_ComplianceCsid_request_id_result fatoora_ComplianceCsid_request_id(const fatoora::capi::ComplianceCsid* self);

    typedef struct fatoora_ComplianceCsid_binary_security_token_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_binary_security_token_result;
    fatoora_ComplianceCsid_binary_security_token_result fatoora_ComplianceCsid_binary_security_token(const fatoora::capi::ComplianceCsid* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ComplianceCsid_secret_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_secret_result;
    fatoora_ComplianceCsid_secret_result fatoora_ComplianceCsid_secret(const fatoora::capi::ComplianceCsid* self, diplomat::capi::DiplomatWrite* write);

    void fatoora_ComplianceCsid_destroy(ComplianceCsid* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceCsid::create(fatoora::Environment environment, std::optional<std::string_view> request_id, std::string_view token, std::string_view secret) {
    auto result = fatoora::capi::fatoora_ComplianceCsid_create(environment.AsFFI(),
        request_id.has_value() ? (diplomat::capi::OptionStringView{ { {request_id.value().data(), request_id.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }),
        {token.data(), token.size()},
        {secret.data(), secret.size()});
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::ComplianceCsid>>(std::unique_ptr<fatoora::ComplianceCsid>(fatoora::ComplianceCsid::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<fatoora::Environment, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceCsid::env() const {
    auto result = fatoora::capi::fatoora_ComplianceCsid_env(this->AsFFI());
    return result.is_ok ? diplomat::result<fatoora::Environment, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<fatoora::Environment>(fatoora::Environment::FromFFI(result.ok))) : diplomat::result<fatoora::Environment, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceCsid::request_id() const {
    auto result = fatoora::capi::fatoora_ComplianceCsid_request_id(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceCsid::binary_security_token() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ComplianceCsid_binary_security_token(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceCsid::binary_security_token_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ComplianceCsid_binary_security_token(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceCsid::secret() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ComplianceCsid_secret(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ComplianceCsid::secret_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ComplianceCsid_secret(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::ComplianceCsid* fatoora::ComplianceCsid::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::ComplianceCsid*>(this);
}

inline fatoora::capi::ComplianceCsid* fatoora::ComplianceCsid::AsFFI() {
    return reinterpret_cast<fatoora::capi::ComplianceCsid*>(this);
}

inline const fatoora::ComplianceCsid* fatoora::ComplianceCsid::FromFFI(const fatoora::capi::ComplianceCsid* ptr) {
    return reinterpret_cast<const fatoora::ComplianceCsid*>(ptr);
}

inline fatoora::ComplianceCsid* fatoora::ComplianceCsid::FromFFI(fatoora::capi::ComplianceCsid* ptr) {
    return reinterpret_cast<fatoora::ComplianceCsid*>(ptr);
}

inline void fatoora::ComplianceCsid::operator delete(void* ptr) {
    fatoora::capi::fatoora_ComplianceCsid_destroy(reinterpret_cast<fatoora::capi::ComplianceCsid*>(ptr));
}


#endif // fatoora_ComplianceCsid_HPP

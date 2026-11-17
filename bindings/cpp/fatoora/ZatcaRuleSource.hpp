#ifndef fatoora_ZatcaRuleSource_HPP
#define fatoora_ZatcaRuleSource_HPP

#include "ZatcaRuleSource.d.hpp"

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

    typedef struct fatoora_ZatcaRuleSource_stylesheet_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaRuleSource_stylesheet_result;
    fatoora_ZatcaRuleSource_stylesheet_result fatoora_ZatcaRuleSource_stylesheet(const fatoora::capi::ZatcaRuleSource* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_ZatcaRuleSource_sha256_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_ZatcaRuleSource_sha256_result;
    fatoora_ZatcaRuleSource_sha256_result fatoora_ZatcaRuleSource_sha256(const fatoora::capi::ZatcaRuleSource* self, diplomat::capi::DiplomatWrite* write);

    void fatoora_ZatcaRuleSource_destroy(ZatcaRuleSource* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaRuleSource::stylesheet() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ZatcaRuleSource_stylesheet(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaRuleSource::stylesheet_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ZatcaRuleSource_stylesheet(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaRuleSource::sha256() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_ZatcaRuleSource_sha256(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::ZatcaRuleSource::sha256_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_ZatcaRuleSource_sha256(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::ZatcaRuleSource* fatoora::ZatcaRuleSource::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::ZatcaRuleSource*>(this);
}

inline fatoora::capi::ZatcaRuleSource* fatoora::ZatcaRuleSource::AsFFI() {
    return reinterpret_cast<fatoora::capi::ZatcaRuleSource*>(this);
}

inline const fatoora::ZatcaRuleSource* fatoora::ZatcaRuleSource::FromFFI(const fatoora::capi::ZatcaRuleSource* ptr) {
    return reinterpret_cast<const fatoora::ZatcaRuleSource*>(ptr);
}

inline fatoora::ZatcaRuleSource* fatoora::ZatcaRuleSource::FromFFI(fatoora::capi::ZatcaRuleSource* ptr) {
    return reinterpret_cast<fatoora::ZatcaRuleSource*>(ptr);
}

inline void fatoora::ZatcaRuleSource::operator delete(void* ptr) {
    fatoora::capi::fatoora_ZatcaRuleSource_destroy(reinterpret_cast<fatoora::capi::ZatcaRuleSource*>(ptr));
}


#endif // fatoora_ZatcaRuleSource_HPP

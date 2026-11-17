#ifndef fatoora_Config_HPP
#define fatoora_Config_HPP

#include "Config.d.hpp"

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


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_Config_new_result {union {fatoora::capi::Config* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_Config_new_result;
    fatoora_Config_new_result fatoora_Config_new(fatoora::capi::Environment env);

    typedef struct fatoora_Config_env_result {union {fatoora::capi::Environment ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_Config_env_result;
    fatoora_Config_env_result fatoora_Config_env(const fatoora::capi::Config* self);

    void fatoora_Config_destroy(Config* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::Config>, std::unique_ptr<fatoora::BindingError>> fatoora::Config::new_(fatoora::Environment env) {
    auto result = fatoora::capi::fatoora_Config_new(env.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Config>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Config>>(std::unique_ptr<fatoora::Config>(fatoora::Config::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Config>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<fatoora::Environment, std::unique_ptr<fatoora::BindingError>> fatoora::Config::env() const {
    auto result = fatoora::capi::fatoora_Config_env(this->AsFFI());
    return result.is_ok ? diplomat::result<fatoora::Environment, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<fatoora::Environment>(fatoora::Environment::FromFFI(result.ok))) : diplomat::result<fatoora::Environment, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::Config* fatoora::Config::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::Config*>(this);
}

inline fatoora::capi::Config* fatoora::Config::AsFFI() {
    return reinterpret_cast<fatoora::capi::Config*>(this);
}

inline const fatoora::Config* fatoora::Config::FromFFI(const fatoora::capi::Config* ptr) {
    return reinterpret_cast<const fatoora::Config*>(ptr);
}

inline fatoora::Config* fatoora::Config::FromFFI(fatoora::capi::Config* ptr) {
    return reinterpret_cast<fatoora::Config*>(ptr);
}

inline void fatoora::Config::operator delete(void* ptr) {
    fatoora::capi::fatoora_Config_destroy(reinterpret_cast<fatoora::capi::Config*>(ptr));
}


#endif // fatoora_Config_HPP

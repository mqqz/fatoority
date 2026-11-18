#ifndef fatoora_OtherId_HPP
#define fatoora_OtherId_HPP

#include "OtherId.d.hpp"

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

    typedef struct fatoora_OtherId_value_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_OtherId_value_result;
    fatoora_OtherId_value_result fatoora_OtherId_value(const fatoora::capi::OtherId* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_OtherId_scheme_result { bool is_ok;} fatoora_OtherId_scheme_result;
    fatoora_OtherId_scheme_result fatoora_OtherId_scheme(const fatoora::capi::OtherId* self, diplomat::capi::DiplomatWrite* write);

    void fatoora_OtherId_destroy(OtherId* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::OtherId::value() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_OtherId_value(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::OtherId::value_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_OtherId_value(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline std::optional<std::string> fatoora::OtherId::scheme() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_OtherId_scheme(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}
template<typename W>
inline std::optional<std::monostate> fatoora::OtherId::scheme_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_OtherId_scheme(this->AsFFI(),
        &write);
    return result.is_ok ? std::optional<std::monostate>(std::monostate()) : std::nullopt;
}

inline const fatoora::capi::OtherId* fatoora::OtherId::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::OtherId*>(this);
}

inline fatoora::capi::OtherId* fatoora::OtherId::AsFFI() {
    return reinterpret_cast<fatoora::capi::OtherId*>(this);
}

inline const fatoora::OtherId* fatoora::OtherId::FromFFI(const fatoora::capi::OtherId* ptr) {
    return reinterpret_cast<const fatoora::OtherId*>(ptr);
}

inline fatoora::OtherId* fatoora::OtherId::FromFFI(fatoora::capi::OtherId* ptr) {
    return reinterpret_cast<fatoora::OtherId*>(ptr);
}

inline void fatoora::OtherId::operator delete(void* ptr) {
    fatoora::capi::fatoora_OtherId_destroy(reinterpret_cast<fatoora::capi::OtherId*>(ptr));
}


#endif // fatoora_OtherId_HPP

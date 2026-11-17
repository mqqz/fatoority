#ifndef fatoora_ValidationStatus_HPP
#define fatoora_ValidationStatus_HPP

#include "ValidationStatus.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace fatoora {
namespace capi {

} // namespace capi
} // namespace

inline fatoora::capi::ValidationStatus fatoora::ValidationStatus::AsFFI() const {
    return static_cast<fatoora::capi::ValidationStatus>(value);
}

inline fatoora::ValidationStatus fatoora::ValidationStatus::FromFFI(fatoora::capi::ValidationStatus c_enum) {
    switch (c_enum) {
        case fatoora::capi::ValidationStatus_Unknown:
        case fatoora::capi::ValidationStatus_Pass:
        case fatoora::capi::ValidationStatus_Warning:
        case fatoora::capi::ValidationStatus_Error:
            return static_cast<fatoora::ValidationStatus::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_ValidationStatus_HPP

#ifndef _NATIVE_ValidationStatus_HPP
#define _NATIVE_ValidationStatus_HPP

#include "ValidationStatus.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {

} // namespace capi
} // namespace

inline _native::capi::ValidationStatus _native::ValidationStatus::AsFFI() const {
    return static_cast<_native::capi::ValidationStatus>(value);
}

inline _native::ValidationStatus _native::ValidationStatus::FromFFI(_native::capi::ValidationStatus c_enum) {
    switch (c_enum) {
        case _native::capi::ValidationStatus_Unknown:
        case _native::capi::ValidationStatus_Pass:
        case _native::capi::ValidationStatus_Warning:
        case _native::capi::ValidationStatus_Error:
            return static_cast<_native::ValidationStatus::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_ValidationStatus_HPP

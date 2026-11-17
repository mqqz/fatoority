#ifndef _NATIVE_ClearanceStatus_HPP
#define _NATIVE_ClearanceStatus_HPP

#include "ClearanceStatus.d.hpp"

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

inline _native::capi::ClearanceStatus _native::ClearanceStatus::AsFFI() const {
    return static_cast<_native::capi::ClearanceStatus>(value);
}

inline _native::ClearanceStatus _native::ClearanceStatus::FromFFI(_native::capi::ClearanceStatus c_enum) {
    switch (c_enum) {
        case _native::capi::ClearanceStatus_Unknown:
        case _native::capi::ClearanceStatus_Cleared:
        case _native::capi::ClearanceStatus_NotCleared:
            return static_cast<_native::ClearanceStatus::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_ClearanceStatus_HPP

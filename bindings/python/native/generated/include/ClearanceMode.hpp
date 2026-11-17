#ifndef _NATIVE_ClearanceMode_HPP
#define _NATIVE_ClearanceMode_HPP

#include "ClearanceMode.d.hpp"

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

inline _native::capi::ClearanceMode _native::ClearanceMode::AsFFI() const {
    return static_cast<_native::capi::ClearanceMode>(value);
}

inline _native::ClearanceMode _native::ClearanceMode::FromFFI(_native::capi::ClearanceMode c_enum) {
    switch (c_enum) {
        case _native::capi::ClearanceMode_Disabled:
        case _native::capi::ClearanceMode_Enabled:
            return static_cast<_native::ClearanceMode::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_ClearanceMode_HPP

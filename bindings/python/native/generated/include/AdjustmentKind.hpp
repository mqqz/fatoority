#ifndef _NATIVE_AdjustmentKind_HPP
#define _NATIVE_AdjustmentKind_HPP

#include "AdjustmentKind.d.hpp"

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

inline _native::capi::AdjustmentKind _native::AdjustmentKind::AsFFI() const {
    return static_cast<_native::capi::AdjustmentKind>(value);
}

inline _native::AdjustmentKind _native::AdjustmentKind::FromFFI(_native::capi::AdjustmentKind c_enum) {
    switch (c_enum) {
        case _native::capi::AdjustmentKind_Allowance:
        case _native::capi::AdjustmentKind_Charge:
            return static_cast<_native::AdjustmentKind::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_AdjustmentKind_HPP

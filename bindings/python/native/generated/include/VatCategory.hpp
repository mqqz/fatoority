#ifndef _NATIVE_VatCategory_HPP
#define _NATIVE_VatCategory_HPP

#include "VatCategory.d.hpp"

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

inline _native::capi::VatCategory _native::VatCategory::AsFFI() const {
    return static_cast<_native::capi::VatCategory>(value);
}

inline _native::VatCategory _native::VatCategory::FromFFI(_native::capi::VatCategory c_enum) {
    switch (c_enum) {
        case _native::capi::VatCategory_Exempt:
        case _native::capi::VatCategory_Standard:
        case _native::capi::VatCategory_Zero:
        case _native::capi::VatCategory_OutOfScope:
            return static_cast<_native::VatCategory::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_VatCategory_HPP

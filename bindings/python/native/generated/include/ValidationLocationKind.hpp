#ifndef _NATIVE_ValidationLocationKind_HPP
#define _NATIVE_ValidationLocationKind_HPP

#include "ValidationLocationKind.d.hpp"

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

inline _native::capi::ValidationLocationKind _native::ValidationLocationKind::AsFFI() const {
    return static_cast<_native::capi::ValidationLocationKind>(value);
}

inline _native::ValidationLocationKind _native::ValidationLocationKind::FromFFI(_native::capi::ValidationLocationKind c_enum) {
    switch (c_enum) {
        case _native::capi::ValidationLocationKind_Unknown:
        case _native::capi::ValidationLocationKind_Field:
        case _native::capi::ValidationLocationKind_XPath:
        case _native::capi::ValidationLocationKind_Xml:
            return static_cast<_native::ValidationLocationKind::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_ValidationLocationKind_HPP

#ifndef _NATIVE_Severity_HPP
#define _NATIVE_Severity_HPP

#include "Severity.d.hpp"

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

inline _native::capi::Severity _native::Severity::AsFFI() const {
    return static_cast<_native::capi::Severity>(value);
}

inline _native::Severity _native::Severity::FromFFI(_native::capi::Severity c_enum) {
    switch (c_enum) {
        case _native::capi::Severity_Warning:
        case _native::capi::Severity_Error:
            return static_cast<_native::Severity::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_Severity_HPP

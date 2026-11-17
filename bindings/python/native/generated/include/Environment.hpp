#ifndef _NATIVE_Environment_HPP
#define _NATIVE_Environment_HPP

#include "Environment.d.hpp"

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

inline _native::capi::Environment _native::Environment::AsFFI() const {
    return static_cast<_native::capi::Environment>(value);
}

inline _native::Environment _native::Environment::FromFFI(_native::capi::Environment c_enum) {
    switch (c_enum) {
        case _native::capi::Environment_NonProduction:
        case _native::capi::Environment_Simulation:
        case _native::capi::Environment_Production:
            return static_cast<_native::Environment::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_Environment_HPP

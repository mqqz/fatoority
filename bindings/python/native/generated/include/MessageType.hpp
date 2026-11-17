#ifndef _NATIVE_MessageType_HPP
#define _NATIVE_MessageType_HPP

#include "MessageType.d.hpp"

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

inline _native::capi::MessageType _native::MessageType::AsFFI() const {
    return static_cast<_native::capi::MessageType>(value);
}

inline _native::MessageType _native::MessageType::FromFFI(_native::capi::MessageType c_enum) {
    switch (c_enum) {
        case _native::capi::MessageType_Unknown:
        case _native::capi::MessageType_Info:
        case _native::capi::MessageType_Warning:
        case _native::capi::MessageType_Error:
            return static_cast<_native::MessageType::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_MessageType_HPP

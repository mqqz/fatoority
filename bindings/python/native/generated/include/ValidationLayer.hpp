#ifndef _NATIVE_ValidationLayer_HPP
#define _NATIVE_ValidationLayer_HPP

#include "ValidationLayer.d.hpp"

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

inline _native::capi::ValidationLayer _native::ValidationLayer::AsFFI() const {
    return static_cast<_native::capi::ValidationLayer>(value);
}

inline _native::ValidationLayer _native::ValidationLayer::FromFFI(_native::capi::ValidationLayer c_enum) {
    switch (c_enum) {
        case _native::capi::ValidationLayer_Unknown:
        case _native::capi::ValidationLayer_FieldChecks:
        case _native::capi::ValidationLayer_Xsd:
        case _native::capi::ValidationLayer_BusinessRules:
        case _native::capi::ValidationLayer_Signature:
        case _native::capi::ValidationLayer_Qr:
        case _native::capi::ValidationLayer_PreviousInvoiceHash:
            return static_cast<_native::ValidationLayer::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_ValidationLayer_HPP

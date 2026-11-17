#ifndef _NATIVE_InvoiceSubType_HPP
#define _NATIVE_InvoiceSubType_HPP

#include "InvoiceSubType.d.hpp"

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

inline _native::capi::InvoiceSubType _native::InvoiceSubType::AsFFI() const {
    return static_cast<_native::capi::InvoiceSubType>(value);
}

inline _native::InvoiceSubType _native::InvoiceSubType::FromFFI(_native::capi::InvoiceSubType c_enum) {
    switch (c_enum) {
        case _native::capi::InvoiceSubType_Standard:
        case _native::capi::InvoiceSubType_Simplified:
            return static_cast<_native::InvoiceSubType::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_InvoiceSubType_HPP

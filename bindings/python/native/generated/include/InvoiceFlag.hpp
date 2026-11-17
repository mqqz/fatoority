#ifndef _NATIVE_InvoiceFlag_HPP
#define _NATIVE_InvoiceFlag_HPP

#include "InvoiceFlag.d.hpp"

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
    extern "C" {

    uint8_t fatoora_InvoiceFlag_bit(_native::capi::InvoiceFlag self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::capi::InvoiceFlag _native::InvoiceFlag::AsFFI() const {
    return static_cast<_native::capi::InvoiceFlag>(value);
}

inline _native::InvoiceFlag _native::InvoiceFlag::FromFFI(_native::capi::InvoiceFlag c_enum) {
    switch (c_enum) {
        case _native::capi::InvoiceFlag_ThirdParty:
        case _native::capi::InvoiceFlag_Nominal:
        case _native::capi::InvoiceFlag_Export:
        case _native::capi::InvoiceFlag_Summary:
        case _native::capi::InvoiceFlag_SelfBilled:
            return static_cast<_native::InvoiceFlag::Value>(c_enum);
        default:
            std::abort();
    }
}

inline uint8_t _native::InvoiceFlag::bit() const {
    auto result = _native::capi::fatoora_InvoiceFlag_bit(this->AsFFI());
    return result;
}
#endif // _NATIVE_InvoiceFlag_HPP

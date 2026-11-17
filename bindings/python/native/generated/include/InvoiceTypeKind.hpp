#ifndef _NATIVE_InvoiceTypeKind_HPP
#define _NATIVE_InvoiceTypeKind_HPP

#include "InvoiceTypeKind.d.hpp"

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

inline _native::capi::InvoiceTypeKind _native::InvoiceTypeKind::AsFFI() const {
    return static_cast<_native::capi::InvoiceTypeKind>(value);
}

inline _native::InvoiceTypeKind _native::InvoiceTypeKind::FromFFI(_native::capi::InvoiceTypeKind c_enum) {
    switch (c_enum) {
        case _native::capi::InvoiceTypeKind_Tax:
        case _native::capi::InvoiceTypeKind_Prepayment:
        case _native::capi::InvoiceTypeKind_CreditNote:
        case _native::capi::InvoiceTypeKind_DebitNote:
            return static_cast<_native::InvoiceTypeKind::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_InvoiceTypeKind_HPP

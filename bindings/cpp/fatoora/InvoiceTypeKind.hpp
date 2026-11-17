#ifndef fatoora_InvoiceTypeKind_HPP
#define fatoora_InvoiceTypeKind_HPP

#include "InvoiceTypeKind.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace fatoora {
namespace capi {

} // namespace capi
} // namespace

inline fatoora::capi::InvoiceTypeKind fatoora::InvoiceTypeKind::AsFFI() const {
    return static_cast<fatoora::capi::InvoiceTypeKind>(value);
}

inline fatoora::InvoiceTypeKind fatoora::InvoiceTypeKind::FromFFI(fatoora::capi::InvoiceTypeKind c_enum) {
    switch (c_enum) {
        case fatoora::capi::InvoiceTypeKind_Tax:
        case fatoora::capi::InvoiceTypeKind_Prepayment:
        case fatoora::capi::InvoiceTypeKind_CreditNote:
        case fatoora::capi::InvoiceTypeKind_DebitNote:
            return static_cast<fatoora::InvoiceTypeKind::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_InvoiceTypeKind_HPP

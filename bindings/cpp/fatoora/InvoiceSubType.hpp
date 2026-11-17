#ifndef fatoora_InvoiceSubType_HPP
#define fatoora_InvoiceSubType_HPP

#include "InvoiceSubType.d.hpp"

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

inline fatoora::capi::InvoiceSubType fatoora::InvoiceSubType::AsFFI() const {
    return static_cast<fatoora::capi::InvoiceSubType>(value);
}

inline fatoora::InvoiceSubType fatoora::InvoiceSubType::FromFFI(fatoora::capi::InvoiceSubType c_enum) {
    switch (c_enum) {
        case fatoora::capi::InvoiceSubType_Standard:
        case fatoora::capi::InvoiceSubType_Simplified:
            return static_cast<fatoora::InvoiceSubType::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_InvoiceSubType_HPP

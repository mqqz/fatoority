#ifndef fatoora_InvoiceFlag_HPP
#define fatoora_InvoiceFlag_HPP

#include "InvoiceFlag.d.hpp"

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
    extern "C" {

    uint8_t fatoora_InvoiceFlag_bit(fatoora::capi::InvoiceFlag self);

    } // extern "C"
} // namespace capi
} // namespace

inline fatoora::capi::InvoiceFlag fatoora::InvoiceFlag::AsFFI() const {
    return static_cast<fatoora::capi::InvoiceFlag>(value);
}

inline fatoora::InvoiceFlag fatoora::InvoiceFlag::FromFFI(fatoora::capi::InvoiceFlag c_enum) {
    switch (c_enum) {
        case fatoora::capi::InvoiceFlag_ThirdParty:
        case fatoora::capi::InvoiceFlag_Nominal:
        case fatoora::capi::InvoiceFlag_Export:
        case fatoora::capi::InvoiceFlag_Summary:
        case fatoora::capi::InvoiceFlag_SelfBilled:
            return static_cast<fatoora::InvoiceFlag::Value>(c_enum);
        default:
            std::abort();
    }
}

inline uint8_t fatoora::InvoiceFlag::bit() const {
    auto result = fatoora::capi::fatoora_InvoiceFlag_bit(this->AsFFI());
    return result;
}
#endif // fatoora_InvoiceFlag_HPP

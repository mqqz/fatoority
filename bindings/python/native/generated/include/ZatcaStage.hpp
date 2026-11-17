#ifndef _NATIVE_ZatcaStage_HPP
#define _NATIVE_ZatcaStage_HPP

#include "ZatcaStage.d.hpp"

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

inline _native::capi::ZatcaStage _native::ZatcaStage::AsFFI() const {
    return static_cast<_native::capi::ZatcaStage>(value);
}

inline _native::ZatcaStage _native::ZatcaStage::FromFFI(_native::capi::ZatcaStage c_enum) {
    switch (c_enum) {
        case _native::capi::ZatcaStage_Unknown:
        case _native::capi::ZatcaStage_Xsd:
        case _native::capi::ZatcaStage_Cen:
        case _native::capi::ZatcaStage_Ksa:
        case _native::capi::ZatcaStage_Signature:
        case _native::capi::ZatcaStage_Qr:
        case _native::capi::ZatcaStage_PreviousInvoiceHash:
            return static_cast<_native::ZatcaStage::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_ZatcaStage_HPP

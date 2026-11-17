#ifndef fatoora_ZatcaStage_HPP
#define fatoora_ZatcaStage_HPP

#include "ZatcaStage.d.hpp"

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

inline fatoora::capi::ZatcaStage fatoora::ZatcaStage::AsFFI() const {
    return static_cast<fatoora::capi::ZatcaStage>(value);
}

inline fatoora::ZatcaStage fatoora::ZatcaStage::FromFFI(fatoora::capi::ZatcaStage c_enum) {
    switch (c_enum) {
        case fatoora::capi::ZatcaStage_Unknown:
        case fatoora::capi::ZatcaStage_Xsd:
        case fatoora::capi::ZatcaStage_Cen:
        case fatoora::capi::ZatcaStage_Ksa:
        case fatoora::capi::ZatcaStage_Signature:
        case fatoora::capi::ZatcaStage_Qr:
        case fatoora::capi::ZatcaStage_PreviousInvoiceHash:
            return static_cast<fatoora::ZatcaStage::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_ZatcaStage_HPP

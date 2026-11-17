#ifndef fatoora_ZatcaStageStatus_HPP
#define fatoora_ZatcaStageStatus_HPP

#include "ZatcaStageStatus.d.hpp"

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

inline fatoora::capi::ZatcaStageStatus fatoora::ZatcaStageStatus::AsFFI() const {
    return static_cast<fatoora::capi::ZatcaStageStatus>(value);
}

inline fatoora::ZatcaStageStatus fatoora::ZatcaStageStatus::FromFFI(fatoora::capi::ZatcaStageStatus c_enum) {
    switch (c_enum) {
        case fatoora::capi::ZatcaStageStatus_Unknown:
        case fatoora::capi::ZatcaStageStatus_NotRun:
        case fatoora::capi::ZatcaStageStatus_Completed:
        case fatoora::capi::ZatcaStageStatus_NotApplicable:
        case fatoora::capi::ZatcaStageStatus_ContextRequired:
        case fatoora::capi::ZatcaStageStatus_EvaluationFailed:
            return static_cast<fatoora::ZatcaStageStatus::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_ZatcaStageStatus_HPP

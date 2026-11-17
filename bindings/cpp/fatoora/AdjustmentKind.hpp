#ifndef fatoora_AdjustmentKind_HPP
#define fatoora_AdjustmentKind_HPP

#include "AdjustmentKind.d.hpp"

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

inline fatoora::capi::AdjustmentKind fatoora::AdjustmentKind::AsFFI() const {
    return static_cast<fatoora::capi::AdjustmentKind>(value);
}

inline fatoora::AdjustmentKind fatoora::AdjustmentKind::FromFFI(fatoora::capi::AdjustmentKind c_enum) {
    switch (c_enum) {
        case fatoora::capi::AdjustmentKind_Allowance:
        case fatoora::capi::AdjustmentKind_Charge:
            return static_cast<fatoora::AdjustmentKind::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_AdjustmentKind_HPP

#ifndef fatoora_ClearanceStatus_HPP
#define fatoora_ClearanceStatus_HPP

#include "ClearanceStatus.d.hpp"

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

inline fatoora::capi::ClearanceStatus fatoora::ClearanceStatus::AsFFI() const {
    return static_cast<fatoora::capi::ClearanceStatus>(value);
}

inline fatoora::ClearanceStatus fatoora::ClearanceStatus::FromFFI(fatoora::capi::ClearanceStatus c_enum) {
    switch (c_enum) {
        case fatoora::capi::ClearanceStatus_Unknown:
        case fatoora::capi::ClearanceStatus_Cleared:
        case fatoora::capi::ClearanceStatus_NotCleared:
            return static_cast<fatoora::ClearanceStatus::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_ClearanceStatus_HPP

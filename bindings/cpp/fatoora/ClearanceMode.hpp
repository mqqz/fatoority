#ifndef fatoora_ClearanceMode_HPP
#define fatoora_ClearanceMode_HPP

#include "ClearanceMode.d.hpp"

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

inline fatoora::capi::ClearanceMode fatoora::ClearanceMode::AsFFI() const {
    return static_cast<fatoora::capi::ClearanceMode>(value);
}

inline fatoora::ClearanceMode fatoora::ClearanceMode::FromFFI(fatoora::capi::ClearanceMode c_enum) {
    switch (c_enum) {
        case fatoora::capi::ClearanceMode_Disabled:
        case fatoora::capi::ClearanceMode_Enabled:
            return static_cast<fatoora::ClearanceMode::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_ClearanceMode_HPP

#ifndef fatoora_Environment_HPP
#define fatoora_Environment_HPP

#include "Environment.d.hpp"

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

inline fatoora::capi::Environment fatoora::Environment::AsFFI() const {
    return static_cast<fatoora::capi::Environment>(value);
}

inline fatoora::Environment fatoora::Environment::FromFFI(fatoora::capi::Environment c_enum) {
    switch (c_enum) {
        case fatoora::capi::Environment_NonProduction:
        case fatoora::capi::Environment_Simulation:
        case fatoora::capi::Environment_Production:
            return static_cast<fatoora::Environment::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_Environment_HPP

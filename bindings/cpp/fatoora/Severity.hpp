#ifndef fatoora_Severity_HPP
#define fatoora_Severity_HPP

#include "Severity.d.hpp"

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

inline fatoora::capi::Severity fatoora::Severity::AsFFI() const {
    return static_cast<fatoora::capi::Severity>(value);
}

inline fatoora::Severity fatoora::Severity::FromFFI(fatoora::capi::Severity c_enum) {
    switch (c_enum) {
        case fatoora::capi::Severity_Warning:
        case fatoora::capi::Severity_Error:
            return static_cast<fatoora::Severity::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_Severity_HPP

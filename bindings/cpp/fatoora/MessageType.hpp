#ifndef fatoora_MessageType_HPP
#define fatoora_MessageType_HPP

#include "MessageType.d.hpp"

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

inline fatoora::capi::MessageType fatoora::MessageType::AsFFI() const {
    return static_cast<fatoora::capi::MessageType>(value);
}

inline fatoora::MessageType fatoora::MessageType::FromFFI(fatoora::capi::MessageType c_enum) {
    switch (c_enum) {
        case fatoora::capi::MessageType_Unknown:
        case fatoora::capi::MessageType_Info:
        case fatoora::capi::MessageType_Warning:
        case fatoora::capi::MessageType_Error:
            return static_cast<fatoora::MessageType::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_MessageType_HPP

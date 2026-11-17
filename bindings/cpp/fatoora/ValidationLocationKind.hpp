#ifndef fatoora_ValidationLocationKind_HPP
#define fatoora_ValidationLocationKind_HPP

#include "ValidationLocationKind.d.hpp"

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

inline fatoora::capi::ValidationLocationKind fatoora::ValidationLocationKind::AsFFI() const {
    return static_cast<fatoora::capi::ValidationLocationKind>(value);
}

inline fatoora::ValidationLocationKind fatoora::ValidationLocationKind::FromFFI(fatoora::capi::ValidationLocationKind c_enum) {
    switch (c_enum) {
        case fatoora::capi::ValidationLocationKind_Unknown:
        case fatoora::capi::ValidationLocationKind_Field:
        case fatoora::capi::ValidationLocationKind_XPath:
        case fatoora::capi::ValidationLocationKind_Xml:
            return static_cast<fatoora::ValidationLocationKind::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_ValidationLocationKind_HPP

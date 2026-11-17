#ifndef fatoora_VatCategory_HPP
#define fatoora_VatCategory_HPP

#include "VatCategory.d.hpp"

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

inline fatoora::capi::VatCategory fatoora::VatCategory::AsFFI() const {
    return static_cast<fatoora::capi::VatCategory>(value);
}

inline fatoora::VatCategory fatoora::VatCategory::FromFFI(fatoora::capi::VatCategory c_enum) {
    switch (c_enum) {
        case fatoora::capi::VatCategory_Exempt:
        case fatoora::capi::VatCategory_Standard:
        case fatoora::capi::VatCategory_Zero:
        case fatoora::capi::VatCategory_OutOfScope:
            return static_cast<fatoora::VatCategory::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_VatCategory_HPP

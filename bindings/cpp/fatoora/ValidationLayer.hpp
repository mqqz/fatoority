#ifndef fatoora_ValidationLayer_HPP
#define fatoora_ValidationLayer_HPP

#include "ValidationLayer.d.hpp"

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

inline fatoora::capi::ValidationLayer fatoora::ValidationLayer::AsFFI() const {
    return static_cast<fatoora::capi::ValidationLayer>(value);
}

inline fatoora::ValidationLayer fatoora::ValidationLayer::FromFFI(fatoora::capi::ValidationLayer c_enum) {
    switch (c_enum) {
        case fatoora::capi::ValidationLayer_Unknown:
        case fatoora::capi::ValidationLayer_FieldChecks:
        case fatoora::capi::ValidationLayer_Xsd:
        case fatoora::capi::ValidationLayer_BusinessRules:
        case fatoora::capi::ValidationLayer_Signature:
        case fatoora::capi::ValidationLayer_Qr:
        case fatoora::capi::ValidationLayer_PreviousInvoiceHash:
            return static_cast<fatoora::ValidationLayer::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_ValidationLayer_HPP

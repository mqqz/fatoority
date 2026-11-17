#ifndef fatoora_ReportingStatus_HPP
#define fatoora_ReportingStatus_HPP

#include "ReportingStatus.d.hpp"

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

inline fatoora::capi::ReportingStatus fatoora::ReportingStatus::AsFFI() const {
    return static_cast<fatoora::capi::ReportingStatus>(value);
}

inline fatoora::ReportingStatus fatoora::ReportingStatus::FromFFI(fatoora::capi::ReportingStatus c_enum) {
    switch (c_enum) {
        case fatoora::capi::ReportingStatus_Unknown:
        case fatoora::capi::ReportingStatus_Reported:
        case fatoora::capi::ReportingStatus_NotReported:
            return static_cast<fatoora::ReportingStatus::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // fatoora_ReportingStatus_HPP

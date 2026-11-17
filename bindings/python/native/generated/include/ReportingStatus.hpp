#ifndef _NATIVE_ReportingStatus_HPP
#define _NATIVE_ReportingStatus_HPP

#include "ReportingStatus.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {

} // namespace capi
} // namespace

inline _native::capi::ReportingStatus _native::ReportingStatus::AsFFI() const {
    return static_cast<_native::capi::ReportingStatus>(value);
}

inline _native::ReportingStatus _native::ReportingStatus::FromFFI(_native::capi::ReportingStatus c_enum) {
    switch (c_enum) {
        case _native::capi::ReportingStatus_Unknown:
        case _native::capi::ReportingStatus_Reported:
        case _native::capi::ReportingStatus_NotReported:
            return static_cast<_native::ReportingStatus::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // _NATIVE_ReportingStatus_HPP

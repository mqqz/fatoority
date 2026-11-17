#ifndef _NATIVE_ZatcaStageStatus_D_HPP
#define _NATIVE_ZatcaStageStatus_D_HPP

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
    enum ZatcaStageStatus {
      ZatcaStageStatus_Unknown = 0,
      ZatcaStageStatus_NotRun = 1,
      ZatcaStageStatus_Completed = 2,
      ZatcaStageStatus_NotApplicable = 3,
      ZatcaStageStatus_ContextRequired = 4,
      ZatcaStageStatus_EvaluationFailed = 5,
    };

    typedef struct ZatcaStageStatus_option {union { ZatcaStageStatus ok; }; bool is_ok; } ZatcaStageStatus_option;
} // namespace capi
} // namespace

namespace _native {
/**
 * Whether a stage ran and produced a complete result.
 *
 * `Unknown` marks a status this binding version does not recognize.
 */
class ZatcaStageStatus {
public:
    enum Value {
        Unknown = 0,
        /**
         * The stage did not start, typically because an earlier stage failed.
         */
        NotRun = 1,
        /**
         * All checks finished; findings can still contain validation errors.
         */
        Completed = 2,
        /**
         * The profile does not require this stage for the document.
         */
        NotApplicable = 3,
        /**
         * Required caller context was unavailable.
         */
        ContextRequired = 4,
        /**
         * Execution stopped before the stage completed; findings may be partial.
         */
        EvaluationFailed = 5,
    };

    ZatcaStageStatus(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr ZatcaStageStatus(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline _native::capi::ZatcaStageStatus AsFFI() const;
    inline static _native::ZatcaStageStatus FromFFI(_native::capi::ZatcaStageStatus c_enum);
private:
    Value value;
};

} // namespace
#endif // _NATIVE_ZatcaStageStatus_D_HPP

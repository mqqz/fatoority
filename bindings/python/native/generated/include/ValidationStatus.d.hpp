#ifndef _NATIVE_ValidationStatus_D_HPP
#define _NATIVE_ValidationStatus_D_HPP

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
    enum ValidationStatus {
      ValidationStatus_Unknown = 0,
      ValidationStatus_Pass = 1,
      ValidationStatus_Warning = 2,
      ValidationStatus_Error = 3,
    };

    typedef struct ValidationStatus_option {union { ValidationStatus ok; }; bool is_ok; } ValidationStatus_option;
} // namespace capi
} // namespace

namespace _native {
/**
 * Overall `validationResults.status`, or the `status` of one validation message.
 *
 * `Unknown` marks a value this binding version does not recognize; read the
 * verbatim wire value with the sibling `*_raw` getter.
 */
class ValidationStatus {
public:
    enum Value {
        Unknown = 0,
        /**
         * Wire value `PASS`.
         */
        Pass = 1,
        /**
         * Wire value `WARNING`.
         */
        Warning = 2,
        /**
         * Wire value `ERROR`.
         */
        Error = 3,
    };

    ValidationStatus(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr ValidationStatus(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline _native::capi::ValidationStatus AsFFI() const;
    inline static _native::ValidationStatus FromFFI(_native::capi::ValidationStatus c_enum);
private:
    Value value;
};

} // namespace
#endif // _NATIVE_ValidationStatus_D_HPP

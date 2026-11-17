#ifndef _NATIVE_ClearanceStatus_D_HPP
#define _NATIVE_ClearanceStatus_D_HPP

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
    enum ClearanceStatus {
      ClearanceStatus_Unknown = 0,
      ClearanceStatus_Cleared = 1,
      ClearanceStatus_NotCleared = 2,
    };

    typedef struct ClearanceStatus_option {union { ClearanceStatus ok; }; bool is_ok; } ClearanceStatus_option;
} // namespace capi
} // namespace

namespace _native {
/**
 * `clearanceStatus` of a clearance response.
 *
 * `Unknown` marks a value this binding version does not recognize; read the
 * verbatim wire value with the sibling `*_raw` getter.
 */
class ClearanceStatus {
public:
    enum Value {
        Unknown = 0,
        /**
         * Wire value `CLEARED`.
         */
        Cleared = 1,
        /**
         * Wire value `NOT_CLEARED`.
         */
        NotCleared = 2,
    };

    ClearanceStatus(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr ClearanceStatus(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline _native::capi::ClearanceStatus AsFFI() const;
    inline static _native::ClearanceStatus FromFFI(_native::capi::ClearanceStatus c_enum);
private:
    Value value;
};

} // namespace
#endif // _NATIVE_ClearanceStatus_D_HPP

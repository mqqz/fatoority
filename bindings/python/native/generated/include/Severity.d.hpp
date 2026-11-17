#ifndef _NATIVE_Severity_D_HPP
#define _NATIVE_Severity_D_HPP

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
    enum Severity {
      Severity_Warning = 0,
      Severity_Error = 1,
    };

    typedef struct Severity_option {union { Severity ok; }; bool is_ok; } Severity_option;
} // namespace capi
} // namespace

namespace _native {
/**
 * Severity of a validation finding.
 */
class Severity {
public:
    enum Value {
        /**
         * Reported but does not make the document invalid.
         */
        Warning = 0,
        /**
         * Makes the document invalid.
         */
        Error = 1,
    };

    Severity(): value(Value::Warning) {}

    // Implicit conversions between enum and ::Value
    constexpr Severity(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline _native::capi::Severity AsFFI() const;
    inline static _native::Severity FromFFI(_native::capi::Severity c_enum);
private:
    Value value;
};

} // namespace
#endif // _NATIVE_Severity_D_HPP

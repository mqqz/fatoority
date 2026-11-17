#ifndef _NATIVE_ValidationLocationKind_D_HPP
#define _NATIVE_ValidationLocationKind_D_HPP

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
    enum ValidationLocationKind {
      ValidationLocationKind_Unknown = 0,
      ValidationLocationKind_Field = 1,
      ValidationLocationKind_XPath = 2,
      ValidationLocationKind_Xml = 3,
    };

    typedef struct ValidationLocationKind_option {union { ValidationLocationKind ok; }; bool is_ok; } ValidationLocationKind_option;
} // namespace capi
} // namespace

namespace _native {
/**
 * Which kind of location a finding carries.
 *
 * `Unknown` marks a location kind this binding version does not recognize.
 */
class ValidationLocationKind {
public:
    enum Value {
        Unknown = 0,
        /**
         * A model field path such as `line_items[0].quantity`; see `location_path`.
         */
        Field = 1,
        /**
         * A namespace-independent XPath; see `location_path`.
         */
        XPath = 2,
        /**
         * One-based XML source coordinates; see `location_line`/`location_column`.
         */
        Xml = 3,
    };

    ValidationLocationKind(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr ValidationLocationKind(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline _native::capi::ValidationLocationKind AsFFI() const;
    inline static _native::ValidationLocationKind FromFFI(_native::capi::ValidationLocationKind c_enum);
private:
    Value value;
};

} // namespace
#endif // _NATIVE_ValidationLocationKind_D_HPP

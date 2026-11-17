#ifndef _NATIVE_Environment_D_HPP
#define _NATIVE_Environment_D_HPP

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
    enum Environment {
      Environment_NonProduction = 0,
      Environment_Simulation = 1,
      Environment_Production = 2,
    };

    typedef struct Environment_option {union { Environment ok; }; bool is_ok; } Environment_option;
} // namespace capi
} // namespace

namespace _native {
/**
 * ZATCA environment selecting the API base URL and certificate profile.
 *
 * Discriminants match the `u8` codes used before 0.6. .NET names it
 * `ZatcaEnvironment` so it cannot be confused with `System.Environment`.
 */
class Environment {
public:
    enum Value {
        NonProduction = 0,
        Simulation = 1,
        Production = 2,
    };

    Environment(): value(Value::NonProduction) {}

    // Implicit conversions between enum and ::Value
    constexpr Environment(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline _native::capi::Environment AsFFI() const;
    inline static _native::Environment FromFFI(_native::capi::Environment c_enum);
private:
    Value value;
};

} // namespace
#endif // _NATIVE_Environment_D_HPP

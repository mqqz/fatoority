#ifndef _NATIVE_InvoiceFlag_D_HPP
#define _NATIVE_InvoiceFlag_D_HPP

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
    enum InvoiceFlag {
      InvoiceFlag_ThirdParty = 1,
      InvoiceFlag_Nominal = 2,
      InvoiceFlag_Export = 4,
      InvoiceFlag_Summary = 8,
      InvoiceFlag_SelfBilled = 16,
    };

    typedef struct InvoiceFlag_option {union { InvoiceFlag ok; }; bool is_ok; } InvoiceFlag_option;
} // namespace capi
} // namespace

namespace _native {
/**
 * Named bits of the `u8` invoice flag mask accepted by
 * `InvoiceBuilder::flags` and returned by `InvoiceData::flags_raw`.
 *
 * Each discriminant is the flag's bit value; `bit()` returns it as a `u8`
 * for languages whose enums do not expose their discriminant.
 */
class InvoiceFlag {
public:
    enum Value {
        ThirdParty = 1,
        Nominal = 2,
        Export = 4,
        Summary = 8,
        SelfBilled = 16,
    };

    InvoiceFlag(): value(Value::ThirdParty) {}

    // Implicit conversions between enum and ::Value
    constexpr InvoiceFlag(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

  /**
   * The flag's bit within the invoice flag mask.
   */
  inline uint8_t bit() const;

    inline _native::capi::InvoiceFlag AsFFI() const;
    inline static _native::InvoiceFlag FromFFI(_native::capi::InvoiceFlag c_enum);
private:
    Value value;
};

} // namespace
#endif // _NATIVE_InvoiceFlag_D_HPP

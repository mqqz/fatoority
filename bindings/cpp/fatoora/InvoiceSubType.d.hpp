#ifndef fatoora_InvoiceSubType_D_HPP
#define fatoora_InvoiceSubType_D_HPP

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
    enum InvoiceSubType {
      InvoiceSubType_Standard = 0,
      InvoiceSubType_Simplified = 1,
    };

    typedef struct InvoiceSubType_option {union { InvoiceSubType ok; }; bool is_ok; } InvoiceSubType_option;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * Standard (B2B, cleared) or simplified (B2C, reported) invoice.
 *
 * Discriminants match the `u8` codes used before 0.6.
 */
class InvoiceSubType {
public:
    enum Value {
        Standard = 0,
        Simplified = 1,
    };

    InvoiceSubType(): value(Value::Standard) {}

    // Implicit conversions between enum and ::Value
    constexpr InvoiceSubType(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline fatoora::capi::InvoiceSubType AsFFI() const;
    inline static fatoora::InvoiceSubType FromFFI(fatoora::capi::InvoiceSubType c_enum);
private:
    Value value;
};

} // namespace
#endif // fatoora_InvoiceSubType_D_HPP

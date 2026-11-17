#ifndef _NATIVE_InvoiceTypeKind_D_HPP
#define _NATIVE_InvoiceTypeKind_D_HPP

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
    enum InvoiceTypeKind {
      InvoiceTypeKind_Tax = 0,
      InvoiceTypeKind_Prepayment = 1,
      InvoiceTypeKind_CreditNote = 2,
      InvoiceTypeKind_DebitNote = 3,
    };

    typedef struct InvoiceTypeKind_option {union { InvoiceTypeKind ok; }; bool is_ok; } InvoiceTypeKind_option;
} // namespace capi
} // namespace

namespace _native {
/**
 * Kind of invoice document (UNCL1001 type code).
 *
 * Discriminants match the `u8` codes used before 0.6.
 */
class InvoiceTypeKind {
public:
    enum Value {
        /**
         * Tax invoice (`388`).
         */
        Tax = 0,
        /**
         * Prepayment invoice (`386`).
         */
        Prepayment = 1,
        /**
         * Credit note (`381`).
         */
        CreditNote = 2,
        /**
         * Debit note (`383`).
         */
        DebitNote = 3,
    };

    InvoiceTypeKind(): value(Value::Tax) {}

    // Implicit conversions between enum and ::Value
    constexpr InvoiceTypeKind(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline _native::capi::InvoiceTypeKind AsFFI() const;
    inline static _native::InvoiceTypeKind FromFFI(_native::capi::InvoiceTypeKind c_enum);
private:
    Value value;
};

} // namespace
#endif // _NATIVE_InvoiceTypeKind_D_HPP

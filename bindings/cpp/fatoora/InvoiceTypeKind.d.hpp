#ifndef fatoora_InvoiceTypeKind_D_HPP
#define fatoora_InvoiceTypeKind_D_HPP

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
    enum InvoiceTypeKind {
      InvoiceTypeKind_Tax = 0,
      InvoiceTypeKind_Prepayment = 1,
      InvoiceTypeKind_CreditNote = 2,
      InvoiceTypeKind_DebitNote = 3,
    };

    typedef struct InvoiceTypeKind_option {union { InvoiceTypeKind ok; }; bool is_ok; } InvoiceTypeKind_option;
} // namespace capi
} // namespace

namespace fatoora {
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

    inline fatoora::capi::InvoiceTypeKind AsFFI() const;
    inline static fatoora::InvoiceTypeKind FromFFI(fatoora::capi::InvoiceTypeKind c_enum);
private:
    Value value;
};

} // namespace
#endif // fatoora_InvoiceTypeKind_D_HPP

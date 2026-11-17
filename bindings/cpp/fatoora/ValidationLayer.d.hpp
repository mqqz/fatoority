#ifndef fatoora_ValidationLayer_D_HPP
#define fatoora_ValidationLayer_D_HPP

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
    enum ValidationLayer {
      ValidationLayer_Unknown = 0,
      ValidationLayer_FieldChecks = 1,
      ValidationLayer_Xsd = 2,
      ValidationLayer_BusinessRules = 3,
      ValidationLayer_Signature = 4,
      ValidationLayer_Qr = 5,
      ValidationLayer_PreviousInvoiceHash = 6,
    };

    typedef struct ValidationLayer_option {union { ValidationLayer ok; }; bool is_ok; } ValidationLayer_option;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * The validation layer that produced a finding.
 *
 * `Unknown` marks a layer this binding version does not recognize.
 */
class ValidationLayer {
public:
    enum Value {
        Unknown = 0,
        /**
         * Builder/model field checks.
         */
        FieldChecks = 1,
        /**
         * UBL XML schema (XSD) validation.
         */
        Xsd = 2,
        /**
         * CEN EN 16931 and KSA business rules.
         */
        BusinessRules = 3,
        /**
         * Cryptographic signature, digest and certificate checks.
         */
        Signature = 4,
        /**
         * QR payload consistency.
         */
        Qr = 5,
        /**
         * Previous invoice hash comparison.
         */
        PreviousInvoiceHash = 6,
    };

    ValidationLayer(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr ValidationLayer(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline fatoora::capi::ValidationLayer AsFFI() const;
    inline static fatoora::ValidationLayer FromFFI(fatoora::capi::ValidationLayer c_enum);
private:
    Value value;
};

} // namespace
#endif // fatoora_ValidationLayer_D_HPP

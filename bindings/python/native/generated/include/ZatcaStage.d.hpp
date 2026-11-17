#ifndef _NATIVE_ZatcaStage_D_HPP
#define _NATIVE_ZatcaStage_D_HPP

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
    enum ZatcaStage {
      ZatcaStage_Unknown = 0,
      ZatcaStage_Xsd = 1,
      ZatcaStage_Cen = 2,
      ZatcaStage_Ksa = 3,
      ZatcaStage_Signature = 4,
      ZatcaStage_Qr = 5,
      ZatcaStage_PreviousInvoiceHash = 6,
    };

    typedef struct ZatcaStage_option {union { ZatcaStage ok; }; bool is_ok; } ZatcaStage_option;
} // namespace capi
} // namespace

namespace _native {
/**
 * A stage of the local ZATCA validation profile.
 *
 * `Unknown` marks a stage this binding version does not recognize.
 */
class ZatcaStage {
public:
    enum Value {
        Unknown = 0,
        /**
         * UBL XML schema (XSD) validation.
         */
        Xsd = 1,
        /**
         * CEN EN 16931 business rules.
         */
        Cen = 2,
        /**
         * ZATCA KSA-specific business rules.
         */
        Ksa = 3,
        /**
         * Cryptographic signature, digest and certificate checks.
         */
        Signature = 4,
        /**
         * QR payload consistency checks.
         */
        Qr = 5,
        /**
         * Comparison of the embedded previous invoice hash with the supplied one.
         */
        PreviousInvoiceHash = 6,
    };

    ZatcaStage(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr ZatcaStage(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline _native::capi::ZatcaStage AsFFI() const;
    inline static _native::ZatcaStage FromFFI(_native::capi::ZatcaStage c_enum);
private:
    Value value;
};

} // namespace
#endif // _NATIVE_ZatcaStage_D_HPP

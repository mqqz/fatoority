#ifndef fatoora_VatCategory_D_HPP
#define fatoora_VatCategory_D_HPP

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
    enum VatCategory {
      VatCategory_Exempt = 0,
      VatCategory_Standard = 1,
      VatCategory_Zero = 2,
      VatCategory_OutOfScope = 3,
    };

    typedef struct VatCategory_option {union { VatCategory ok; }; bool is_ok; } VatCategory_option;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * VAT category of a line, breakdown, or document-level adjustment.
 *
 * Discriminants match the `u8` codes used before 0.6.
 */
class VatCategory {
public:
    enum Value {
        /**
         * UNCL5305 `E`.
         */
        Exempt = 0,
        /**
         * UNCL5305 `S`.
         */
        Standard = 1,
        /**
         * UNCL5305 `Z`.
         */
        Zero = 2,
        /**
         * UNCL5305 `O`.
         */
        OutOfScope = 3,
    };

    VatCategory(): value(Value::Exempt) {}

    // Implicit conversions between enum and ::Value
    constexpr VatCategory(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline fatoora::capi::VatCategory AsFFI() const;
    inline static fatoora::VatCategory FromFFI(fatoora::capi::VatCategory c_enum);
private:
    Value value;
};

} // namespace
#endif // fatoora_VatCategory_D_HPP

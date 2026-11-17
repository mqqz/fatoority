#ifndef fatoora_AdjustmentKind_D_HPP
#define fatoora_AdjustmentKind_D_HPP

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
    enum AdjustmentKind {
      AdjustmentKind_Allowance = 0,
      AdjustmentKind_Charge = 1,
    };

    typedef struct AdjustmentKind_option {union { AdjustmentKind ok; }; bool is_ok; } AdjustmentKind_option;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * Whether a document-level adjustment is an allowance (discount) or a charge.
 */
class AdjustmentKind {
public:
    enum Value {
        Allowance = 0,
        Charge = 1,
    };

    AdjustmentKind(): value(Value::Allowance) {}

    // Implicit conversions between enum and ::Value
    constexpr AdjustmentKind(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline fatoora::capi::AdjustmentKind AsFFI() const;
    inline static fatoora::AdjustmentKind FromFFI(fatoora::capi::AdjustmentKind c_enum);
private:
    Value value;
};

} // namespace
#endif // fatoora_AdjustmentKind_D_HPP

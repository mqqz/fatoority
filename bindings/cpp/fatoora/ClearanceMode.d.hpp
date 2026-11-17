#ifndef fatoora_ClearanceMode_D_HPP
#define fatoora_ClearanceMode_D_HPP

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
    enum ClearanceMode {
      ClearanceMode_Disabled = 0,
      ClearanceMode_Enabled = 1,
    };

    typedef struct ClearanceMode_option {union { ClearanceMode ok; }; bool is_ok; } ClearanceMode_option;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * Value of the `Clearance-Status` header sent with reporting and clearance requests.
 */
class ClearanceMode {
public:
    enum Value {
        /**
         * Header value `0` (clearance is switched off for the taxpayer).
         */
        Disabled = 0,
        /**
         * Header value `1` (clearance is switched on for the taxpayer).
         */
        Enabled = 1,
    };

    ClearanceMode(): value(Value::Disabled) {}

    // Implicit conversions between enum and ::Value
    constexpr ClearanceMode(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline fatoora::capi::ClearanceMode AsFFI() const;
    inline static fatoora::ClearanceMode FromFFI(fatoora::capi::ClearanceMode c_enum);
private:
    Value value;
};

} // namespace
#endif // fatoora_ClearanceMode_D_HPP

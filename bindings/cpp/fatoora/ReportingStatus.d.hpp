#ifndef fatoora_ReportingStatus_D_HPP
#define fatoora_ReportingStatus_D_HPP

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
    enum ReportingStatus {
      ReportingStatus_Unknown = 0,
      ReportingStatus_Reported = 1,
      ReportingStatus_NotReported = 2,
    };

    typedef struct ReportingStatus_option {union { ReportingStatus ok; }; bool is_ok; } ReportingStatus_option;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * `reportingStatus` of a reporting response.
 *
 * `Unknown` marks a value this binding version does not recognize; read the
 * verbatim wire value with the sibling `*_raw` getter.
 */
class ReportingStatus {
public:
    enum Value {
        Unknown = 0,
        /**
         * Wire value `REPORTED`.
         */
        Reported = 1,
        /**
         * Wire value `NOT_REPORTED`.
         */
        NotReported = 2,
    };

    ReportingStatus(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr ReportingStatus(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline fatoora::capi::ReportingStatus AsFFI() const;
    inline static fatoora::ReportingStatus FromFFI(fatoora::capi::ReportingStatus c_enum);
private:
    Value value;
};

} // namespace
#endif // fatoora_ReportingStatus_D_HPP

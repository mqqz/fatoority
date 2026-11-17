#ifndef fatoora_MessageType_D_HPP
#define fatoora_MessageType_D_HPP

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
    enum MessageType {
      MessageType_Unknown = 0,
      MessageType_Info = 1,
      MessageType_Warning = 2,
      MessageType_Error = 3,
    };

    typedef struct MessageType_option {union { MessageType ok; }; bool is_ok; } MessageType_option;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * The `type` of a validation message.
 *
 * `Unknown` marks a value this binding version does not recognize; read the
 * verbatim wire value with the sibling `*_raw` getter.
 */
class MessageType {
public:
    enum Value {
        Unknown = 0,
        /**
         * Wire value `INFO`.
         */
        Info = 1,
        /**
         * Wire value `WARNING`.
         */
        Warning = 2,
        /**
         * Wire value `ERROR`.
         */
        Error = 3,
    };

    MessageType(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr MessageType(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline fatoora::capi::MessageType AsFFI() const;
    inline static fatoora::MessageType FromFFI(fatoora::capi::MessageType c_enum);
private:
    Value value;
};

} // namespace
#endif // fatoora_MessageType_D_HPP

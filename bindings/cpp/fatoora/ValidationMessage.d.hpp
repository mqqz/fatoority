#ifndef fatoora_ValidationMessage_D_HPP
#define fatoora_ValidationMessage_D_HPP

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
namespace capi { struct BindingError; }
class BindingError;
namespace capi { struct Text; }
class Text;
class MessageType;
class ValidationStatus;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct ValidationMessage;
} // namespace capi
} // namespace

namespace fatoora {
class ValidationMessage {
public:

  /**
   * The parsed message `type`, or null when the message has none.
   */
  inline std::optional<fatoora::MessageType> message_type() const;

  /**
   * The message `type` exactly as ZATCA sent it, or null when absent.
   */
  inline diplomat::result<diplomat::maybe_null<std::unique_ptr<fatoora::Text>>, std::unique_ptr<fatoora::BindingError>> message_type_raw() const;

  inline diplomat::result<diplomat::maybe_null<std::unique_ptr<fatoora::Text>>, std::unique_ptr<fatoora::BindingError>> code() const;

  inline diplomat::result<diplomat::maybe_null<std::unique_ptr<fatoora::Text>>, std::unique_ptr<fatoora::BindingError>> category() const;

  inline diplomat::result<diplomat::maybe_null<std::unique_ptr<fatoora::Text>>, std::unique_ptr<fatoora::BindingError>> message() const;

  /**
   * The parsed message `status`, or null when the message has none.
   */
  inline std::optional<fatoora::ValidationStatus> status() const;

  /**
   * The message `status` exactly as ZATCA sent it, or null when absent.
   */
  inline diplomat::result<diplomat::maybe_null<std::unique_ptr<fatoora::Text>>, std::unique_ptr<fatoora::BindingError>> status_raw() const;

    inline const fatoora::capi::ValidationMessage* AsFFI() const;
    inline fatoora::capi::ValidationMessage* AsFFI();
    inline static const fatoora::ValidationMessage* FromFFI(const fatoora::capi::ValidationMessage* ptr);
    inline static fatoora::ValidationMessage* FromFFI(fatoora::capi::ValidationMessage* ptr);
    inline static void operator delete(void* ptr);
private:
    ValidationMessage() = delete;
    ValidationMessage(const fatoora::ValidationMessage&) = delete;
    ValidationMessage(fatoora::ValidationMessage&&) noexcept = delete;
    ValidationMessage operator=(const fatoora::ValidationMessage&) = delete;
    ValidationMessage operator=(fatoora::ValidationMessage&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_ValidationMessage_D_HPP

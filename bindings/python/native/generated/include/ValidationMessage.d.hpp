#ifndef _NATIVE_ValidationMessage_D_HPP
#define _NATIVE_ValidationMessage_D_HPP

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
class MessageType;
class ValidationStatus;
} // namespace _native



namespace _native {
namespace capi {
    struct ValidationMessage;
} // namespace capi
} // namespace

namespace _native {
class ValidationMessage {
public:

  /**
   * The parsed message `type`, or null when the message has none.
   */
  inline std::optional<_native::MessageType> message_type() const;

  /**
   * The message `type` exactly as ZATCA sent it, or null when absent.
   */
  inline std::optional<std::string> message_type_raw() const;
  template<typename W>
  inline std::optional<std::monostate> message_type_raw_write(W& writeable_output) const;

  inline std::optional<std::string> code() const;
  template<typename W>
  inline std::optional<std::monostate> code_write(W& writeable_output) const;

  inline std::optional<std::string> category() const;
  template<typename W>
  inline std::optional<std::monostate> category_write(W& writeable_output) const;

  inline std::optional<std::string> message() const;
  template<typename W>
  inline std::optional<std::monostate> message_write(W& writeable_output) const;

  /**
   * The parsed message `status`, or null when the message has none.
   */
  inline std::optional<_native::ValidationStatus> status() const;

  /**
   * The message `status` exactly as ZATCA sent it, or null when absent.
   */
  inline std::optional<std::string> status_raw() const;
  template<typename W>
  inline std::optional<std::monostate> status_raw_write(W& writeable_output) const;

    inline const _native::capi::ValidationMessage* AsFFI() const;
    inline _native::capi::ValidationMessage* AsFFI();
    inline static const _native::ValidationMessage* FromFFI(const _native::capi::ValidationMessage* ptr);
    inline static _native::ValidationMessage* FromFFI(_native::capi::ValidationMessage* ptr);
    inline static void operator delete(void* ptr);
private:
    ValidationMessage() = delete;
    ValidationMessage(const _native::ValidationMessage&) = delete;
    ValidationMessage(_native::ValidationMessage&&) noexcept = delete;
    ValidationMessage operator=(const _native::ValidationMessage&) = delete;
    ValidationMessage operator=(_native::ValidationMessage&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_ValidationMessage_D_HPP

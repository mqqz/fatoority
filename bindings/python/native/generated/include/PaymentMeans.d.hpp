#ifndef _NATIVE_PaymentMeans_D_HPP
#define _NATIVE_PaymentMeans_D_HPP

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
namespace capi { struct BindingError; }
class BindingError;
namespace capi { struct PaymentMeans; }
class PaymentMeans;
} // namespace _native



namespace _native {
namespace capi {
    struct PaymentMeans;
} // namespace capi
} // namespace

namespace _native {
class PaymentMeans {
public:

  inline _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>> with_instruction_note(std::string_view value) const;

  inline size_t instruction_notes_len() const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> instruction_note_at(size_t index) const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> instruction_note_at_write(size_t index, W& writeable_output) const;

  inline static _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>> new_(std::string_view code, std::optional<std::string_view> instruction);

  inline _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>> with_account(std::optional<std::string_view> id, std::optional<std::string_view> terms) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> code() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> code_write(W& writeable_output) const;

  inline std::optional<std::string> instruction_note() const;
  template<typename W>
  inline std::optional<std::monostate> instruction_note_write(W& writeable_output) const;

  inline bool has_account() const;

  inline std::optional<std::string> account_identifier() const;
  template<typename W>
  inline std::optional<std::monostate> account_identifier_write(W& writeable_output) const;

  inline std::optional<std::string> payment_terms() const;
  template<typename W>
  inline std::optional<std::monostate> payment_terms_write(W& writeable_output) const;

    inline const _native::capi::PaymentMeans* AsFFI() const;
    inline _native::capi::PaymentMeans* AsFFI();
    inline static const _native::PaymentMeans* FromFFI(const _native::capi::PaymentMeans* ptr);
    inline static _native::PaymentMeans* FromFFI(_native::capi::PaymentMeans* ptr);
    inline static void operator delete(void* ptr);
private:
    PaymentMeans() = delete;
    PaymentMeans(const _native::PaymentMeans&) = delete;
    PaymentMeans(_native::PaymentMeans&&) noexcept = delete;
    PaymentMeans operator=(const _native::PaymentMeans&) = delete;
    PaymentMeans operator=(_native::PaymentMeans&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_PaymentMeans_D_HPP

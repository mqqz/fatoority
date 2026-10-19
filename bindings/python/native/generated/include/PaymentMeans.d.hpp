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
namespace capi { struct Text; }
class Text;
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

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> instruction_note_at(size_t index) const;

  inline static _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>> new_(std::string_view code, std::optional<std::string_view> instruction);

  inline _native::diplomat::result<std::unique_ptr<_native::PaymentMeans>, std::unique_ptr<_native::BindingError>> with_account(std::optional<std::string_view> id, std::optional<std::string_view> terms) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> code() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> code_write(W& writeable_output) const;

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> instruction_note() const;

  inline bool has_account() const;

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> account_identifier() const;

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> payment_terms() const;

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

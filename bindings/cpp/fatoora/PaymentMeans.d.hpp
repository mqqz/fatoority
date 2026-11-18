#ifndef fatoora_PaymentMeans_D_HPP
#define fatoora_PaymentMeans_D_HPP

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
namespace capi { struct PaymentMeans; }
class PaymentMeans;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct PaymentMeans;
} // namespace capi
} // namespace

namespace fatoora {
class PaymentMeans {
public:

  inline diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>> with_instruction_note(std::string_view value) const;

  inline size_t instruction_notes_len() const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> instruction_note_at(size_t index) const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> instruction_note_at_write(size_t index, W& writeable_output) const;

  inline static diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>> new_(std::string_view code, std::optional<std::string_view> instruction);

  inline diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>> with_account(std::optional<std::string_view> id, std::optional<std::string_view> terms) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> code() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> code_write(W& writeable_output) const;

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

    inline const fatoora::capi::PaymentMeans* AsFFI() const;
    inline fatoora::capi::PaymentMeans* AsFFI();
    inline static const fatoora::PaymentMeans* FromFFI(const fatoora::capi::PaymentMeans* ptr);
    inline static fatoora::PaymentMeans* FromFFI(fatoora::capi::PaymentMeans* ptr);
    inline static void operator delete(void* ptr);
private:
    PaymentMeans() = delete;
    PaymentMeans(const fatoora::PaymentMeans&) = delete;
    PaymentMeans(fatoora::PaymentMeans&&) noexcept = delete;
    PaymentMeans operator=(const fatoora::PaymentMeans&) = delete;
    PaymentMeans operator=(fatoora::PaymentMeans&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_PaymentMeans_D_HPP

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
namespace capi { struct Text; }
class Text;
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

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> instruction_note_at(size_t index) const;

  inline static diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>> new_(std::string_view code, std::optional<std::string_view> instruction);

  inline diplomat::result<std::unique_ptr<fatoora::PaymentMeans>, std::unique_ptr<fatoora::BindingError>> with_account(std::optional<std::string_view> id, std::optional<std::string_view> terms) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> code() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> code_write(W& writeable_output) const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> instruction_note() const;

  inline bool has_account() const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> account_identifier() const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> payment_terms() const;

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

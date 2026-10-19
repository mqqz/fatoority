#ifndef fatoora_VatBreakdown_D_HPP
#define fatoora_VatBreakdown_D_HPP

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
namespace capi { struct TaxExemption; }
class TaxExemption;
namespace capi { struct VatBreakdown; }
class VatBreakdown;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct VatBreakdown;
} // namespace capi
} // namespace

namespace fatoora {
class VatBreakdown {
public:

  inline bool has_vat_rate() const;

  inline diplomat::result<std::unique_ptr<fatoora::VatBreakdown>, std::unique_ptr<fatoora::BindingError>> without_vat_rate() const;

  inline static diplomat::result<std::unique_ptr<fatoora::VatBreakdown>, std::unique_ptr<fatoora::BindingError>> new_(uint8_t category, std::string_view rate, std::string_view taxable_amount, std::string_view tax_amount, const fatoora::TaxExemption& exemption);

  inline uint8_t category() const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> rate() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> rate_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> taxable_amount() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> taxable_amount_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> tax_amount() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> tax_amount_write(W& writeable_output) const;

  inline diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>> exemption() const;

    inline const fatoora::capi::VatBreakdown* AsFFI() const;
    inline fatoora::capi::VatBreakdown* AsFFI();
    inline static const fatoora::VatBreakdown* FromFFI(const fatoora::capi::VatBreakdown* ptr);
    inline static fatoora::VatBreakdown* FromFFI(fatoora::capi::VatBreakdown* ptr);
    inline static void operator delete(void* ptr);
private:
    VatBreakdown() = delete;
    VatBreakdown(const fatoora::VatBreakdown&) = delete;
    VatBreakdown(fatoora::VatBreakdown&&) noexcept = delete;
    VatBreakdown operator=(const fatoora::VatBreakdown&) = delete;
    VatBreakdown operator=(fatoora::VatBreakdown&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_VatBreakdown_D_HPP

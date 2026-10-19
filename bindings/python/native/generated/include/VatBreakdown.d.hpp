#ifndef _NATIVE_VatBreakdown_D_HPP
#define _NATIVE_VatBreakdown_D_HPP

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
namespace capi { struct TaxExemption; }
class TaxExemption;
namespace capi { struct VatBreakdown; }
class VatBreakdown;
} // namespace _native



namespace _native {
namespace capi {
    struct VatBreakdown;
} // namespace capi
} // namespace

namespace _native {
class VatBreakdown {
public:

  inline bool has_vat_rate() const;

  inline _native::diplomat::result<std::unique_ptr<_native::VatBreakdown>, std::unique_ptr<_native::BindingError>> without_vat_rate() const;

  inline static _native::diplomat::result<std::unique_ptr<_native::VatBreakdown>, std::unique_ptr<_native::BindingError>> new_(uint8_t category, std::string_view rate, std::string_view taxable_amount, std::string_view tax_amount, const _native::TaxExemption& exemption);

  inline uint8_t category() const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> rate() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> rate_write(W& writeable_output) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> taxable_amount() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> taxable_amount_write(W& writeable_output) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> tax_amount() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> tax_amount_write(W& writeable_output) const;

  inline _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>> exemption() const;

    inline const _native::capi::VatBreakdown* AsFFI() const;
    inline _native::capi::VatBreakdown* AsFFI();
    inline static const _native::VatBreakdown* FromFFI(const _native::capi::VatBreakdown* ptr);
    inline static _native::VatBreakdown* FromFFI(_native::capi::VatBreakdown* ptr);
    inline static void operator delete(void* ptr);
private:
    VatBreakdown() = delete;
    VatBreakdown(const _native::VatBreakdown&) = delete;
    VatBreakdown(_native::VatBreakdown&&) noexcept = delete;
    VatBreakdown operator=(const _native::VatBreakdown&) = delete;
    VatBreakdown operator=(_native::VatBreakdown&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_VatBreakdown_D_HPP

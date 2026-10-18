#ifndef _NATIVE_AllowanceCharge_D_HPP
#define _NATIVE_AllowanceCharge_D_HPP

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
namespace capi { struct AllowanceCharge; }
class AllowanceCharge;
namespace capi { struct BindingError; }
class BindingError;
namespace capi { struct TaxExemption; }
class TaxExemption;
namespace capi { struct Text; }
class Text;
} // namespace _native



namespace _native {
namespace capi {
    struct AllowanceCharge;
} // namespace capi
} // namespace

namespace _native {
class AllowanceCharge {
public:

  inline static _native::diplomat::result<std::unique_ptr<_native::AllowanceCharge>, std::unique_ptr<_native::BindingError>> new_(bool charge, std::string_view amount, std::optional<std::string_view> reason, std::optional<std::string_view> reason_code, uint8_t category, std::string_view rate, const _native::TaxExemption& exemption);

  inline bool is_charge() const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> amount() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> amount_write(W& writeable_output) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> rate() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> rate_write(W& writeable_output) const;

  inline uint8_t category() const;

  inline size_t tax_category_count() const;

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> reason() const;

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> reason_code() const;

  inline _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>> exemption() const;

    inline const _native::capi::AllowanceCharge* AsFFI() const;
    inline _native::capi::AllowanceCharge* AsFFI();
    inline static const _native::AllowanceCharge* FromFFI(const _native::capi::AllowanceCharge* ptr);
    inline static _native::AllowanceCharge* FromFFI(_native::capi::AllowanceCharge* ptr);
    inline static void operator delete(void* ptr);
private:
    AllowanceCharge() = delete;
    AllowanceCharge(const _native::AllowanceCharge&) = delete;
    AllowanceCharge(_native::AllowanceCharge&&) noexcept = delete;
    AllowanceCharge operator=(const _native::AllowanceCharge&) = delete;
    AllowanceCharge operator=(_native::AllowanceCharge&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_AllowanceCharge_D_HPP

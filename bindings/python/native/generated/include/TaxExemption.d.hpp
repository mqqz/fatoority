#ifndef _NATIVE_TaxExemption_D_HPP
#define _NATIVE_TaxExemption_D_HPP

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
namespace capi { struct Text; }
class Text;
} // namespace _native



namespace _native {
namespace capi {
    struct TaxExemption;
} // namespace capi
} // namespace

namespace _native {
class TaxExemption {
public:

  inline static _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>> new_(std::optional<std::string_view> code, std::optional<std::string_view> reason);

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> reason_code() const;

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> reason() const;

    inline const _native::capi::TaxExemption* AsFFI() const;
    inline _native::capi::TaxExemption* AsFFI();
    inline static const _native::TaxExemption* FromFFI(const _native::capi::TaxExemption* ptr);
    inline static _native::TaxExemption* FromFFI(_native::capi::TaxExemption* ptr);
    inline static void operator delete(void* ptr);
private:
    TaxExemption() = delete;
    TaxExemption(const _native::TaxExemption&) = delete;
    TaxExemption(_native::TaxExemption&&) noexcept = delete;
    TaxExemption operator=(const _native::TaxExemption&) = delete;
    TaxExemption operator=(_native::TaxExemption&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_TaxExemption_D_HPP

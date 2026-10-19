#ifndef _NATIVE_Adjustment_D_HPP
#define _NATIVE_Adjustment_D_HPP

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
namespace capi { struct Adjustment; }
class Adjustment;
namespace capi { struct BindingError; }
class BindingError;
namespace capi { struct Text; }
class Text;
} // namespace _native



namespace _native {
namespace capi {
    struct Adjustment;
} // namespace capi
} // namespace

namespace _native {
class Adjustment {
public:

  inline static _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>> amount(std::string_view value);

  inline static _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>> percentage(std::string_view base, std::string_view percent);

  inline _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>> with_supplied_amount(std::string_view value) const;

  inline _native::diplomat::result<std::unique_ptr<_native::Adjustment>, std::unique_ptr<_native::BindingError>> with_reason(std::optional<std::string_view> reason, std::optional<std::string_view> code) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> value() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> value_write(W& writeable_output) const;

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> base_amount() const;

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> percent() const;

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> reason() const;

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> reason_code() const;

    inline const _native::capi::Adjustment* AsFFI() const;
    inline _native::capi::Adjustment* AsFFI();
    inline static const _native::Adjustment* FromFFI(const _native::capi::Adjustment* ptr);
    inline static _native::Adjustment* FromFFI(_native::capi::Adjustment* ptr);
    inline static void operator delete(void* ptr);
private:
    Adjustment() = delete;
    Adjustment(const _native::Adjustment&) = delete;
    Adjustment(_native::Adjustment&&) noexcept = delete;
    Adjustment operator=(const _native::Adjustment&) = delete;
    Adjustment operator=(_native::Adjustment&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_Adjustment_D_HPP

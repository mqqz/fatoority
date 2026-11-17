#ifndef _NATIVE_ZatcaRuleSource_D_HPP
#define _NATIVE_ZatcaRuleSource_D_HPP

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
} // namespace _native



namespace _native {
namespace capi {
    struct ZatcaRuleSource;
} // namespace capi
} // namespace

namespace _native {
/**
 * Pinned upstream rule source of a business-rule stage.
 */
class ZatcaRuleSource {
public:

  /**
   * File name of the upstream rule stylesheet.
   */
  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> stylesheet() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> stylesheet_write(W& writeable_output) const;

  /**
   * Hex SHA-256 digest of the pinned stylesheet.
   */
  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> sha256() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> sha256_write(W& writeable_output) const;

    inline const _native::capi::ZatcaRuleSource* AsFFI() const;
    inline _native::capi::ZatcaRuleSource* AsFFI();
    inline static const _native::ZatcaRuleSource* FromFFI(const _native::capi::ZatcaRuleSource* ptr);
    inline static _native::ZatcaRuleSource* FromFFI(_native::capi::ZatcaRuleSource* ptr);
    inline static void operator delete(void* ptr);
private:
    ZatcaRuleSource() = delete;
    ZatcaRuleSource(const _native::ZatcaRuleSource&) = delete;
    ZatcaRuleSource(_native::ZatcaRuleSource&&) noexcept = delete;
    ZatcaRuleSource operator=(const _native::ZatcaRuleSource&) = delete;
    ZatcaRuleSource operator=(_native::ZatcaRuleSource&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_ZatcaRuleSource_D_HPP

#ifndef fatoora_ZatcaRuleSource_D_HPP
#define fatoora_ZatcaRuleSource_D_HPP

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
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct ZatcaRuleSource;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * Pinned upstream rule source of a business-rule stage.
 */
class ZatcaRuleSource {
public:

  /**
   * File name of the upstream rule stylesheet.
   */
  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> stylesheet() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> stylesheet_write(W& writeable_output) const;

  /**
   * Hex SHA-256 digest of the pinned stylesheet.
   */
  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> sha256() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> sha256_write(W& writeable_output) const;

    inline const fatoora::capi::ZatcaRuleSource* AsFFI() const;
    inline fatoora::capi::ZatcaRuleSource* AsFFI();
    inline static const fatoora::ZatcaRuleSource* FromFFI(const fatoora::capi::ZatcaRuleSource* ptr);
    inline static fatoora::ZatcaRuleSource* FromFFI(fatoora::capi::ZatcaRuleSource* ptr);
    inline static void operator delete(void* ptr);
private:
    ZatcaRuleSource() = delete;
    ZatcaRuleSource(const fatoora::ZatcaRuleSource&) = delete;
    ZatcaRuleSource(fatoora::ZatcaRuleSource&&) noexcept = delete;
    ZatcaRuleSource operator=(const fatoora::ZatcaRuleSource&) = delete;
    ZatcaRuleSource operator=(fatoora::ZatcaRuleSource&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_ZatcaRuleSource_D_HPP

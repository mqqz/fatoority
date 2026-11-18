#ifndef _NATIVE_ZatcaStageReport_D_HPP
#define _NATIVE_ZatcaStageReport_D_HPP

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
namespace capi { struct ZatcaFinding; }
class ZatcaFinding;
namespace capi { struct ZatcaRuleSource; }
class ZatcaRuleSource;
class ZatcaStage;
class ZatcaStageStatus;
} // namespace _native



namespace _native {
namespace capi {
    struct ZatcaStageReport;
} // namespace capi
} // namespace

namespace _native {
/**
 * Coverage and findings from one validation stage.
 */
class ZatcaStageReport {
public:

  inline _native::ZatcaStage stage() const;

  inline _native::ZatcaStageStatus status() const;

  /**
   * Pinned rule source, for the CEN and KSA business-rule stages.
   */
  inline _native::diplomat::result<_native::diplomat::maybe_null<std::unique_ptr<_native::ZatcaRuleSource>>, std::unique_ptr<_native::BindingError>> provenance() const;

  inline size_t evaluated_assertions_len() const;

  /**
   * Assertion site at `index` whose selected contexts were all evaluated.
   */
  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> evaluated_assertion(size_t index) const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> evaluated_assertion_write(size_t index, W& writeable_output) const;

  inline size_t findings_len() const;

  inline _native::diplomat::result<std::unique_ptr<_native::ZatcaFinding>, std::unique_ptr<_native::BindingError>> finding(size_t index) const;

    inline const _native::capi::ZatcaStageReport* AsFFI() const;
    inline _native::capi::ZatcaStageReport* AsFFI();
    inline static const _native::ZatcaStageReport* FromFFI(const _native::capi::ZatcaStageReport* ptr);
    inline static _native::ZatcaStageReport* FromFFI(_native::capi::ZatcaStageReport* ptr);
    inline static void operator delete(void* ptr);
private:
    ZatcaStageReport() = delete;
    ZatcaStageReport(const _native::ZatcaStageReport&) = delete;
    ZatcaStageReport(_native::ZatcaStageReport&&) noexcept = delete;
    ZatcaStageReport operator=(const _native::ZatcaStageReport&) = delete;
    ZatcaStageReport operator=(_native::ZatcaStageReport&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_ZatcaStageReport_D_HPP

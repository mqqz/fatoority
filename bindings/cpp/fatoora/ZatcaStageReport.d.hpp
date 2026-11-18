#ifndef fatoora_ZatcaStageReport_D_HPP
#define fatoora_ZatcaStageReport_D_HPP

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
namespace capi { struct ZatcaFinding; }
class ZatcaFinding;
namespace capi { struct ZatcaRuleSource; }
class ZatcaRuleSource;
class ZatcaStage;
class ZatcaStageStatus;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct ZatcaStageReport;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * Coverage and findings from one validation stage.
 */
class ZatcaStageReport {
public:

  inline fatoora::ZatcaStage stage() const;

  inline fatoora::ZatcaStageStatus status() const;

  /**
   * Pinned rule source, for the CEN and KSA business-rule stages.
   */
  inline diplomat::result<diplomat::maybe_null<std::unique_ptr<fatoora::ZatcaRuleSource>>, std::unique_ptr<fatoora::BindingError>> provenance() const;

  inline size_t evaluated_assertions_len() const;

  /**
   * Assertion site at `index` whose selected contexts were all evaluated.
   */
  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> evaluated_assertion(size_t index) const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> evaluated_assertion_write(size_t index, W& writeable_output) const;

  inline size_t findings_len() const;

  inline diplomat::result<std::unique_ptr<fatoora::ZatcaFinding>, std::unique_ptr<fatoora::BindingError>> finding(size_t index) const;

    inline const fatoora::capi::ZatcaStageReport* AsFFI() const;
    inline fatoora::capi::ZatcaStageReport* AsFFI();
    inline static const fatoora::ZatcaStageReport* FromFFI(const fatoora::capi::ZatcaStageReport* ptr);
    inline static fatoora::ZatcaStageReport* FromFFI(fatoora::capi::ZatcaStageReport* ptr);
    inline static void operator delete(void* ptr);
private:
    ZatcaStageReport() = delete;
    ZatcaStageReport(const fatoora::ZatcaStageReport&) = delete;
    ZatcaStageReport(fatoora::ZatcaStageReport&&) noexcept = delete;
    ZatcaStageReport operator=(const fatoora::ZatcaStageReport&) = delete;
    ZatcaStageReport operator=(fatoora::ZatcaStageReport&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_ZatcaStageReport_D_HPP

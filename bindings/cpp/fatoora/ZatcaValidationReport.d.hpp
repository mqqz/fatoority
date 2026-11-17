#ifndef fatoora_ZatcaValidationReport_D_HPP
#define fatoora_ZatcaValidationReport_D_HPP

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
namespace capi { struct ZatcaStageReport; }
class ZatcaStageReport;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct ZatcaValidationReport;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * Owned results of local ZATCA validation with explicit stage coverage.
 *
 * Completion and validity describe only the local profile; they do not
 * establish acceptance by ZATCA or certificate issuer trust.
 */
class ZatcaValidationReport {
public:

  /**
   * Version of the report's serialized format.
   */
  inline uint32_t schema_version() const;

  /**
   * Identifier of the validation profile (the ZATCA SDK release it mirrors).
   */
  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> profile() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> profile_write(W& writeable_output) const;

  /**
   * RFC 3339 instant used for time-dependent rules.
   */
  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> evaluated_at() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> evaluated_at_write(W& writeable_output) const;

  /**
   * Whether every required stage finished exactly once.
   */
  inline bool is_complete() const;

  /**
   * Whether any stage, including an incomplete stage, reported an error.
   */
  inline bool has_errors() const;

  /**
   * Whether validation is complete and contains no error findings.
   */
  inline bool is_valid() const;

  inline size_t stages_len() const;

  /**
   * The stage report at `index`, in stage order.
   */
  inline diplomat::result<std::unique_ptr<fatoora::ZatcaStageReport>, std::unique_ptr<fatoora::BindingError>> stage(size_t index) const;

  /**
   * The report serialized as JSON, including the derived `is_complete`,
   * `has_errors` and `is_valid` fields.
   */
  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> to_json_write(W& writeable_output) const;

    inline const fatoora::capi::ZatcaValidationReport* AsFFI() const;
    inline fatoora::capi::ZatcaValidationReport* AsFFI();
    inline static const fatoora::ZatcaValidationReport* FromFFI(const fatoora::capi::ZatcaValidationReport* ptr);
    inline static fatoora::ZatcaValidationReport* FromFFI(fatoora::capi::ZatcaValidationReport* ptr);
    inline static void operator delete(void* ptr);
private:
    ZatcaValidationReport() = delete;
    ZatcaValidationReport(const fatoora::ZatcaValidationReport&) = delete;
    ZatcaValidationReport(fatoora::ZatcaValidationReport&&) noexcept = delete;
    ZatcaValidationReport operator=(const fatoora::ZatcaValidationReport&) = delete;
    ZatcaValidationReport operator=(fatoora::ZatcaValidationReport&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_ZatcaValidationReport_D_HPP

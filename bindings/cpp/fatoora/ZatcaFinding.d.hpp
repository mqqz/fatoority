#ifndef fatoora_ZatcaFinding_D_HPP
#define fatoora_ZatcaFinding_D_HPP

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
class Severity;
class ValidationLayer;
class ValidationLocationKind;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct ZatcaFinding;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * One validation finding with its source assertion, when applicable.
 */
class ZatcaFinding {
public:

  /**
   * Rule assertion that produced the finding, for business-rule findings.
   */
  inline std::optional<std::string> assertion_site() const;
  template<typename W>
  inline std::optional<std::monostate> assertion_site_write(W& writeable_output) const;

  inline fatoora::ValidationLayer layer() const;

  /**
   * Library-owned rule identifier, independent of message wording.
   */
  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> code() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> code_write(W& writeable_output) const;

  inline fatoora::Severity severity() const;

  /**
   * Human-readable text; the wording is not a stable API.
   */
  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> message() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> message_write(W& writeable_output) const;

  /**
   * The kind of location, or null when the source cannot locate the finding.
   */
  inline std::optional<fatoora::ValidationLocationKind> location_kind() const;

  /**
   * The field path or XPath, or null for other location kinds.
   */
  inline std::optional<std::string> location_path() const;
  template<typename W>
  inline std::optional<std::monostate> location_path_write(W& writeable_output) const;

  /**
   * One-based XML line, or null when the location is not XML coordinates.
   */
  inline std::optional<uint32_t> location_line() const;

  /**
   * One-based XML column, or null when unknown.
   */
  inline std::optional<uint32_t> location_column() const;

    inline const fatoora::capi::ZatcaFinding* AsFFI() const;
    inline fatoora::capi::ZatcaFinding* AsFFI();
    inline static const fatoora::ZatcaFinding* FromFFI(const fatoora::capi::ZatcaFinding* ptr);
    inline static fatoora::ZatcaFinding* FromFFI(fatoora::capi::ZatcaFinding* ptr);
    inline static void operator delete(void* ptr);
private:
    ZatcaFinding() = delete;
    ZatcaFinding(const fatoora::ZatcaFinding&) = delete;
    ZatcaFinding(fatoora::ZatcaFinding&&) noexcept = delete;
    ZatcaFinding operator=(const fatoora::ZatcaFinding&) = delete;
    ZatcaFinding operator=(fatoora::ZatcaFinding&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_ZatcaFinding_D_HPP

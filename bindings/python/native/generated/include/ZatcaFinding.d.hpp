#ifndef _NATIVE_ZatcaFinding_D_HPP
#define _NATIVE_ZatcaFinding_D_HPP

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
class Severity;
class ValidationLayer;
class ValidationLocationKind;
} // namespace _native



namespace _native {
namespace capi {
    struct ZatcaFinding;
} // namespace capi
} // namespace

namespace _native {
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

  inline _native::ValidationLayer layer() const;

  /**
   * Library-owned rule identifier, independent of message wording.
   */
  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> code() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> code_write(W& writeable_output) const;

  inline _native::Severity severity() const;

  /**
   * Human-readable text; the wording is not a stable API.
   */
  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> message() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> message_write(W& writeable_output) const;

  /**
   * The kind of location, or null when the source cannot locate the finding.
   */
  inline std::optional<_native::ValidationLocationKind> location_kind() const;

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

    inline const _native::capi::ZatcaFinding* AsFFI() const;
    inline _native::capi::ZatcaFinding* AsFFI();
    inline static const _native::ZatcaFinding* FromFFI(const _native::capi::ZatcaFinding* ptr);
    inline static _native::ZatcaFinding* FromFFI(_native::capi::ZatcaFinding* ptr);
    inline static void operator delete(void* ptr);
private:
    ZatcaFinding() = delete;
    ZatcaFinding(const _native::ZatcaFinding&) = delete;
    ZatcaFinding(_native::ZatcaFinding&&) noexcept = delete;
    ZatcaFinding operator=(const _native::ZatcaFinding&) = delete;
    ZatcaFinding operator=(_native::ZatcaFinding&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_ZatcaFinding_D_HPP

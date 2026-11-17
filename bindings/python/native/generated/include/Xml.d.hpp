#ifndef _NATIVE_Xml_D_HPP
#define _NATIVE_Xml_D_HPP

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
namespace capi { struct ZatcaValidationReport; }
class ZatcaValidationReport;
} // namespace _native



namespace _native {
namespace capi {
    struct Xml;
} // namespace capi
} // namespace

namespace _native {
class Xml {
public:

  /**
   * Run the local ZATCA validation profile and return its owned report.
   *
   * Inspect `is_valid` before acceptance. Options default when absent;
   * `options_json` is limited to 4 KiB. Validation needs no configuration:
   * it mirrors the stateless core `Validator::validate_zatca`.
   */
  inline static _native::diplomat::result<std::unique_ptr<_native::ZatcaValidationReport>, std::unique_ptr<_native::BindingError>> validate_zatca(std::string_view xml, std::optional<std::string_view> options_json);

  /**
   * Check the XML against the UBL schema and business rules, mirroring core
   * `Validator::validate_xml`. Returns true or a validation error.
   */
  inline static _native::diplomat::result<bool, std::unique_ptr<_native::BindingError>> validate(std::string_view xml);

  inline static _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> hash(std::string_view xml);
  template<typename W>
  inline static _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> hash_write(std::string_view xml, W& writeable_output);

    inline const _native::capi::Xml* AsFFI() const;
    inline _native::capi::Xml* AsFFI();
    inline static const _native::Xml* FromFFI(const _native::capi::Xml* ptr);
    inline static _native::Xml* FromFFI(_native::capi::Xml* ptr);
    inline static void operator delete(void* ptr);
private:
    Xml() = delete;
    Xml(const _native::Xml&) = delete;
    Xml(_native::Xml&&) noexcept = delete;
    Xml operator=(const _native::Xml&) = delete;
    Xml operator=(_native::Xml&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_Xml_D_HPP

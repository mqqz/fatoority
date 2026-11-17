#ifndef fatoora_Xml_D_HPP
#define fatoora_Xml_D_HPP

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
namespace capi { struct ZatcaValidationReport; }
class ZatcaValidationReport;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct Xml;
} // namespace capi
} // namespace

namespace fatoora {
class Xml {
public:

  /**
   * Run the local ZATCA validation profile and return its owned report.
   *
   * Inspect `is_valid` before acceptance. Options default when absent;
   * `options_json` is limited to 4 KiB. Validation needs no configuration:
   * it mirrors the stateless core `Validator::validate_zatca`.
   */
  inline static diplomat::result<std::unique_ptr<fatoora::ZatcaValidationReport>, std::unique_ptr<fatoora::BindingError>> validate_zatca(std::string_view xml, std::optional<std::string_view> options_json);

  /**
   * Check the XML against the UBL schema and business rules, mirroring core
   * `Validator::validate_xml`. Returns true or a validation error.
   */
  inline static diplomat::result<bool, std::unique_ptr<fatoora::BindingError>> validate(std::string_view xml);

  inline static diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> hash(std::string_view xml);
  template<typename W>
  inline static diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> hash_write(std::string_view xml, W& writeable_output);

    inline const fatoora::capi::Xml* AsFFI() const;
    inline fatoora::capi::Xml* AsFFI();
    inline static const fatoora::Xml* FromFFI(const fatoora::capi::Xml* ptr);
    inline static fatoora::Xml* FromFFI(fatoora::capi::Xml* ptr);
    inline static void operator delete(void* ptr);
private:
    Xml() = delete;
    Xml(const fatoora::Xml&) = delete;
    Xml(fatoora::Xml&&) noexcept = delete;
    Xml operator=(const fatoora::Xml&) = delete;
    Xml operator=(fatoora::Xml&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_Xml_D_HPP

#ifndef fatoora_CsrProperties_D_HPP
#define fatoora_CsrProperties_D_HPP

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
namespace capi { struct Csr; }
class Csr;
namespace capi { struct CsrProperties; }
class CsrProperties;
namespace capi { struct SigningKey; }
class SigningKey;
class Environment;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct CsrProperties;
} // namespace capi
} // namespace

namespace fatoora {
class CsrProperties {
public:

  inline static diplomat::result<std::unique_ptr<fatoora::CsrProperties>, std::unique_ptr<fatoora::BindingError>> new_(std::string_view common_name, std::string_view serial_number, std::string_view organization_identifier, std::string_view organization_unit_name, std::string_view organization_name, std::string_view country_name, std::string_view invoice_type, std::string_view location_address, std::string_view industry_business_category);

  /**
   * Parse `csr.*` properties text, mirroring core `CsrProperties::from_str`.
   */
  inline static diplomat::result<std::unique_ptr<fatoora::CsrProperties>, std::unique_ptr<fatoora::BindingError>> from_str(std::string_view properties);

  /**
   * Load a `csr.*` properties file, mirroring core `CsrProperties::from_file`.
   */
  inline static diplomat::result<std::unique_ptr<fatoora::CsrProperties>, std::unique_ptr<fatoora::BindingError>> from_file(std::string_view path);

  inline diplomat::result<std::unique_ptr<fatoora::Csr>, std::unique_ptr<fatoora::BindingError>> build(const fatoora::SigningKey& key, fatoora::Environment env) const;

    inline const fatoora::capi::CsrProperties* AsFFI() const;
    inline fatoora::capi::CsrProperties* AsFFI();
    inline static const fatoora::CsrProperties* FromFFI(const fatoora::capi::CsrProperties* ptr);
    inline static fatoora::CsrProperties* FromFFI(fatoora::capi::CsrProperties* ptr);
    inline static void operator delete(void* ptr);
private:
    CsrProperties() = delete;
    CsrProperties(const fatoora::CsrProperties&) = delete;
    CsrProperties(fatoora::CsrProperties&&) noexcept = delete;
    CsrProperties operator=(const fatoora::CsrProperties&) = delete;
    CsrProperties operator=(fatoora::CsrProperties&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_CsrProperties_D_HPP

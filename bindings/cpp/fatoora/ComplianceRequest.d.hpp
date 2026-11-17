#ifndef fatoora_ComplianceRequest_D_HPP
#define fatoora_ComplianceRequest_D_HPP

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
namespace capi { struct Bytes; }
class Bytes;
namespace capi { struct ComplianceCsid; }
class ComplianceCsid;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct ComplianceRequest;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * Owned request and decoding context; contains credentials.
 */
class ComplianceRequest {
public:

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> method() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> method_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> url() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> url_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> headers_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> headers_json_write(W& writeable_output) const;

  inline diplomat::result<std::unique_ptr<fatoora::Bytes>, std::unique_ptr<fatoora::BindingError>> body() const;

  inline diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>> decode(uint16_t status, diplomat::span<const uint8_t> body) const;

    inline const fatoora::capi::ComplianceRequest* AsFFI() const;
    inline fatoora::capi::ComplianceRequest* AsFFI();
    inline static const fatoora::ComplianceRequest* FromFFI(const fatoora::capi::ComplianceRequest* ptr);
    inline static fatoora::ComplianceRequest* FromFFI(fatoora::capi::ComplianceRequest* ptr);
    inline static void operator delete(void* ptr);
private:
    ComplianceRequest() = delete;
    ComplianceRequest(const fatoora::ComplianceRequest&) = delete;
    ComplianceRequest(fatoora::ComplianceRequest&&) noexcept = delete;
    ComplianceRequest operator=(const fatoora::ComplianceRequest&) = delete;
    ComplianceRequest operator=(fatoora::ComplianceRequest&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_ComplianceRequest_D_HPP

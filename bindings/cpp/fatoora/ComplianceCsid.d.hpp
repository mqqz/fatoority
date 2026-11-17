#ifndef fatoora_ComplianceCsid_D_HPP
#define fatoora_ComplianceCsid_D_HPP

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
namespace capi { struct ComplianceCsid; }
class ComplianceCsid;
namespace capi { struct Text; }
class Text;
class Environment;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct ComplianceCsid;
} // namespace capi
} // namespace

namespace fatoora {
class ComplianceCsid {
public:

  inline static diplomat::result<std::unique_ptr<fatoora::ComplianceCsid>, std::unique_ptr<fatoora::BindingError>> new_(fatoora::Environment environment, std::optional<std::string_view> request_id, std::string_view token, std::string_view secret);

  inline diplomat::result<fatoora::Environment, std::unique_ptr<fatoora::BindingError>> env() const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> request_id() const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> binary_security_token() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> binary_security_token_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> secret() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> secret_write(W& writeable_output) const;

    inline const fatoora::capi::ComplianceCsid* AsFFI() const;
    inline fatoora::capi::ComplianceCsid* AsFFI();
    inline static const fatoora::ComplianceCsid* FromFFI(const fatoora::capi::ComplianceCsid* ptr);
    inline static fatoora::ComplianceCsid* FromFFI(fatoora::capi::ComplianceCsid* ptr);
    inline static void operator delete(void* ptr);
private:
    ComplianceCsid() = delete;
    ComplianceCsid(const fatoora::ComplianceCsid&) = delete;
    ComplianceCsid(fatoora::ComplianceCsid&&) noexcept = delete;
    ComplianceCsid operator=(const fatoora::ComplianceCsid&) = delete;
    ComplianceCsid operator=(fatoora::ComplianceCsid&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_ComplianceCsid_D_HPP

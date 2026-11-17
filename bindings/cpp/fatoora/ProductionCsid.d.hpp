#ifndef fatoora_ProductionCsid_D_HPP
#define fatoora_ProductionCsid_D_HPP

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
namespace capi { struct ProductionCsid; }
class ProductionCsid;
namespace capi { struct Text; }
class Text;
class Environment;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct ProductionCsid;
} // namespace capi
} // namespace

namespace fatoora {
class ProductionCsid {
public:

  inline static diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>> create(fatoora::Environment environment, std::optional<std::string_view> request_id, std::string_view token, std::string_view secret);

  inline diplomat::result<fatoora::Environment, std::unique_ptr<fatoora::BindingError>> env() const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> request_id() const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> binary_security_token() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> binary_security_token_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> secret() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> secret_write(W& writeable_output) const;

    inline const fatoora::capi::ProductionCsid* AsFFI() const;
    inline fatoora::capi::ProductionCsid* AsFFI();
    inline static const fatoora::ProductionCsid* FromFFI(const fatoora::capi::ProductionCsid* ptr);
    inline static fatoora::ProductionCsid* FromFFI(fatoora::capi::ProductionCsid* ptr);
    inline static void operator delete(void* ptr);
private:
    ProductionCsid() = delete;
    ProductionCsid(const fatoora::ProductionCsid&) = delete;
    ProductionCsid(fatoora::ProductionCsid&&) noexcept = delete;
    ProductionCsid operator=(const fatoora::ProductionCsid&) = delete;
    ProductionCsid operator=(fatoora::ProductionCsid&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_ProductionCsid_D_HPP

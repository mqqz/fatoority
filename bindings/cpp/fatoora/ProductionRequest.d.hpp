#ifndef fatoora_ProductionRequest_D_HPP
#define fatoora_ProductionRequest_D_HPP

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
namespace capi { struct ProductionCsid; }
class ProductionCsid;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct ProductionRequest;
} // namespace capi
} // namespace

namespace fatoora {
/**
 * Owned request and decoding context; contains credentials.
 */
class ProductionRequest {
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

  inline diplomat::result<std::unique_ptr<fatoora::ProductionCsid>, std::unique_ptr<fatoora::BindingError>> decode(uint16_t status, diplomat::span<const uint8_t> body) const;

    inline const fatoora::capi::ProductionRequest* AsFFI() const;
    inline fatoora::capi::ProductionRequest* AsFFI();
    inline static const fatoora::ProductionRequest* FromFFI(const fatoora::capi::ProductionRequest* ptr);
    inline static fatoora::ProductionRequest* FromFFI(fatoora::capi::ProductionRequest* ptr);
    inline static void operator delete(void* ptr);
private:
    ProductionRequest() = delete;
    ProductionRequest(const fatoora::ProductionRequest&) = delete;
    ProductionRequest(fatoora::ProductionRequest&&) noexcept = delete;
    ProductionRequest operator=(const fatoora::ProductionRequest&) = delete;
    ProductionRequest operator=(fatoora::ProductionRequest&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_ProductionRequest_D_HPP

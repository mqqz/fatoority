#ifndef _NATIVE_ProductionRequest_D_HPP
#define _NATIVE_ProductionRequest_D_HPP

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
namespace capi { struct Bytes; }
class Bytes;
namespace capi { struct ProductionCsid; }
class ProductionCsid;
} // namespace _native



namespace _native {
namespace capi {
    struct ProductionRequest;
} // namespace capi
} // namespace

namespace _native {
/**
 * Owned request and decoding context; contains credentials.
 */
class ProductionRequest {
public:

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> method() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> method_write(W& writeable_output) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> url() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> url_write(W& writeable_output) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> headers_json() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> headers_json_write(W& writeable_output) const;

  inline _native::diplomat::result<std::unique_ptr<_native::Bytes>, std::unique_ptr<_native::BindingError>> body() const;

  inline _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>> decode(uint16_t status, _native::diplomat::span<const uint8_t> body) const;

    inline const _native::capi::ProductionRequest* AsFFI() const;
    inline _native::capi::ProductionRequest* AsFFI();
    inline static const _native::ProductionRequest* FromFFI(const _native::capi::ProductionRequest* ptr);
    inline static _native::ProductionRequest* FromFFI(_native::capi::ProductionRequest* ptr);
    inline static void operator delete(void* ptr);
private:
    ProductionRequest() = delete;
    ProductionRequest(const _native::ProductionRequest&) = delete;
    ProductionRequest(_native::ProductionRequest&&) noexcept = delete;
    ProductionRequest operator=(const _native::ProductionRequest&) = delete;
    ProductionRequest operator=(_native::ProductionRequest&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_ProductionRequest_D_HPP

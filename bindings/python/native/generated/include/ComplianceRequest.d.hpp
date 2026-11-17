#ifndef _NATIVE_ComplianceRequest_D_HPP
#define _NATIVE_ComplianceRequest_D_HPP

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
namespace capi { struct ComplianceCsid; }
class ComplianceCsid;
} // namespace _native



namespace _native {
namespace capi {
    struct ComplianceRequest;
} // namespace capi
} // namespace

namespace _native {
/**
 * Owned request and decoding context; contains credentials.
 */
class ComplianceRequest {
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

  inline _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>> decode(uint16_t status, _native::diplomat::span<const uint8_t> body) const;

    inline const _native::capi::ComplianceRequest* AsFFI() const;
    inline _native::capi::ComplianceRequest* AsFFI();
    inline static const _native::ComplianceRequest* FromFFI(const _native::capi::ComplianceRequest* ptr);
    inline static _native::ComplianceRequest* FromFFI(_native::capi::ComplianceRequest* ptr);
    inline static void operator delete(void* ptr);
private:
    ComplianceRequest() = delete;
    ComplianceRequest(const _native::ComplianceRequest&) = delete;
    ComplianceRequest(_native::ComplianceRequest&&) noexcept = delete;
    ComplianceRequest operator=(const _native::ComplianceRequest&) = delete;
    ComplianceRequest operator=(_native::ComplianceRequest&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_ComplianceRequest_D_HPP

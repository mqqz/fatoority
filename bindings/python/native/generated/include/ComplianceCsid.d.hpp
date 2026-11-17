#ifndef _NATIVE_ComplianceCsid_D_HPP
#define _NATIVE_ComplianceCsid_D_HPP

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
namespace capi { struct ComplianceCsid; }
class ComplianceCsid;
namespace capi { struct Text; }
class Text;
class Environment;
} // namespace _native



namespace _native {
namespace capi {
    struct ComplianceCsid;
} // namespace capi
} // namespace

namespace _native {
class ComplianceCsid {
public:

  inline static _native::diplomat::result<std::unique_ptr<_native::ComplianceCsid>, std::unique_ptr<_native::BindingError>> new_(_native::Environment environment, std::optional<std::string_view> request_id, std::string_view token, std::string_view secret);

  inline _native::diplomat::result<_native::Environment, std::unique_ptr<_native::BindingError>> env() const;

  inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> request_id() const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> binary_security_token() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> binary_security_token_write(W& writeable_output) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> secret() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> secret_write(W& writeable_output) const;

    inline const _native::capi::ComplianceCsid* AsFFI() const;
    inline _native::capi::ComplianceCsid* AsFFI();
    inline static const _native::ComplianceCsid* FromFFI(const _native::capi::ComplianceCsid* ptr);
    inline static _native::ComplianceCsid* FromFFI(_native::capi::ComplianceCsid* ptr);
    inline static void operator delete(void* ptr);
private:
    ComplianceCsid() = delete;
    ComplianceCsid(const _native::ComplianceCsid&) = delete;
    ComplianceCsid(_native::ComplianceCsid&&) noexcept = delete;
    ComplianceCsid operator=(const _native::ComplianceCsid&) = delete;
    ComplianceCsid operator=(_native::ComplianceCsid&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_ComplianceCsid_D_HPP

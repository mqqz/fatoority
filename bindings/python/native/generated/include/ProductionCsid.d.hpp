#ifndef _NATIVE_ProductionCsid_D_HPP
#define _NATIVE_ProductionCsid_D_HPP

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
namespace capi { struct ProductionCsid; }
class ProductionCsid;
class Environment;
} // namespace _native



namespace _native {
namespace capi {
    struct ProductionCsid;
} // namespace capi
} // namespace

namespace _native {
class ProductionCsid {
public:

  inline static _native::diplomat::result<std::unique_ptr<_native::ProductionCsid>, std::unique_ptr<_native::BindingError>> new_(_native::Environment environment, std::optional<std::string_view> request_id, std::string_view token, std::string_view secret);

  inline _native::diplomat::result<_native::Environment, std::unique_ptr<_native::BindingError>> env() const;

  inline std::optional<std::string> request_id() const;
  template<typename W>
  inline std::optional<std::monostate> request_id_write(W& writeable_output) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> binary_security_token() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> binary_security_token_write(W& writeable_output) const;

  inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> secret() const;
  template<typename W>
  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> secret_write(W& writeable_output) const;

    inline const _native::capi::ProductionCsid* AsFFI() const;
    inline _native::capi::ProductionCsid* AsFFI();
    inline static const _native::ProductionCsid* FromFFI(const _native::capi::ProductionCsid* ptr);
    inline static _native::ProductionCsid* FromFFI(_native::capi::ProductionCsid* ptr);
    inline static void operator delete(void* ptr);
private:
    ProductionCsid() = delete;
    ProductionCsid(const _native::ProductionCsid&) = delete;
    ProductionCsid(_native::ProductionCsid&&) noexcept = delete;
    ProductionCsid operator=(const _native::ProductionCsid&) = delete;
    ProductionCsid operator=(_native::ProductionCsid&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_ProductionCsid_D_HPP

#ifndef fatoora_OtherId_D_HPP
#define fatoora_OtherId_D_HPP

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
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct OtherId;
} // namespace capi
} // namespace

namespace fatoora {
class OtherId {
public:

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> value() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> value_write(W& writeable_output) const;

  inline std::optional<std::string> scheme() const;
  template<typename W>
  inline std::optional<std::monostate> scheme_write(W& writeable_output) const;

    inline const fatoora::capi::OtherId* AsFFI() const;
    inline fatoora::capi::OtherId* AsFFI();
    inline static const fatoora::OtherId* FromFFI(const fatoora::capi::OtherId* ptr);
    inline static fatoora::OtherId* FromFFI(fatoora::capi::OtherId* ptr);
    inline static void operator delete(void* ptr);
private:
    OtherId() = delete;
    OtherId(const fatoora::OtherId&) = delete;
    OtherId(fatoora::OtherId&&) noexcept = delete;
    OtherId operator=(const fatoora::OtherId&) = delete;
    OtherId operator=(fatoora::OtherId&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_OtherId_D_HPP

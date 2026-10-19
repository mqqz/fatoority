#ifndef fatoora_Adjustment_D_HPP
#define fatoora_Adjustment_D_HPP

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
namespace capi { struct Adjustment; }
class Adjustment;
namespace capi { struct BindingError; }
class BindingError;
namespace capi { struct Text; }
class Text;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct Adjustment;
} // namespace capi
} // namespace

namespace fatoora {
class Adjustment {
public:

  inline static diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>> amount(std::string_view value);

  inline static diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>> percentage(std::string_view base, std::string_view percent);

  inline diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>> with_supplied_amount(std::string_view value) const;

  inline diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>> with_reason(std::optional<std::string_view> reason, std::optional<std::string_view> code) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> value() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> value_write(W& writeable_output) const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> base_amount() const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> percent() const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> reason() const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> reason_code() const;

    inline const fatoora::capi::Adjustment* AsFFI() const;
    inline fatoora::capi::Adjustment* AsFFI();
    inline static const fatoora::Adjustment* FromFFI(const fatoora::capi::Adjustment* ptr);
    inline static fatoora::Adjustment* FromFFI(fatoora::capi::Adjustment* ptr);
    inline static void operator delete(void* ptr);
private:
    Adjustment() = delete;
    Adjustment(const fatoora::Adjustment&) = delete;
    Adjustment(fatoora::Adjustment&&) noexcept = delete;
    Adjustment operator=(const fatoora::Adjustment&) = delete;
    Adjustment operator=(fatoora::Adjustment&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_Adjustment_D_HPP

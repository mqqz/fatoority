#ifndef fatoora_TaxExemption_D_HPP
#define fatoora_TaxExemption_D_HPP

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
namespace capi { struct TaxExemption; }
class TaxExemption;
namespace capi { struct Text; }
class Text;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct TaxExemption;
} // namespace capi
} // namespace

namespace fatoora {
class TaxExemption {
public:

  inline static diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>> new_(std::optional<std::string_view> code, std::optional<std::string_view> reason);

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> reason_code() const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> reason() const;

    inline const fatoora::capi::TaxExemption* AsFFI() const;
    inline fatoora::capi::TaxExemption* AsFFI();
    inline static const fatoora::TaxExemption* FromFFI(const fatoora::capi::TaxExemption* ptr);
    inline static fatoora::TaxExemption* FromFFI(fatoora::capi::TaxExemption* ptr);
    inline static void operator delete(void* ptr);
private:
    TaxExemption() = delete;
    TaxExemption(const fatoora::TaxExemption&) = delete;
    TaxExemption(fatoora::TaxExemption&&) noexcept = delete;
    TaxExemption operator=(const fatoora::TaxExemption&) = delete;
    TaxExemption operator=(fatoora::TaxExemption&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_TaxExemption_D_HPP

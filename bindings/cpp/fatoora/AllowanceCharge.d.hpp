#ifndef fatoora_AllowanceCharge_D_HPP
#define fatoora_AllowanceCharge_D_HPP

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
namespace capi { struct AllowanceCharge; }
class AllowanceCharge;
namespace capi { struct BindingError; }
class BindingError;
namespace capi { struct TaxExemption; }
class TaxExemption;
namespace capi { struct Text; }
class Text;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct AllowanceCharge;
} // namespace capi
} // namespace

namespace fatoora {
class AllowanceCharge {
public:

  inline static diplomat::result<std::unique_ptr<fatoora::AllowanceCharge>, std::unique_ptr<fatoora::BindingError>> new_(bool charge, std::string_view amount, std::optional<std::string_view> reason, std::optional<std::string_view> reason_code, uint8_t category, std::string_view rate, const fatoora::TaxExemption& exemption);

  inline bool is_charge() const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> amount() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> amount_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> rate() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> rate_write(W& writeable_output) const;

  inline uint8_t category() const;

  inline size_t tax_category_count() const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> reason() const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> reason_code() const;

  inline diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>> exemption() const;

    inline const fatoora::capi::AllowanceCharge* AsFFI() const;
    inline fatoora::capi::AllowanceCharge* AsFFI();
    inline static const fatoora::AllowanceCharge* FromFFI(const fatoora::capi::AllowanceCharge* ptr);
    inline static fatoora::AllowanceCharge* FromFFI(fatoora::capi::AllowanceCharge* ptr);
    inline static void operator delete(void* ptr);
private:
    AllowanceCharge() = delete;
    AllowanceCharge(const fatoora::AllowanceCharge&) = delete;
    AllowanceCharge(fatoora::AllowanceCharge&&) noexcept = delete;
    AllowanceCharge operator=(const fatoora::AllowanceCharge&) = delete;
    AllowanceCharge operator=(fatoora::AllowanceCharge&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_AllowanceCharge_D_HPP

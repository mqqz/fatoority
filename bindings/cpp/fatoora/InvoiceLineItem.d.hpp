#ifndef fatoora_InvoiceLineItem_D_HPP
#define fatoora_InvoiceLineItem_D_HPP

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
namespace capi { struct InvoiceLineItem; }
class InvoiceLineItem;
namespace capi { struct OriginalInvoiceRef; }
class OriginalInvoiceRef;
namespace capi { struct TaxExemption; }
class TaxExemption;
namespace capi { struct Text; }
class Text;
namespace capi { struct VatBreakdown; }
class VatBreakdown;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct InvoiceLineItem;
} // namespace capi
} // namespace

namespace fatoora {
class InvoiceLineItem {
public:

  inline diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>> with_document_reference(const fatoora::OriginalInvoiceRef& reference) const;

  inline diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>> with_tax_subtotal(const fatoora::VatBreakdown& subtotal) const;

  inline size_t document_references_len() const;

  inline diplomat::result<std::unique_ptr<fatoora::OriginalInvoiceRef>, std::unique_ptr<fatoora::BindingError>> document_reference(size_t index) const;

  inline size_t tax_subtotals_len() const;

  inline diplomat::result<std::unique_ptr<fatoora::VatBreakdown>, std::unique_ptr<fatoora::BindingError>> tax_subtotal(size_t index) const;

  inline static diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>> new_(std::string_view description, std::string_view quantity, std::string_view unit_code, std::string_view unit_price, std::string_view rate, uint8_t category);

  inline diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>> with_metadata(std::optional<std::string_view> id, const fatoora::TaxExemption& exemption) const;

  inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> id() const;

  inline diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>> exemption() const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> description() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> description_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> unit_code() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> unit_code_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> quantity() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> quantity_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> unit_price() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> unit_price_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> total_amount() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> total_amount_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> vat_rate() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> vat_rate_write(W& writeable_output) const;

  inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> vat_amount() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> vat_amount_write(W& writeable_output) const;

  inline uint8_t vat_category() const;

    inline const fatoora::capi::InvoiceLineItem* AsFFI() const;
    inline fatoora::capi::InvoiceLineItem* AsFFI();
    inline static const fatoora::InvoiceLineItem* FromFFI(const fatoora::capi::InvoiceLineItem* ptr);
    inline static fatoora::InvoiceLineItem* FromFFI(fatoora::capi::InvoiceLineItem* ptr);
    inline static void operator delete(void* ptr);
private:
    InvoiceLineItem() = delete;
    InvoiceLineItem(const fatoora::InvoiceLineItem&) = delete;
    InvoiceLineItem(fatoora::InvoiceLineItem&&) noexcept = delete;
    InvoiceLineItem operator=(const fatoora::InvoiceLineItem&) = delete;
    InvoiceLineItem operator=(fatoora::InvoiceLineItem&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_InvoiceLineItem_D_HPP

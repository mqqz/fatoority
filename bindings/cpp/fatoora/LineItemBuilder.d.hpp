#ifndef fatoora_LineItemBuilder_D_HPP
#define fatoora_LineItemBuilder_D_HPP

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
namespace capi { struct InvoiceLineItem; }
class InvoiceLineItem;
namespace capi { struct LineItemBuilder; }
class LineItemBuilder;
namespace capi { struct OriginalInvoiceRef; }
class OriginalInvoiceRef;
namespace capi { struct TaxExemption; }
class TaxExemption;
namespace capi { struct VatBreakdown; }
class VatBreakdown;
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct LineItemBuilder;
} // namespace capi
} // namespace

namespace fatoora {
class LineItemBuilder {
public:

  inline static std::unique_ptr<fatoora::LineItemBuilder> new_();

  inline diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>> build();

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_item_name(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_invoiced_quantity_unit_code(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_item_price_base_quantity_unit_code(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_id(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_invoiced_quantity(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_item_net_price(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_item_gross_price(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_item_price_discount(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_item_price_base_quantity(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_line_net_amount(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_line_vat_amount(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_line_amount_with_vat(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_vat_rate(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_vat_category(uint8_t value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_exemption(const fatoora::TaxExemption& value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> add_allowance(const fatoora::Adjustment& value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> add_charge(const fatoora::Adjustment& value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> add_document_reference(const fatoora::OriginalInvoiceRef& value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> add_tax_subtotal(const fatoora::VatBreakdown& value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_item_seller_identifier(std::string_view value, std::optional<std::string_view> scheme);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_item_buyer_identifier(std::string_view value, std::optional<std::string_view> scheme);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_item_standard_identifier(std::string_view value, std::optional<std::string_view> scheme);

    inline const fatoora::capi::LineItemBuilder* AsFFI() const;
    inline fatoora::capi::LineItemBuilder* AsFFI();
    inline static const fatoora::LineItemBuilder* FromFFI(const fatoora::capi::LineItemBuilder* ptr);
    inline static fatoora::LineItemBuilder* FromFFI(fatoora::capi::LineItemBuilder* ptr);
    inline static void operator delete(void* ptr);
private:
    LineItemBuilder() = delete;
    LineItemBuilder(const fatoora::LineItemBuilder&) = delete;
    LineItemBuilder(fatoora::LineItemBuilder&&) noexcept = delete;
    LineItemBuilder operator=(const fatoora::LineItemBuilder&) = delete;
    LineItemBuilder operator=(fatoora::LineItemBuilder&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_LineItemBuilder_D_HPP

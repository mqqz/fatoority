#ifndef _NATIVE_LineItemBuilder_D_HPP
#define _NATIVE_LineItemBuilder_D_HPP

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
} // namespace _native



namespace _native {
namespace capi {
    struct LineItemBuilder;
} // namespace capi
} // namespace

namespace _native {
class LineItemBuilder {
public:

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_tax_total_present(bool present);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_gross_amount_present(bool present);

  inline static std::unique_ptr<_native::LineItemBuilder> new_();

  inline _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>> build();

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_item_name(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_invoiced_quantity_unit_code(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_item_price_base_quantity_unit_code(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_id(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_invoiced_quantity(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_item_net_price(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_item_gross_price(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_item_price_discount(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_item_price_base_quantity(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_line_net_amount(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_line_vat_amount(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_line_amount_with_vat(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_vat_rate(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_vat_category(uint8_t value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_exemption(const _native::TaxExemption& value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> add_allowance(const _native::Adjustment& value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> add_charge(const _native::Adjustment& value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> add_document_reference(const _native::OriginalInvoiceRef& value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> add_tax_subtotal(const _native::VatBreakdown& value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_item_seller_identifier(std::string_view value, std::optional<std::string_view> scheme);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_item_buyer_identifier(std::string_view value, std::optional<std::string_view> scheme);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_item_standard_identifier(std::string_view value, std::optional<std::string_view> scheme);

    inline const _native::capi::LineItemBuilder* AsFFI() const;
    inline _native::capi::LineItemBuilder* AsFFI();
    inline static const _native::LineItemBuilder* FromFFI(const _native::capi::LineItemBuilder* ptr);
    inline static _native::LineItemBuilder* FromFFI(_native::capi::LineItemBuilder* ptr);
    inline static void operator delete(void* ptr);
private:
    LineItemBuilder() = delete;
    LineItemBuilder(const _native::LineItemBuilder&) = delete;
    LineItemBuilder(_native::LineItemBuilder&&) noexcept = delete;
    LineItemBuilder operator=(const _native::LineItemBuilder&) = delete;
    LineItemBuilder operator=(_native::LineItemBuilder&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_LineItemBuilder_D_HPP

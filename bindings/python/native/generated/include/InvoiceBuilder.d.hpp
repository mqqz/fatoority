#ifndef _NATIVE_InvoiceBuilder_D_HPP
#define _NATIVE_InvoiceBuilder_D_HPP

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
namespace capi { struct Address; }
class Address;
namespace capi { struct AllowanceCharge; }
class AllowanceCharge;
namespace capi { struct BindingError; }
class BindingError;
namespace capi { struct FinalizedInvoice; }
class FinalizedInvoice;
namespace capi { struct InvoiceBuilder; }
class InvoiceBuilder;
namespace capi { struct InvoiceLineItem; }
class InvoiceLineItem;
namespace capi { struct OriginalInvoiceRef; }
class OriginalInvoiceRef;
namespace capi { struct TaxExemption; }
class TaxExemption;
} // namespace _native



namespace _native {
namespace capi {
    struct InvoiceBuilder;
} // namespace capi
} // namespace

namespace _native {
class InvoiceBuilder {
public:

  /**
   * Construct a credit or debit note with a complete billing reference.
   */
  inline static _native::diplomat::result<std::unique_ptr<_native::InvoiceBuilder>, std::unique_ptr<_native::BindingError>> new_with_reference(uint8_t kind, uint8_t subtype, const _native::OriginalInvoiceRef& reference, std::string_view reason);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_note_without_language(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_payment_instruction(std::string_view instruction);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_prepaid_amount(std::string_view amount);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_payable_rounding_amount(std::string_view amount);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_delivery(std::optional<std::string_view> actual, std::optional<std::string_view> latest);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_tax_currency(std::string_view currency, std::string_view amount);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_tax_exemption(uint8_t category, std::string_view rate, const _native::TaxExemption& exemption);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> add_adjustment(const _native::AllowanceCharge& adjustment);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> add_item(const _native::InvoiceLineItem& item);

  inline static _native::diplomat::result<std::unique_ptr<_native::InvoiceBuilder>, std::unique_ptr<_native::BindingError>> new_(uint8_t kind, uint8_t subtype, std::optional<std::string_view> original_id, std::optional<std::string_view> original_uuid, std::optional<std::string_view> original_date, std::optional<std::string_view> reason);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_id(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_uuid(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_issue_datetime(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_currency(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_previous_invoice_hash(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_payment_means_code(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> allowance_reason(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> invoice_level_charge(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> invoice_level_discount(std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_invoice_counter(uint64_t value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_vat_category(uint8_t value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> flags(uint8_t value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_note(std::string_view language, std::string_view value);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_allowance(std::string_view reason, std::string_view amount);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_seller(std::string_view name, const _native::Address& address, std::string_view vat_id, std::optional<std::string_view> other_id, std::optional<std::string_view> scheme);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> set_buyer(std::string_view name, const _native::Address& address, std::optional<std::string_view> vat_id, std::optional<std::string_view> other_id, std::optional<std::string_view> scheme);

  inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> add_line_item(std::string_view description, std::string_view quantity, std::string_view unit_code, std::string_view unit_price, std::string_view vat_rate, uint8_t category);

  inline _native::diplomat::result<std::unique_ptr<_native::FinalizedInvoice>, std::unique_ptr<_native::BindingError>> build();

    inline const _native::capi::InvoiceBuilder* AsFFI() const;
    inline _native::capi::InvoiceBuilder* AsFFI();
    inline static const _native::InvoiceBuilder* FromFFI(const _native::capi::InvoiceBuilder* ptr);
    inline static _native::InvoiceBuilder* FromFFI(_native::capi::InvoiceBuilder* ptr);
    inline static void operator delete(void* ptr);
private:
    InvoiceBuilder() = delete;
    InvoiceBuilder(const _native::InvoiceBuilder&) = delete;
    InvoiceBuilder(_native::InvoiceBuilder&&) noexcept = delete;
    InvoiceBuilder operator=(const _native::InvoiceBuilder&) = delete;
    InvoiceBuilder operator=(_native::InvoiceBuilder&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // _NATIVE_InvoiceBuilder_D_HPP

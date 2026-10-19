#ifndef fatoora_InvoiceBuilder_D_HPP
#define fatoora_InvoiceBuilder_D_HPP

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
} // namespace fatoora




namespace fatoora {
namespace capi {
    struct InvoiceBuilder;
} // namespace capi
} // namespace

namespace fatoora {
class InvoiceBuilder {
public:

  /**
   * Construct a credit or debit note with a complete billing reference.
   */
  inline static diplomat::result<std::unique_ptr<fatoora::InvoiceBuilder>, std::unique_ptr<fatoora::BindingError>> new_with_reference(uint8_t kind, uint8_t subtype, const fatoora::OriginalInvoiceRef& reference, std::string_view reason);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_note_without_language(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_payment_instruction(std::string_view instruction);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_prepaid_amount(std::string_view amount);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_payable_rounding_amount(std::string_view amount);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_delivery(std::optional<std::string_view> actual, std::optional<std::string_view> latest);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_tax_currency(std::string_view currency, std::string_view amount);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_tax_exemption(uint8_t category, std::string_view rate, const fatoora::TaxExemption& exemption);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> add_adjustment(const fatoora::AllowanceCharge& adjustment);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> add_item(const fatoora::InvoiceLineItem& item);

  inline static diplomat::result<std::unique_ptr<fatoora::InvoiceBuilder>, std::unique_ptr<fatoora::BindingError>> new_(uint8_t kind, uint8_t subtype, std::optional<std::string_view> original_id, std::optional<std::string_view> original_uuid, std::optional<std::string_view> original_date, std::optional<std::string_view> reason);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_id(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_uuid(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_issue_datetime(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_currency(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_previous_invoice_hash(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_payment_means_code(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> allowance_reason(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> invoice_level_charge(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> invoice_level_discount(std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_invoice_counter(uint64_t value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_vat_category(uint8_t value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> flags(uint8_t value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_note(std::string_view language, std::string_view value);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_allowance(std::string_view reason, std::string_view amount);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_seller(std::string_view name, const fatoora::Address& address, std::string_view vat_id, std::optional<std::string_view> other_id, std::optional<std::string_view> scheme);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> set_buyer(std::string_view name, const fatoora::Address& address, std::optional<std::string_view> vat_id, std::optional<std::string_view> other_id, std::optional<std::string_view> scheme);

  inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> add_line_item(std::string_view description, std::string_view quantity, std::string_view unit_code, std::string_view unit_price, std::string_view vat_rate, uint8_t category);

  inline diplomat::result<std::unique_ptr<fatoora::FinalizedInvoice>, std::unique_ptr<fatoora::BindingError>> build();

    inline const fatoora::capi::InvoiceBuilder* AsFFI() const;
    inline fatoora::capi::InvoiceBuilder* AsFFI();
    inline static const fatoora::InvoiceBuilder* FromFFI(const fatoora::capi::InvoiceBuilder* ptr);
    inline static fatoora::InvoiceBuilder* FromFFI(fatoora::capi::InvoiceBuilder* ptr);
    inline static void operator delete(void* ptr);
private:
    InvoiceBuilder() = delete;
    InvoiceBuilder(const fatoora::InvoiceBuilder&) = delete;
    InvoiceBuilder(fatoora::InvoiceBuilder&&) noexcept = delete;
    InvoiceBuilder operator=(const fatoora::InvoiceBuilder&) = delete;
    InvoiceBuilder operator=(fatoora::InvoiceBuilder&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // fatoora_InvoiceBuilder_D_HPP

#ifndef fatoora_InvoiceLineItem_HPP
#define fatoora_InvoiceLineItem_HPP

#include "InvoiceLineItem.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "Adjustment.hpp"
#include "BindingError.hpp"
#include "OriginalInvoiceRef.hpp"
#include "OtherId.hpp"
#include "TaxExemption.hpp"
#include "Text.hpp"
#include "VatBreakdown.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    typedef struct fatoora_InvoiceLineItem_item_gross_price_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_gross_price_result;
    fatoora_InvoiceLineItem_item_gross_price_result fatoora_InvoiceLineItem_item_gross_price(const fatoora::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_item_price_discount_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_price_discount_result;
    fatoora_InvoiceLineItem_item_price_discount_result fatoora_InvoiceLineItem_item_price_discount(const fatoora::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_item_price_base_quantity_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_price_base_quantity_result;
    fatoora_InvoiceLineItem_item_price_base_quantity_result fatoora_InvoiceLineItem_item_price_base_quantity(const fatoora::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_item_price_base_quantity_unit_code_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_price_base_quantity_unit_code_result;
    fatoora_InvoiceLineItem_item_price_base_quantity_unit_code_result fatoora_InvoiceLineItem_item_price_base_quantity_unit_code(const fatoora::capi::InvoiceLineItem* self);

    size_t fatoora_InvoiceLineItem_adjustments_len(const fatoora::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_adjustment_result {union {fatoora::capi::Adjustment* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_adjustment_result;
    fatoora_InvoiceLineItem_adjustment_result fatoora_InvoiceLineItem_adjustment(const fatoora::capi::InvoiceLineItem* self, size_t index);

    typedef struct fatoora_InvoiceLineItem_adjustment_is_charge_result {union {bool ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_adjustment_is_charge_result;
    fatoora_InvoiceLineItem_adjustment_is_charge_result fatoora_InvoiceLineItem_adjustment_is_charge(const fatoora::capi::InvoiceLineItem* self, size_t index);

    typedef struct fatoora_InvoiceLineItem_item_seller_identifier_result {union {fatoora::capi::OtherId* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_seller_identifier_result;
    fatoora_InvoiceLineItem_item_seller_identifier_result fatoora_InvoiceLineItem_item_seller_identifier(const fatoora::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_item_buyer_identifier_result {union {fatoora::capi::OtherId* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_buyer_identifier_result;
    fatoora_InvoiceLineItem_item_buyer_identifier_result fatoora_InvoiceLineItem_item_buyer_identifier(const fatoora::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_item_standard_identifier_result {union {fatoora::capi::OtherId* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_standard_identifier_result;
    fatoora_InvoiceLineItem_item_standard_identifier_result fatoora_InvoiceLineItem_item_standard_identifier(const fatoora::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_with_document_reference_result {union {fatoora::capi::InvoiceLineItem* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_with_document_reference_result;
    fatoora_InvoiceLineItem_with_document_reference_result fatoora_InvoiceLineItem_with_document_reference(const fatoora::capi::InvoiceLineItem* self, const fatoora::capi::OriginalInvoiceRef* reference);

    typedef struct fatoora_InvoiceLineItem_with_tax_subtotal_result {union {fatoora::capi::InvoiceLineItem* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_with_tax_subtotal_result;
    fatoora_InvoiceLineItem_with_tax_subtotal_result fatoora_InvoiceLineItem_with_tax_subtotal(const fatoora::capi::InvoiceLineItem* self, const fatoora::capi::VatBreakdown* subtotal);

    size_t fatoora_InvoiceLineItem_document_references_len(const fatoora::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_document_reference_result {union {fatoora::capi::OriginalInvoiceRef* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_document_reference_result;
    fatoora_InvoiceLineItem_document_reference_result fatoora_InvoiceLineItem_document_reference(const fatoora::capi::InvoiceLineItem* self, size_t index);

    size_t fatoora_InvoiceLineItem_tax_subtotals_len(const fatoora::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_tax_subtotal_result {union {fatoora::capi::VatBreakdown* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_tax_subtotal_result;
    fatoora_InvoiceLineItem_tax_subtotal_result fatoora_InvoiceLineItem_tax_subtotal(const fatoora::capi::InvoiceLineItem* self, size_t index);

    typedef struct fatoora_InvoiceLineItem_new_result {union {fatoora::capi::InvoiceLineItem* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_new_result;
    fatoora_InvoiceLineItem_new_result fatoora_InvoiceLineItem_new(diplomat::capi::DiplomatStringView description, diplomat::capi::DiplomatStringView quantity, diplomat::capi::DiplomatStringView unit_code, diplomat::capi::DiplomatStringView unit_price, diplomat::capi::DiplomatStringView rate, uint8_t category);

    typedef struct fatoora_InvoiceLineItem_with_metadata_result {union {fatoora::capi::InvoiceLineItem* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_with_metadata_result;
    fatoora_InvoiceLineItem_with_metadata_result fatoora_InvoiceLineItem_with_metadata(const fatoora::capi::InvoiceLineItem* self, diplomat::capi::OptionStringView id, const fatoora::capi::TaxExemption* exemption);

    typedef struct fatoora_InvoiceLineItem_id_result {union {fatoora::capi::Text* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_id_result;
    fatoora_InvoiceLineItem_id_result fatoora_InvoiceLineItem_id(const fatoora::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_exemption_result {union {fatoora::capi::TaxExemption* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_exemption_result;
    fatoora_InvoiceLineItem_exemption_result fatoora_InvoiceLineItem_exemption(const fatoora::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_description_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_description_result;
    fatoora_InvoiceLineItem_description_result fatoora_InvoiceLineItem_description(const fatoora::capi::InvoiceLineItem* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_unit_code_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_unit_code_result;
    fatoora_InvoiceLineItem_unit_code_result fatoora_InvoiceLineItem_unit_code(const fatoora::capi::InvoiceLineItem* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_quantity_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_quantity_result;
    fatoora_InvoiceLineItem_quantity_result fatoora_InvoiceLineItem_quantity(const fatoora::capi::InvoiceLineItem* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_unit_price_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_unit_price_result;
    fatoora_InvoiceLineItem_unit_price_result fatoora_InvoiceLineItem_unit_price(const fatoora::capi::InvoiceLineItem* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_total_amount_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_total_amount_result;
    fatoora_InvoiceLineItem_total_amount_result fatoora_InvoiceLineItem_total_amount(const fatoora::capi::InvoiceLineItem* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_vat_rate_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_vat_rate_result;
    fatoora_InvoiceLineItem_vat_rate_result fatoora_InvoiceLineItem_vat_rate(const fatoora::capi::InvoiceLineItem* self, diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_vat_amount_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_vat_amount_result;
    fatoora_InvoiceLineItem_vat_amount_result fatoora_InvoiceLineItem_vat_amount(const fatoora::capi::InvoiceLineItem* self, diplomat::capi::DiplomatWrite* write);

    uint8_t fatoora_InvoiceLineItem_vat_category(const fatoora::capi::InvoiceLineItem* self);

    void fatoora_InvoiceLineItem_destroy(InvoiceLineItem* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::item_gross_price() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_item_gross_price(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::item_price_discount() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_item_price_discount(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::item_price_base_quantity() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_item_price_base_quantity(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::item_price_base_quantity_unit_code() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_item_price_base_quantity_unit_code(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline size_t fatoora::InvoiceLineItem::adjustments_len() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_adjustments_len(this->AsFFI());
    return result;
}

inline diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::adjustment(size_t index) const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_adjustment(this->AsFFI(),
        index);
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Adjustment>>(std::unique_ptr<fatoora::Adjustment>(fatoora::Adjustment::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Adjustment>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<bool, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::adjustment_is_charge(size_t index) const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_adjustment_is_charge(this->AsFFI(),
        index);
    return result.is_ok ? diplomat::result<bool, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<bool>(result.ok)) : diplomat::result<bool, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::OtherId>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::item_seller_identifier() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_item_seller_identifier(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::OtherId>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::OtherId>>(std::unique_ptr<fatoora::OtherId>(fatoora::OtherId::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::OtherId>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::OtherId>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::item_buyer_identifier() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_item_buyer_identifier(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::OtherId>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::OtherId>>(std::unique_ptr<fatoora::OtherId>(fatoora::OtherId::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::OtherId>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::OtherId>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::item_standard_identifier() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_item_standard_identifier(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::OtherId>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::OtherId>>(std::unique_ptr<fatoora::OtherId>(fatoora::OtherId::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::OtherId>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::with_document_reference(const fatoora::OriginalInvoiceRef& reference) const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_with_document_reference(this->AsFFI(),
        reference.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::InvoiceLineItem>>(std::unique_ptr<fatoora::InvoiceLineItem>(fatoora::InvoiceLineItem::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::with_tax_subtotal(const fatoora::VatBreakdown& subtotal) const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_with_tax_subtotal(this->AsFFI(),
        subtotal.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::InvoiceLineItem>>(std::unique_ptr<fatoora::InvoiceLineItem>(fatoora::InvoiceLineItem::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline size_t fatoora::InvoiceLineItem::document_references_len() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_document_references_len(this->AsFFI());
    return result;
}

inline diplomat::result<std::unique_ptr<fatoora::OriginalInvoiceRef>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::document_reference(size_t index) const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_document_reference(this->AsFFI(),
        index);
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::OriginalInvoiceRef>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::OriginalInvoiceRef>>(std::unique_ptr<fatoora::OriginalInvoiceRef>(fatoora::OriginalInvoiceRef::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::OriginalInvoiceRef>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline size_t fatoora::InvoiceLineItem::tax_subtotals_len() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_tax_subtotals_len(this->AsFFI());
    return result;
}

inline diplomat::result<std::unique_ptr<fatoora::VatBreakdown>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::tax_subtotal(size_t index) const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_tax_subtotal(this->AsFFI(),
        index);
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::VatBreakdown>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::VatBreakdown>>(std::unique_ptr<fatoora::VatBreakdown>(fatoora::VatBreakdown::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::VatBreakdown>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::new_(std::string_view description, std::string_view quantity, std::string_view unit_code, std::string_view unit_price, std::string_view rate, uint8_t category) {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_new({description.data(), description.size()},
        {quantity.data(), quantity.size()},
        {unit_code.data(), unit_code.size()},
        {unit_price.data(), unit_price.size()},
        {rate.data(), rate.size()},
        category);
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::InvoiceLineItem>>(std::unique_ptr<fatoora::InvoiceLineItem>(fatoora::InvoiceLineItem::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::with_metadata(std::optional<std::string_view> id, const fatoora::TaxExemption& exemption) const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_with_metadata(this->AsFFI(),
        id.has_value() ? (diplomat::capi::OptionStringView{ { {id.value().data(), id.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }),
        exemption.AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::InvoiceLineItem>>(std::unique_ptr<fatoora::InvoiceLineItem>(fatoora::InvoiceLineItem::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::id() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_id(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::Text>>(std::unique_ptr<fatoora::Text>(fatoora::Text::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::Text>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::exemption() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_exemption(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::TaxExemption>>(std::unique_ptr<fatoora::TaxExemption>(fatoora::TaxExemption::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::TaxExemption>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::description() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_description(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::description_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_description(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::unit_code() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_unit_code(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::unit_code_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_unit_code(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::quantity() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_quantity(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::quantity_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_quantity(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::unit_price() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_unit_price(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::unit_price_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_unit_price(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::total_amount() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_total_amount(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::total_amount_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_total_amount(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::vat_rate() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_vat_rate(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::vat_rate_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_vat_rate(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::vat_amount() const {
    std::string output;
    diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_vat_amount(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::InvoiceLineItem::vat_amount_write(W& writeable) const {
    diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
    auto result = fatoora::capi::fatoora_InvoiceLineItem_vat_amount(this->AsFFI(),
        &write);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline uint8_t fatoora::InvoiceLineItem::vat_category() const {
    auto result = fatoora::capi::fatoora_InvoiceLineItem_vat_category(this->AsFFI());
    return result;
}

inline const fatoora::capi::InvoiceLineItem* fatoora::InvoiceLineItem::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::InvoiceLineItem*>(this);
}

inline fatoora::capi::InvoiceLineItem* fatoora::InvoiceLineItem::AsFFI() {
    return reinterpret_cast<fatoora::capi::InvoiceLineItem*>(this);
}

inline const fatoora::InvoiceLineItem* fatoora::InvoiceLineItem::FromFFI(const fatoora::capi::InvoiceLineItem* ptr) {
    return reinterpret_cast<const fatoora::InvoiceLineItem*>(ptr);
}

inline fatoora::InvoiceLineItem* fatoora::InvoiceLineItem::FromFFI(fatoora::capi::InvoiceLineItem* ptr) {
    return reinterpret_cast<fatoora::InvoiceLineItem*>(ptr);
}

inline void fatoora::InvoiceLineItem::operator delete(void* ptr) {
    fatoora::capi::fatoora_InvoiceLineItem_destroy(reinterpret_cast<fatoora::capi::InvoiceLineItem*>(ptr));
}


#endif // fatoora_InvoiceLineItem_HPP

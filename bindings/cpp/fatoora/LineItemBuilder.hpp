#ifndef fatoora_LineItemBuilder_HPP
#define fatoora_LineItemBuilder_HPP

#include "LineItemBuilder.d.hpp"

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
#include "InvoiceLineItem.hpp"
#include "OriginalInvoiceRef.hpp"
#include "TaxExemption.hpp"
#include "VatBreakdown.hpp"


namespace fatoora {
namespace capi {
    extern "C" {

    fatoora::capi::LineItemBuilder* fatoora_LineItemBuilder_new(void);

    typedef struct fatoora_LineItemBuilder_build_result {union {fatoora::capi::InvoiceLineItem* ok; fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_build_result;
    fatoora_LineItemBuilder_build_result fatoora_LineItemBuilder_build(fatoora::capi::LineItemBuilder* self);

    typedef struct fatoora_LineItemBuilder_set_item_name_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_name_result;
    fatoora_LineItemBuilder_set_item_name_result fatoora_LineItemBuilder_set_item_name(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_invoiced_quantity_unit_code_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_invoiced_quantity_unit_code_result;
    fatoora_LineItemBuilder_set_invoiced_quantity_unit_code_result fatoora_LineItemBuilder_set_invoiced_quantity_unit_code(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code_result;
    fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code_result fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_id_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_id_result;
    fatoora_LineItemBuilder_set_id_result fatoora_LineItemBuilder_set_id(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_invoiced_quantity_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_invoiced_quantity_result;
    fatoora_LineItemBuilder_set_invoiced_quantity_result fatoora_LineItemBuilder_set_invoiced_quantity(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_item_net_price_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_net_price_result;
    fatoora_LineItemBuilder_set_item_net_price_result fatoora_LineItemBuilder_set_item_net_price(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_item_gross_price_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_gross_price_result;
    fatoora_LineItemBuilder_set_item_gross_price_result fatoora_LineItemBuilder_set_item_gross_price(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_item_price_discount_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_price_discount_result;
    fatoora_LineItemBuilder_set_item_price_discount_result fatoora_LineItemBuilder_set_item_price_discount(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_item_price_base_quantity_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_price_base_quantity_result;
    fatoora_LineItemBuilder_set_item_price_base_quantity_result fatoora_LineItemBuilder_set_item_price_base_quantity(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_line_net_amount_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_line_net_amount_result;
    fatoora_LineItemBuilder_set_line_net_amount_result fatoora_LineItemBuilder_set_line_net_amount(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_line_vat_amount_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_line_vat_amount_result;
    fatoora_LineItemBuilder_set_line_vat_amount_result fatoora_LineItemBuilder_set_line_vat_amount(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_line_amount_with_vat_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_line_amount_with_vat_result;
    fatoora_LineItemBuilder_set_line_amount_with_vat_result fatoora_LineItemBuilder_set_line_amount_with_vat(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_vat_rate_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_vat_rate_result;
    fatoora_LineItemBuilder_set_vat_rate_result fatoora_LineItemBuilder_set_vat_rate(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_vat_category_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_vat_category_result;
    fatoora_LineItemBuilder_set_vat_category_result fatoora_LineItemBuilder_set_vat_category(fatoora::capi::LineItemBuilder* self, uint8_t value);

    typedef struct fatoora_LineItemBuilder_set_exemption_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_exemption_result;
    fatoora_LineItemBuilder_set_exemption_result fatoora_LineItemBuilder_set_exemption(fatoora::capi::LineItemBuilder* self, const fatoora::capi::TaxExemption* value);

    typedef struct fatoora_LineItemBuilder_add_allowance_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_allowance_result;
    fatoora_LineItemBuilder_add_allowance_result fatoora_LineItemBuilder_add_allowance(fatoora::capi::LineItemBuilder* self, const fatoora::capi::Adjustment* value);

    typedef struct fatoora_LineItemBuilder_add_charge_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_charge_result;
    fatoora_LineItemBuilder_add_charge_result fatoora_LineItemBuilder_add_charge(fatoora::capi::LineItemBuilder* self, const fatoora::capi::Adjustment* value);

    typedef struct fatoora_LineItemBuilder_add_document_reference_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_document_reference_result;
    fatoora_LineItemBuilder_add_document_reference_result fatoora_LineItemBuilder_add_document_reference(fatoora::capi::LineItemBuilder* self, const fatoora::capi::OriginalInvoiceRef* value);

    typedef struct fatoora_LineItemBuilder_add_tax_subtotal_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_tax_subtotal_result;
    fatoora_LineItemBuilder_add_tax_subtotal_result fatoora_LineItemBuilder_add_tax_subtotal(fatoora::capi::LineItemBuilder* self, const fatoora::capi::VatBreakdown* value);

    typedef struct fatoora_LineItemBuilder_set_item_seller_identifier_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_seller_identifier_result;
    fatoora_LineItemBuilder_set_item_seller_identifier_result fatoora_LineItemBuilder_set_item_seller_identifier(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value, diplomat::capi::OptionStringView scheme);

    typedef struct fatoora_LineItemBuilder_set_item_buyer_identifier_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_buyer_identifier_result;
    fatoora_LineItemBuilder_set_item_buyer_identifier_result fatoora_LineItemBuilder_set_item_buyer_identifier(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value, diplomat::capi::OptionStringView scheme);

    typedef struct fatoora_LineItemBuilder_set_item_standard_identifier_result {union { fatoora::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_standard_identifier_result;
    fatoora_LineItemBuilder_set_item_standard_identifier_result fatoora_LineItemBuilder_set_item_standard_identifier(fatoora::capi::LineItemBuilder* self, diplomat::capi::DiplomatStringView value, diplomat::capi::OptionStringView scheme);

    void fatoora_LineItemBuilder_destroy(LineItemBuilder* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<fatoora::LineItemBuilder> fatoora::LineItemBuilder::new_() {
    auto result = fatoora::capi::fatoora_LineItemBuilder_new();
    return std::unique_ptr<fatoora::LineItemBuilder>(fatoora::LineItemBuilder::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::build() {
    auto result = fatoora::capi::fatoora_LineItemBuilder_build(this->AsFFI());
    return result.is_ok ? diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::unique_ptr<fatoora::InvoiceLineItem>>(std::unique_ptr<fatoora::InvoiceLineItem>(fatoora::InvoiceLineItem::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<fatoora::InvoiceLineItem>, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_item_name(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_item_name(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_invoiced_quantity_unit_code(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_invoiced_quantity_unit_code(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_item_price_base_quantity_unit_code(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_id(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_id(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_invoiced_quantity(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_invoiced_quantity(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_item_net_price(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_item_net_price(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_item_gross_price(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_item_gross_price(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_item_price_discount(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_item_price_discount(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_item_price_base_quantity(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_item_price_base_quantity(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_line_net_amount(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_line_net_amount(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_line_vat_amount(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_line_vat_amount(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_line_amount_with_vat(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_line_amount_with_vat(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_vat_rate(std::string_view value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_vat_rate(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_vat_category(uint8_t value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_vat_category(this->AsFFI(),
        value);
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_exemption(const fatoora::TaxExemption& value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_exemption(this->AsFFI(),
        value.AsFFI());
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::add_allowance(const fatoora::Adjustment& value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_add_allowance(this->AsFFI(),
        value.AsFFI());
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::add_charge(const fatoora::Adjustment& value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_add_charge(this->AsFFI(),
        value.AsFFI());
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::add_document_reference(const fatoora::OriginalInvoiceRef& value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_add_document_reference(this->AsFFI(),
        value.AsFFI());
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::add_tax_subtotal(const fatoora::VatBreakdown& value) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_add_tax_subtotal(this->AsFFI(),
        value.AsFFI());
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_item_seller_identifier(std::string_view value, std::optional<std::string_view> scheme) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_item_seller_identifier(this->AsFFI(),
        {value.data(), value.size()},
        scheme.has_value() ? (diplomat::capi::OptionStringView{ { {scheme.value().data(), scheme.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_item_buyer_identifier(std::string_view value, std::optional<std::string_view> scheme) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_item_buyer_identifier(this->AsFFI(),
        {value.data(), value.size()},
        scheme.has_value() ? (diplomat::capi::OptionStringView{ { {scheme.value().data(), scheme.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>> fatoora::LineItemBuilder::set_item_standard_identifier(std::string_view value, std::optional<std::string_view> scheme) {
    auto result = fatoora::capi::fatoora_LineItemBuilder_set_item_standard_identifier(this->AsFFI(),
        {value.data(), value.size()},
        scheme.has_value() ? (diplomat::capi::OptionStringView{ { {scheme.value().data(), scheme.value().size()} }, true }) : (diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<fatoora::BindingError>>(diplomat::Err<std::unique_ptr<fatoora::BindingError>>(std::unique_ptr<fatoora::BindingError>(fatoora::BindingError::FromFFI(result.err))));
}

inline const fatoora::capi::LineItemBuilder* fatoora::LineItemBuilder::AsFFI() const {
    return reinterpret_cast<const fatoora::capi::LineItemBuilder*>(this);
}

inline fatoora::capi::LineItemBuilder* fatoora::LineItemBuilder::AsFFI() {
    return reinterpret_cast<fatoora::capi::LineItemBuilder*>(this);
}

inline const fatoora::LineItemBuilder* fatoora::LineItemBuilder::FromFFI(const fatoora::capi::LineItemBuilder* ptr) {
    return reinterpret_cast<const fatoora::LineItemBuilder*>(ptr);
}

inline fatoora::LineItemBuilder* fatoora::LineItemBuilder::FromFFI(fatoora::capi::LineItemBuilder* ptr) {
    return reinterpret_cast<fatoora::LineItemBuilder*>(ptr);
}

inline void fatoora::LineItemBuilder::operator delete(void* ptr) {
    fatoora::capi::fatoora_LineItemBuilder_destroy(reinterpret_cast<fatoora::capi::LineItemBuilder*>(ptr));
}


#endif // fatoora_LineItemBuilder_HPP

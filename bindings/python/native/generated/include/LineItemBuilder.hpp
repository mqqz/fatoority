#ifndef _NATIVE_LineItemBuilder_HPP
#define _NATIVE_LineItemBuilder_HPP

#include "LineItemBuilder.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Adjustment.hpp"
#include "BindingError.hpp"
#include "InvoiceLineItem.hpp"
#include "OriginalInvoiceRef.hpp"
#include "TaxExemption.hpp"
#include "VatBreakdown.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_LineItemBuilder_set_tax_total_present_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_tax_total_present_result;
    fatoora_LineItemBuilder_set_tax_total_present_result fatoora_LineItemBuilder_set_tax_total_present(_native::capi::LineItemBuilder* self, bool present);

    typedef struct fatoora_LineItemBuilder_set_gross_amount_present_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_gross_amount_present_result;
    fatoora_LineItemBuilder_set_gross_amount_present_result fatoora_LineItemBuilder_set_gross_amount_present(_native::capi::LineItemBuilder* self, bool present);

    _native::capi::LineItemBuilder* fatoora_LineItemBuilder_new(void);

    typedef struct fatoora_LineItemBuilder_build_result {union {_native::capi::InvoiceLineItem* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_build_result;
    fatoora_LineItemBuilder_build_result fatoora_LineItemBuilder_build(_native::capi::LineItemBuilder* self);

    typedef struct fatoora_LineItemBuilder_set_item_name_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_name_result;
    fatoora_LineItemBuilder_set_item_name_result fatoora_LineItemBuilder_set_item_name(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_invoiced_quantity_unit_code_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_invoiced_quantity_unit_code_result;
    fatoora_LineItemBuilder_set_invoiced_quantity_unit_code_result fatoora_LineItemBuilder_set_invoiced_quantity_unit_code(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code_result;
    fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code_result fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_id_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_id_result;
    fatoora_LineItemBuilder_set_id_result fatoora_LineItemBuilder_set_id(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_invoiced_quantity_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_invoiced_quantity_result;
    fatoora_LineItemBuilder_set_invoiced_quantity_result fatoora_LineItemBuilder_set_invoiced_quantity(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_item_net_price_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_net_price_result;
    fatoora_LineItemBuilder_set_item_net_price_result fatoora_LineItemBuilder_set_item_net_price(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_item_gross_price_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_gross_price_result;
    fatoora_LineItemBuilder_set_item_gross_price_result fatoora_LineItemBuilder_set_item_gross_price(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_item_price_discount_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_price_discount_result;
    fatoora_LineItemBuilder_set_item_price_discount_result fatoora_LineItemBuilder_set_item_price_discount(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_item_price_base_quantity_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_price_base_quantity_result;
    fatoora_LineItemBuilder_set_item_price_base_quantity_result fatoora_LineItemBuilder_set_item_price_base_quantity(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_line_net_amount_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_line_net_amount_result;
    fatoora_LineItemBuilder_set_line_net_amount_result fatoora_LineItemBuilder_set_line_net_amount(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_line_vat_amount_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_line_vat_amount_result;
    fatoora_LineItemBuilder_set_line_vat_amount_result fatoora_LineItemBuilder_set_line_vat_amount(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_line_amount_with_vat_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_line_amount_with_vat_result;
    fatoora_LineItemBuilder_set_line_amount_with_vat_result fatoora_LineItemBuilder_set_line_amount_with_vat(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_vat_rate_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_vat_rate_result;
    fatoora_LineItemBuilder_set_vat_rate_result fatoora_LineItemBuilder_set_vat_rate(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value);

    typedef struct fatoora_LineItemBuilder_set_vat_category_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_vat_category_result;
    fatoora_LineItemBuilder_set_vat_category_result fatoora_LineItemBuilder_set_vat_category(_native::capi::LineItemBuilder* self, uint8_t value);

    typedef struct fatoora_LineItemBuilder_set_exemption_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_exemption_result;
    fatoora_LineItemBuilder_set_exemption_result fatoora_LineItemBuilder_set_exemption(_native::capi::LineItemBuilder* self, const _native::capi::TaxExemption* value);

    typedef struct fatoora_LineItemBuilder_add_allowance_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_allowance_result;
    fatoora_LineItemBuilder_add_allowance_result fatoora_LineItemBuilder_add_allowance(_native::capi::LineItemBuilder* self, const _native::capi::Adjustment* value);

    typedef struct fatoora_LineItemBuilder_add_charge_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_charge_result;
    fatoora_LineItemBuilder_add_charge_result fatoora_LineItemBuilder_add_charge(_native::capi::LineItemBuilder* self, const _native::capi::Adjustment* value);

    typedef struct fatoora_LineItemBuilder_add_document_reference_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_document_reference_result;
    fatoora_LineItemBuilder_add_document_reference_result fatoora_LineItemBuilder_add_document_reference(_native::capi::LineItemBuilder* self, const _native::capi::OriginalInvoiceRef* value);

    typedef struct fatoora_LineItemBuilder_add_tax_subtotal_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_tax_subtotal_result;
    fatoora_LineItemBuilder_add_tax_subtotal_result fatoora_LineItemBuilder_add_tax_subtotal(_native::capi::LineItemBuilder* self, const _native::capi::VatBreakdown* value);

    typedef struct fatoora_LineItemBuilder_set_item_seller_identifier_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_seller_identifier_result;
    fatoora_LineItemBuilder_set_item_seller_identifier_result fatoora_LineItemBuilder_set_item_seller_identifier(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value, _native::diplomat::capi::OptionStringView scheme);

    typedef struct fatoora_LineItemBuilder_set_item_buyer_identifier_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_buyer_identifier_result;
    fatoora_LineItemBuilder_set_item_buyer_identifier_result fatoora_LineItemBuilder_set_item_buyer_identifier(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value, _native::diplomat::capi::OptionStringView scheme);

    typedef struct fatoora_LineItemBuilder_set_item_standard_identifier_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_standard_identifier_result;
    fatoora_LineItemBuilder_set_item_standard_identifier_result fatoora_LineItemBuilder_set_item_standard_identifier(_native::capi::LineItemBuilder* self, _native::diplomat::capi::DiplomatStringView value, _native::diplomat::capi::OptionStringView scheme);

    void fatoora_LineItemBuilder_destroy(LineItemBuilder* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_tax_total_present(bool present) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_tax_total_present(this->AsFFI(),
        present);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_gross_amount_present(bool present) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_gross_amount_present(this->AsFFI(),
        present);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline std::unique_ptr<_native::LineItemBuilder> _native::LineItemBuilder::new_() {
    auto result = _native::capi::fatoora_LineItemBuilder_new();
    return std::unique_ptr<_native::LineItemBuilder>(_native::LineItemBuilder::FromFFI(result));
}

inline _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::build() {
    auto result = _native::capi::fatoora_LineItemBuilder_build(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::InvoiceLineItem>>(std::unique_ptr<_native::InvoiceLineItem>(_native::InvoiceLineItem::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_item_name(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_item_name(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_invoiced_quantity_unit_code(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_invoiced_quantity_unit_code(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_item_price_base_quantity_unit_code(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_id(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_id(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_invoiced_quantity(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_invoiced_quantity(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_item_net_price(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_item_net_price(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_item_gross_price(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_item_gross_price(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_item_price_discount(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_item_price_discount(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_item_price_base_quantity(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_item_price_base_quantity(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_line_net_amount(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_line_net_amount(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_line_vat_amount(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_line_vat_amount(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_line_amount_with_vat(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_line_amount_with_vat(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_vat_rate(std::string_view value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_vat_rate(this->AsFFI(),
        {value.data(), value.size()});
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_vat_category(uint8_t value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_vat_category(this->AsFFI(),
        value);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_exemption(const _native::TaxExemption& value) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_exemption(this->AsFFI(),
        value.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::add_allowance(const _native::Adjustment& value) {
    auto result = _native::capi::fatoora_LineItemBuilder_add_allowance(this->AsFFI(),
        value.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::add_charge(const _native::Adjustment& value) {
    auto result = _native::capi::fatoora_LineItemBuilder_add_charge(this->AsFFI(),
        value.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::add_document_reference(const _native::OriginalInvoiceRef& value) {
    auto result = _native::capi::fatoora_LineItemBuilder_add_document_reference(this->AsFFI(),
        value.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::add_tax_subtotal(const _native::VatBreakdown& value) {
    auto result = _native::capi::fatoora_LineItemBuilder_add_tax_subtotal(this->AsFFI(),
        value.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_item_seller_identifier(std::string_view value, std::optional<std::string_view> scheme) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_item_seller_identifier(this->AsFFI(),
        {value.data(), value.size()},
        scheme.has_value() ? (_native::diplomat::capi::OptionStringView{ { {scheme.value().data(), scheme.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_item_buyer_identifier(std::string_view value, std::optional<std::string_view> scheme) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_item_buyer_identifier(this->AsFFI(),
        {value.data(), value.size()},
        scheme.has_value() ? (_native::diplomat::capi::OptionStringView{ { {scheme.value().data(), scheme.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::LineItemBuilder::set_item_standard_identifier(std::string_view value, std::optional<std::string_view> scheme) {
    auto result = _native::capi::fatoora_LineItemBuilder_set_item_standard_identifier(this->AsFFI(),
        {value.data(), value.size()},
        scheme.has_value() ? (_native::diplomat::capi::OptionStringView{ { {scheme.value().data(), scheme.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }));
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline const _native::capi::LineItemBuilder* _native::LineItemBuilder::AsFFI() const {
    return reinterpret_cast<const _native::capi::LineItemBuilder*>(this);
}

inline _native::capi::LineItemBuilder* _native::LineItemBuilder::AsFFI() {
    return reinterpret_cast<_native::capi::LineItemBuilder*>(this);
}

inline const _native::LineItemBuilder* _native::LineItemBuilder::FromFFI(const _native::capi::LineItemBuilder* ptr) {
    return reinterpret_cast<const _native::LineItemBuilder*>(ptr);
}

inline _native::LineItemBuilder* _native::LineItemBuilder::FromFFI(_native::capi::LineItemBuilder* ptr) {
    return reinterpret_cast<_native::LineItemBuilder*>(ptr);
}

inline void _native::LineItemBuilder::operator delete(void* ptr) {
    _native::capi::fatoora_LineItemBuilder_destroy(reinterpret_cast<_native::capi::LineItemBuilder*>(ptr));
}


#endif // _NATIVE_LineItemBuilder_HPP

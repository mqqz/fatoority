#ifndef _NATIVE_InvoiceLineItem_HPP
#define _NATIVE_InvoiceLineItem_HPP

#include "InvoiceLineItem.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BindingError.hpp"
#include "OriginalInvoiceRef.hpp"
#include "TaxExemption.hpp"
#include "Text.hpp"
#include "VatBreakdown.hpp"
#include "diplomat_runtime.hpp"


namespace _native {
namespace capi {
    extern "C" {

    typedef struct fatoora_InvoiceLineItem_with_document_reference_result {union {_native::capi::InvoiceLineItem* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_with_document_reference_result;
    fatoora_InvoiceLineItem_with_document_reference_result fatoora_InvoiceLineItem_with_document_reference(const _native::capi::InvoiceLineItem* self, const _native::capi::OriginalInvoiceRef* reference);

    typedef struct fatoora_InvoiceLineItem_with_tax_subtotal_result {union {_native::capi::InvoiceLineItem* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_with_tax_subtotal_result;
    fatoora_InvoiceLineItem_with_tax_subtotal_result fatoora_InvoiceLineItem_with_tax_subtotal(const _native::capi::InvoiceLineItem* self, const _native::capi::VatBreakdown* subtotal);

    size_t fatoora_InvoiceLineItem_document_references_len(const _native::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_document_reference_result {union {_native::capi::OriginalInvoiceRef* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_document_reference_result;
    fatoora_InvoiceLineItem_document_reference_result fatoora_InvoiceLineItem_document_reference(const _native::capi::InvoiceLineItem* self, size_t index);

    size_t fatoora_InvoiceLineItem_tax_subtotals_len(const _native::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_tax_subtotal_result {union {_native::capi::VatBreakdown* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_tax_subtotal_result;
    fatoora_InvoiceLineItem_tax_subtotal_result fatoora_InvoiceLineItem_tax_subtotal(const _native::capi::InvoiceLineItem* self, size_t index);

    typedef struct fatoora_InvoiceLineItem_new_result {union {_native::capi::InvoiceLineItem* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_new_result;
    fatoora_InvoiceLineItem_new_result fatoora_InvoiceLineItem_new(_native::diplomat::capi::DiplomatStringView description, _native::diplomat::capi::DiplomatStringView quantity, _native::diplomat::capi::DiplomatStringView unit_code, _native::diplomat::capi::DiplomatStringView unit_price, _native::diplomat::capi::DiplomatStringView rate, uint8_t category);

    typedef struct fatoora_InvoiceLineItem_with_metadata_result {union {_native::capi::InvoiceLineItem* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_with_metadata_result;
    fatoora_InvoiceLineItem_with_metadata_result fatoora_InvoiceLineItem_with_metadata(const _native::capi::InvoiceLineItem* self, _native::diplomat::capi::OptionStringView id, const _native::capi::TaxExemption* exemption);

    typedef struct fatoora_InvoiceLineItem_id_result {union {_native::capi::Text* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_id_result;
    fatoora_InvoiceLineItem_id_result fatoora_InvoiceLineItem_id(const _native::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_exemption_result {union {_native::capi::TaxExemption* ok; _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_exemption_result;
    fatoora_InvoiceLineItem_exemption_result fatoora_InvoiceLineItem_exemption(const _native::capi::InvoiceLineItem* self);

    typedef struct fatoora_InvoiceLineItem_description_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_description_result;
    fatoora_InvoiceLineItem_description_result fatoora_InvoiceLineItem_description(const _native::capi::InvoiceLineItem* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_unit_code_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_unit_code_result;
    fatoora_InvoiceLineItem_unit_code_result fatoora_InvoiceLineItem_unit_code(const _native::capi::InvoiceLineItem* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_quantity_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_quantity_result;
    fatoora_InvoiceLineItem_quantity_result fatoora_InvoiceLineItem_quantity(const _native::capi::InvoiceLineItem* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_unit_price_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_unit_price_result;
    fatoora_InvoiceLineItem_unit_price_result fatoora_InvoiceLineItem_unit_price(const _native::capi::InvoiceLineItem* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_total_amount_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_total_amount_result;
    fatoora_InvoiceLineItem_total_amount_result fatoora_InvoiceLineItem_total_amount(const _native::capi::InvoiceLineItem* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_vat_rate_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_vat_rate_result;
    fatoora_InvoiceLineItem_vat_rate_result fatoora_InvoiceLineItem_vat_rate(const _native::capi::InvoiceLineItem* self, _native::diplomat::capi::DiplomatWrite* write);

    typedef struct fatoora_InvoiceLineItem_vat_amount_result {union { _native::capi::BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_vat_amount_result;
    fatoora_InvoiceLineItem_vat_amount_result fatoora_InvoiceLineItem_vat_amount(const _native::capi::InvoiceLineItem* self, _native::diplomat::capi::DiplomatWrite* write);

    uint8_t fatoora_InvoiceLineItem_vat_category(const _native::capi::InvoiceLineItem* self);

    void fatoora_InvoiceLineItem_destroy(InvoiceLineItem* self);

    } // extern "C"
} // namespace capi
} // namespace

inline _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::with_document_reference(const _native::OriginalInvoiceRef& reference) const {
    auto result = _native::capi::fatoora_InvoiceLineItem_with_document_reference(this->AsFFI(),
        reference.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::InvoiceLineItem>>(std::unique_ptr<_native::InvoiceLineItem>(_native::InvoiceLineItem::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::with_tax_subtotal(const _native::VatBreakdown& subtotal) const {
    auto result = _native::capi::fatoora_InvoiceLineItem_with_tax_subtotal(this->AsFFI(),
        subtotal.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::InvoiceLineItem>>(std::unique_ptr<_native::InvoiceLineItem>(_native::InvoiceLineItem::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline size_t _native::InvoiceLineItem::document_references_len() const {
    auto result = _native::capi::fatoora_InvoiceLineItem_document_references_len(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::unique_ptr<_native::OriginalInvoiceRef>, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::document_reference(size_t index) const {
    auto result = _native::capi::fatoora_InvoiceLineItem_document_reference(this->AsFFI(),
        index);
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::OriginalInvoiceRef>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::OriginalInvoiceRef>>(std::unique_ptr<_native::OriginalInvoiceRef>(_native::OriginalInvoiceRef::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::OriginalInvoiceRef>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline size_t _native::InvoiceLineItem::tax_subtotals_len() const {
    auto result = _native::capi::fatoora_InvoiceLineItem_tax_subtotals_len(this->AsFFI());
    return result;
}

inline _native::diplomat::result<std::unique_ptr<_native::VatBreakdown>, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::tax_subtotal(size_t index) const {
    auto result = _native::capi::fatoora_InvoiceLineItem_tax_subtotal(this->AsFFI(),
        index);
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::VatBreakdown>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::VatBreakdown>>(std::unique_ptr<_native::VatBreakdown>(_native::VatBreakdown::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::VatBreakdown>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::new_(std::string_view description, std::string_view quantity, std::string_view unit_code, std::string_view unit_price, std::string_view rate, uint8_t category) {
    auto result = _native::capi::fatoora_InvoiceLineItem_new({description.data(), description.size()},
        {quantity.data(), quantity.size()},
        {unit_code.data(), unit_code.size()},
        {unit_price.data(), unit_price.size()},
        {rate.data(), rate.size()},
        category);
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::InvoiceLineItem>>(std::unique_ptr<_native::InvoiceLineItem>(_native::InvoiceLineItem::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::with_metadata(std::optional<std::string_view> id, const _native::TaxExemption& exemption) const {
    auto result = _native::capi::fatoora_InvoiceLineItem_with_metadata(this->AsFFI(),
        id.has_value() ? (_native::diplomat::capi::OptionStringView{ { {id.value().data(), id.value().size()} }, true }) : (_native::diplomat::capi::OptionStringView{ {}, false }),
        exemption.AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::InvoiceLineItem>>(std::unique_ptr<_native::InvoiceLineItem>(_native::InvoiceLineItem::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::InvoiceLineItem>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::id() const {
    auto result = _native::capi::fatoora_InvoiceLineItem_id(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::Text>>(std::unique_ptr<_native::Text>(_native::Text::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::Text>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::exemption() const {
    auto result = _native::capi::fatoora_InvoiceLineItem_exemption(this->AsFFI());
    return result.is_ok ? _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::unique_ptr<_native::TaxExemption>>(std::unique_ptr<_native::TaxExemption>(_native::TaxExemption::FromFFI(result.ok)))) : _native::diplomat::result<std::unique_ptr<_native::TaxExemption>, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::description() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceLineItem_description(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::description_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceLineItem_description(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::unit_code() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceLineItem_unit_code(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::unit_code_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceLineItem_unit_code(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::quantity() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceLineItem_quantity(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::quantity_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceLineItem_quantity(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::unit_price() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceLineItem_unit_price(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::unit_price_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceLineItem_unit_price(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::total_amount() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceLineItem_total_amount(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::total_amount_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceLineItem_total_amount(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::vat_rate() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceLineItem_vat_rate(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::vat_rate_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceLineItem_vat_rate(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::vat_amount() const {
    std::string output;
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteFromString(output);
    auto result = _native::capi::fatoora_InvoiceLineItem_vat_amount(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::string>(std::move(output))) : _native::diplomat::result<std::string, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}
template<typename W>
inline _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>> _native::InvoiceLineItem::vat_amount_write(W& writeable) const {
    _native::diplomat::capi::DiplomatWrite write = _native::diplomat::WriteTrait<W>::Construct(writeable);
    auto result = _native::capi::fatoora_InvoiceLineItem_vat_amount(this->AsFFI(),
        &write);
    return result.is_ok ? _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Ok<std::monostate>()) : _native::diplomat::result<std::monostate, std::unique_ptr<_native::BindingError>>(_native::diplomat::Err<std::unique_ptr<_native::BindingError>>(std::unique_ptr<_native::BindingError>(_native::BindingError::FromFFI(result.err))));
}

inline uint8_t _native::InvoiceLineItem::vat_category() const {
    auto result = _native::capi::fatoora_InvoiceLineItem_vat_category(this->AsFFI());
    return result;
}

inline const _native::capi::InvoiceLineItem* _native::InvoiceLineItem::AsFFI() const {
    return reinterpret_cast<const _native::capi::InvoiceLineItem*>(this);
}

inline _native::capi::InvoiceLineItem* _native::InvoiceLineItem::AsFFI() {
    return reinterpret_cast<_native::capi::InvoiceLineItem*>(this);
}

inline const _native::InvoiceLineItem* _native::InvoiceLineItem::FromFFI(const _native::capi::InvoiceLineItem* ptr) {
    return reinterpret_cast<const _native::InvoiceLineItem*>(ptr);
}

inline _native::InvoiceLineItem* _native::InvoiceLineItem::FromFFI(_native::capi::InvoiceLineItem* ptr) {
    return reinterpret_cast<_native::InvoiceLineItem*>(ptr);
}

inline void _native::InvoiceLineItem::operator delete(void* ptr) {
    _native::capi::fatoora_InvoiceLineItem_destroy(reinterpret_cast<_native::capi::InvoiceLineItem*>(ptr));
}


#endif // _NATIVE_InvoiceLineItem_HPP

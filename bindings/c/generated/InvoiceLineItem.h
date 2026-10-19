#ifndef InvoiceLineItem_H
#define InvoiceLineItem_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Adjustment.d.h"
#include "BindingError.d.h"
#include "OriginalInvoiceRef.d.h"
#include "OtherId.d.h"
#include "TaxExemption.d.h"
#include "Text.d.h"
#include "VatBreakdown.d.h"

#include "InvoiceLineItem.d.h"






typedef struct fatoora_InvoiceLineItem_item_gross_price_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_gross_price_result;
fatoora_InvoiceLineItem_item_gross_price_result fatoora_InvoiceLineItem_item_gross_price(const InvoiceLineItem* self);

typedef struct fatoora_InvoiceLineItem_item_price_discount_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_price_discount_result;
fatoora_InvoiceLineItem_item_price_discount_result fatoora_InvoiceLineItem_item_price_discount(const InvoiceLineItem* self);

typedef struct fatoora_InvoiceLineItem_item_price_base_quantity_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_price_base_quantity_result;
fatoora_InvoiceLineItem_item_price_base_quantity_result fatoora_InvoiceLineItem_item_price_base_quantity(const InvoiceLineItem* self);

typedef struct fatoora_InvoiceLineItem_item_price_base_quantity_unit_code_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_price_base_quantity_unit_code_result;
fatoora_InvoiceLineItem_item_price_base_quantity_unit_code_result fatoora_InvoiceLineItem_item_price_base_quantity_unit_code(const InvoiceLineItem* self);

size_t fatoora_InvoiceLineItem_adjustments_len(const InvoiceLineItem* self);

typedef struct fatoora_InvoiceLineItem_adjustment_result {union {Adjustment* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_adjustment_result;
fatoora_InvoiceLineItem_adjustment_result fatoora_InvoiceLineItem_adjustment(const InvoiceLineItem* self, size_t index);

typedef struct fatoora_InvoiceLineItem_adjustment_is_charge_result {union {bool ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_adjustment_is_charge_result;
fatoora_InvoiceLineItem_adjustment_is_charge_result fatoora_InvoiceLineItem_adjustment_is_charge(const InvoiceLineItem* self, size_t index);

typedef struct fatoora_InvoiceLineItem_item_seller_identifier_result {union {OtherId* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_seller_identifier_result;
fatoora_InvoiceLineItem_item_seller_identifier_result fatoora_InvoiceLineItem_item_seller_identifier(const InvoiceLineItem* self);

typedef struct fatoora_InvoiceLineItem_item_buyer_identifier_result {union {OtherId* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_buyer_identifier_result;
fatoora_InvoiceLineItem_item_buyer_identifier_result fatoora_InvoiceLineItem_item_buyer_identifier(const InvoiceLineItem* self);

typedef struct fatoora_InvoiceLineItem_item_standard_identifier_result {union {OtherId* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_item_standard_identifier_result;
fatoora_InvoiceLineItem_item_standard_identifier_result fatoora_InvoiceLineItem_item_standard_identifier(const InvoiceLineItem* self);

typedef struct fatoora_InvoiceLineItem_with_document_reference_result {union {InvoiceLineItem* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_with_document_reference_result;
fatoora_InvoiceLineItem_with_document_reference_result fatoora_InvoiceLineItem_with_document_reference(const InvoiceLineItem* self, const OriginalInvoiceRef* reference);

typedef struct fatoora_InvoiceLineItem_with_tax_subtotal_result {union {InvoiceLineItem* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_with_tax_subtotal_result;
fatoora_InvoiceLineItem_with_tax_subtotal_result fatoora_InvoiceLineItem_with_tax_subtotal(const InvoiceLineItem* self, const VatBreakdown* subtotal);

size_t fatoora_InvoiceLineItem_document_references_len(const InvoiceLineItem* self);

typedef struct fatoora_InvoiceLineItem_document_reference_result {union {OriginalInvoiceRef* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_document_reference_result;
fatoora_InvoiceLineItem_document_reference_result fatoora_InvoiceLineItem_document_reference(const InvoiceLineItem* self, size_t index);

size_t fatoora_InvoiceLineItem_tax_subtotals_len(const InvoiceLineItem* self);

typedef struct fatoora_InvoiceLineItem_tax_subtotal_result {union {VatBreakdown* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_tax_subtotal_result;
fatoora_InvoiceLineItem_tax_subtotal_result fatoora_InvoiceLineItem_tax_subtotal(const InvoiceLineItem* self, size_t index);

typedef struct fatoora_InvoiceLineItem_new_result {union {InvoiceLineItem* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_new_result;
fatoora_InvoiceLineItem_new_result fatoora_InvoiceLineItem_new(DiplomatStringView description, DiplomatStringView quantity, DiplomatStringView unit_code, DiplomatStringView unit_price, DiplomatStringView rate, uint8_t category);

typedef struct fatoora_InvoiceLineItem_with_metadata_result {union {InvoiceLineItem* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_with_metadata_result;
fatoora_InvoiceLineItem_with_metadata_result fatoora_InvoiceLineItem_with_metadata(const InvoiceLineItem* self, OptionStringView id, const TaxExemption* exemption);

typedef struct fatoora_InvoiceLineItem_id_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_id_result;
fatoora_InvoiceLineItem_id_result fatoora_InvoiceLineItem_id(const InvoiceLineItem* self);

typedef struct fatoora_InvoiceLineItem_exemption_result {union {TaxExemption* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_exemption_result;
fatoora_InvoiceLineItem_exemption_result fatoora_InvoiceLineItem_exemption(const InvoiceLineItem* self);

typedef struct fatoora_InvoiceLineItem_description_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_description_result;
fatoora_InvoiceLineItem_description_result fatoora_InvoiceLineItem_description(const InvoiceLineItem* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceLineItem_unit_code_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_unit_code_result;
fatoora_InvoiceLineItem_unit_code_result fatoora_InvoiceLineItem_unit_code(const InvoiceLineItem* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceLineItem_quantity_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_quantity_result;
fatoora_InvoiceLineItem_quantity_result fatoora_InvoiceLineItem_quantity(const InvoiceLineItem* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceLineItem_unit_price_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_unit_price_result;
fatoora_InvoiceLineItem_unit_price_result fatoora_InvoiceLineItem_unit_price(const InvoiceLineItem* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceLineItem_total_amount_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_total_amount_result;
fatoora_InvoiceLineItem_total_amount_result fatoora_InvoiceLineItem_total_amount(const InvoiceLineItem* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceLineItem_vat_rate_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_vat_rate_result;
fatoora_InvoiceLineItem_vat_rate_result fatoora_InvoiceLineItem_vat_rate(const InvoiceLineItem* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceLineItem_vat_amount_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceLineItem_vat_amount_result;
fatoora_InvoiceLineItem_vat_amount_result fatoora_InvoiceLineItem_vat_amount(const InvoiceLineItem* self, DiplomatWrite* write);

uint8_t fatoora_InvoiceLineItem_vat_category(const InvoiceLineItem* self);

void fatoora_InvoiceLineItem_destroy(InvoiceLineItem* self);





#endif // InvoiceLineItem_H

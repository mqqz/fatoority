#ifndef LineItemBuilder_H
#define LineItemBuilder_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Adjustment.d.h"
#include "BindingError.d.h"
#include "InvoiceLineItem.d.h"
#include "OriginalInvoiceRef.d.h"
#include "TaxExemption.d.h"
#include "VatBreakdown.d.h"

#include "LineItemBuilder.d.h"






LineItemBuilder* fatoora_LineItemBuilder_new(void);

typedef struct fatoora_LineItemBuilder_build_result {union {InvoiceLineItem* ok; BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_build_result;
fatoora_LineItemBuilder_build_result fatoora_LineItemBuilder_build(LineItemBuilder* self);

typedef struct fatoora_LineItemBuilder_set_item_name_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_name_result;
fatoora_LineItemBuilder_set_item_name_result fatoora_LineItemBuilder_set_item_name(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_invoiced_quantity_unit_code_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_invoiced_quantity_unit_code_result;
fatoora_LineItemBuilder_set_invoiced_quantity_unit_code_result fatoora_LineItemBuilder_set_invoiced_quantity_unit_code(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code_result;
fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code_result fatoora_LineItemBuilder_set_item_price_base_quantity_unit_code(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_id_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_id_result;
fatoora_LineItemBuilder_set_id_result fatoora_LineItemBuilder_set_id(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_invoiced_quantity_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_invoiced_quantity_result;
fatoora_LineItemBuilder_set_invoiced_quantity_result fatoora_LineItemBuilder_set_invoiced_quantity(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_item_net_price_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_net_price_result;
fatoora_LineItemBuilder_set_item_net_price_result fatoora_LineItemBuilder_set_item_net_price(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_item_gross_price_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_gross_price_result;
fatoora_LineItemBuilder_set_item_gross_price_result fatoora_LineItemBuilder_set_item_gross_price(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_item_price_discount_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_price_discount_result;
fatoora_LineItemBuilder_set_item_price_discount_result fatoora_LineItemBuilder_set_item_price_discount(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_item_price_base_quantity_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_price_base_quantity_result;
fatoora_LineItemBuilder_set_item_price_base_quantity_result fatoora_LineItemBuilder_set_item_price_base_quantity(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_line_net_amount_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_line_net_amount_result;
fatoora_LineItemBuilder_set_line_net_amount_result fatoora_LineItemBuilder_set_line_net_amount(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_line_vat_amount_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_line_vat_amount_result;
fatoora_LineItemBuilder_set_line_vat_amount_result fatoora_LineItemBuilder_set_line_vat_amount(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_line_amount_with_vat_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_line_amount_with_vat_result;
fatoora_LineItemBuilder_set_line_amount_with_vat_result fatoora_LineItemBuilder_set_line_amount_with_vat(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_vat_rate_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_vat_rate_result;
fatoora_LineItemBuilder_set_vat_rate_result fatoora_LineItemBuilder_set_vat_rate(LineItemBuilder* self, DiplomatStringView value);

typedef struct fatoora_LineItemBuilder_set_vat_category_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_vat_category_result;
fatoora_LineItemBuilder_set_vat_category_result fatoora_LineItemBuilder_set_vat_category(LineItemBuilder* self, uint8_t value);

typedef struct fatoora_LineItemBuilder_set_exemption_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_exemption_result;
fatoora_LineItemBuilder_set_exemption_result fatoora_LineItemBuilder_set_exemption(LineItemBuilder* self, const TaxExemption* value);

typedef struct fatoora_LineItemBuilder_add_allowance_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_allowance_result;
fatoora_LineItemBuilder_add_allowance_result fatoora_LineItemBuilder_add_allowance(LineItemBuilder* self, const Adjustment* value);

typedef struct fatoora_LineItemBuilder_add_charge_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_charge_result;
fatoora_LineItemBuilder_add_charge_result fatoora_LineItemBuilder_add_charge(LineItemBuilder* self, const Adjustment* value);

typedef struct fatoora_LineItemBuilder_add_document_reference_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_document_reference_result;
fatoora_LineItemBuilder_add_document_reference_result fatoora_LineItemBuilder_add_document_reference(LineItemBuilder* self, const OriginalInvoiceRef* value);

typedef struct fatoora_LineItemBuilder_add_tax_subtotal_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_add_tax_subtotal_result;
fatoora_LineItemBuilder_add_tax_subtotal_result fatoora_LineItemBuilder_add_tax_subtotal(LineItemBuilder* self, const VatBreakdown* value);

typedef struct fatoora_LineItemBuilder_set_item_seller_identifier_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_seller_identifier_result;
fatoora_LineItemBuilder_set_item_seller_identifier_result fatoora_LineItemBuilder_set_item_seller_identifier(LineItemBuilder* self, DiplomatStringView value, OptionStringView scheme);

typedef struct fatoora_LineItemBuilder_set_item_buyer_identifier_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_buyer_identifier_result;
fatoora_LineItemBuilder_set_item_buyer_identifier_result fatoora_LineItemBuilder_set_item_buyer_identifier(LineItemBuilder* self, DiplomatStringView value, OptionStringView scheme);

typedef struct fatoora_LineItemBuilder_set_item_standard_identifier_result {union { BindingError* err;}; bool is_ok;} fatoora_LineItemBuilder_set_item_standard_identifier_result;
fatoora_LineItemBuilder_set_item_standard_identifier_result fatoora_LineItemBuilder_set_item_standard_identifier(LineItemBuilder* self, DiplomatStringView value, OptionStringView scheme);

void fatoora_LineItemBuilder_destroy(LineItemBuilder* self);





#endif // LineItemBuilder_H

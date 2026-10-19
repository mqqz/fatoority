#ifndef InvoiceData_H
#define InvoiceData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "AllowanceCharge.d.h"
#include "BindingError.d.h"
#include "InvoiceLineItem.d.h"
#include "InvoiceNote.d.h"
#include "OriginalInvoiceRef.d.h"
#include "Party.d.h"
#include "PaymentMeans.d.h"
#include "Text.d.h"

#include "InvoiceData.d.h"






typedef struct fatoora_InvoiceData_issue_time_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceData_issue_time_result;
fatoora_InvoiceData_issue_time_result fatoora_InvoiceData_issue_time(const InvoiceData* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceData_issue_date_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceData_issue_date_result;
fatoora_InvoiceData_issue_date_result fatoora_InvoiceData_issue_date(const InvoiceData* self, DiplomatWrite* write);

size_t fatoora_InvoiceData_payment_means_len(const InvoiceData* self);

typedef struct fatoora_InvoiceData_payment_means_result {union {PaymentMeans* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_payment_means_result;
fatoora_InvoiceData_payment_means_result fatoora_InvoiceData_payment_means(const InvoiceData* self, size_t index);

typedef struct fatoora_InvoiceData_purchase_order_id_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_purchase_order_id_result;
fatoora_InvoiceData_purchase_order_id_result fatoora_InvoiceData_purchase_order_id(const InvoiceData* self);

typedef struct fatoora_InvoiceData_contract_id_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_contract_id_result;
fatoora_InvoiceData_contract_id_result fatoora_InvoiceData_contract_id(const InvoiceData* self);

typedef struct fatoora_InvoiceData_payment_instruction_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_payment_instruction_result;
fatoora_InvoiceData_payment_instruction_result fatoora_InvoiceData_payment_instruction(const InvoiceData* self);

typedef struct fatoora_InvoiceData_actual_delivery_date_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_actual_delivery_date_result;
fatoora_InvoiceData_actual_delivery_date_result fatoora_InvoiceData_actual_delivery_date(const InvoiceData* self);

typedef struct fatoora_InvoiceData_latest_delivery_date_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_latest_delivery_date_result;
fatoora_InvoiceData_latest_delivery_date_result fatoora_InvoiceData_latest_delivery_date(const InvoiceData* self);

typedef struct fatoora_InvoiceData_tax_currency_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_tax_currency_result;
fatoora_InvoiceData_tax_currency_result fatoora_InvoiceData_tax_currency(const InvoiceData* self);

typedef struct fatoora_InvoiceData_tax_currency_amount_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_tax_currency_amount_result;
fatoora_InvoiceData_tax_currency_amount_result fatoora_InvoiceData_tax_currency_amount(const InvoiceData* self);

size_t fatoora_InvoiceData_adjustments_len(const InvoiceData* self);

typedef struct fatoora_InvoiceData_adjustment_result {union {AllowanceCharge* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_adjustment_result;
fatoora_InvoiceData_adjustment_result fatoora_InvoiceData_adjustment(const InvoiceData* self, size_t index);

typedef struct fatoora_InvoiceData_id_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceData_id_result;
fatoora_InvoiceData_id_result fatoora_InvoiceData_id(const InvoiceData* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceData_uuid_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceData_uuid_result;
fatoora_InvoiceData_uuid_result fatoora_InvoiceData_uuid(const InvoiceData* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceData_previous_invoice_hash_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceData_previous_invoice_hash_result;
fatoora_InvoiceData_previous_invoice_hash_result fatoora_InvoiceData_previous_invoice_hash(const InvoiceData* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceData_payment_means_code_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceData_payment_means_code_result;
fatoora_InvoiceData_payment_means_code_result fatoora_InvoiceData_payment_means_code(const InvoiceData* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceData_currency_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceData_currency_result;
fatoora_InvoiceData_currency_result fatoora_InvoiceData_currency(const InvoiceData* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceData_issue_datetime_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceData_issue_datetime_result;
fatoora_InvoiceData_issue_datetime_result fatoora_InvoiceData_issue_datetime(const InvoiceData* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceData_invoice_level_charge_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceData_invoice_level_charge_result;
fatoora_InvoiceData_invoice_level_charge_result fatoora_InvoiceData_invoice_level_charge(const InvoiceData* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceData_invoice_level_discount_result {union { BindingError* err;}; bool is_ok;} fatoora_InvoiceData_invoice_level_discount_result;
fatoora_InvoiceData_invoice_level_discount_result fatoora_InvoiceData_invoice_level_discount(const InvoiceData* self, DiplomatWrite* write);

typedef struct fatoora_InvoiceData_allowance_reason_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_allowance_reason_result;
fatoora_InvoiceData_allowance_reason_result fatoora_InvoiceData_allowance_reason(const InvoiceData* self);

uint64_t fatoora_InvoiceData_invoice_counter(const InvoiceData* self);

uint8_t fatoora_InvoiceData_vat_category(const InvoiceData* self);

uint8_t fatoora_InvoiceData_flags_raw(const InvoiceData* self);

uint8_t fatoora_InvoiceData_invoice_type_kind(const InvoiceData* self);

uint8_t fatoora_InvoiceData_invoice_sub_type(const InvoiceData* self);

typedef struct fatoora_InvoiceData_seller_result {union {Party* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_seller_result;
fatoora_InvoiceData_seller_result fatoora_InvoiceData_seller(const InvoiceData* self);

typedef struct fatoora_InvoiceData_buyer_result {union {Party* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_buyer_result;
fatoora_InvoiceData_buyer_result fatoora_InvoiceData_buyer(const InvoiceData* self);

typedef struct fatoora_InvoiceData_note_result {union {InvoiceNote* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_note_result;
fatoora_InvoiceData_note_result fatoora_InvoiceData_note(const InvoiceData* self);

size_t fatoora_InvoiceData_line_items_len(const InvoiceData* self);

typedef struct fatoora_InvoiceData_line_item_result {union {InvoiceLineItem* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_line_item_result;
fatoora_InvoiceData_line_item_result fatoora_InvoiceData_line_item(const InvoiceData* self, size_t index);

typedef struct fatoora_InvoiceData_original_invoice_ref_result {union {OriginalInvoiceRef* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_original_invoice_ref_result;
fatoora_InvoiceData_original_invoice_ref_result fatoora_InvoiceData_original_invoice_ref(const InvoiceData* self);

typedef struct fatoora_InvoiceData_original_invoice_reason_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_original_invoice_reason_result;
fatoora_InvoiceData_original_invoice_reason_result fatoora_InvoiceData_original_invoice_reason(const InvoiceData* self);

size_t fatoora_InvoiceData_notes_len(const InvoiceData* self);

typedef struct fatoora_InvoiceData_note_at_result {union {InvoiceNote* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_note_at_result;
fatoora_InvoiceData_note_at_result fatoora_InvoiceData_note_at(const InvoiceData* self, size_t index);

size_t fatoora_InvoiceData_billing_references_len(const InvoiceData* self);

typedef struct fatoora_InvoiceData_billing_reference_result {union {OriginalInvoiceRef* ok; BindingError* err;}; bool is_ok;} fatoora_InvoiceData_billing_reference_result;
fatoora_InvoiceData_billing_reference_result fatoora_InvoiceData_billing_reference(const InvoiceData* self, size_t index);

void fatoora_InvoiceData_destroy(InvoiceData* self);





#endif // InvoiceData_H

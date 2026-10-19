#ifndef AllowanceCharge_H
#define AllowanceCharge_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "TaxExemption.d.h"
#include "Text.d.h"

#include "AllowanceCharge.d.h"






typedef struct fatoora_AllowanceCharge_base_amount_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_base_amount_result;
fatoora_AllowanceCharge_base_amount_result fatoora_AllowanceCharge_base_amount(const AllowanceCharge* self);

typedef struct fatoora_AllowanceCharge_percentage_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_percentage_result;
fatoora_AllowanceCharge_percentage_result fatoora_AllowanceCharge_percentage(const AllowanceCharge* self);

typedef struct fatoora_AllowanceCharge_new_result {union {AllowanceCharge* ok; BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_new_result;
fatoora_AllowanceCharge_new_result fatoora_AllowanceCharge_new(bool charge, DiplomatStringView amount, OptionStringView reason, OptionStringView reason_code, uint8_t category, DiplomatStringView rate, const TaxExemption* exemption);

bool fatoora_AllowanceCharge_is_charge(const AllowanceCharge* self);

typedef struct fatoora_AllowanceCharge_amount_result {union { BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_amount_result;
fatoora_AllowanceCharge_amount_result fatoora_AllowanceCharge_amount(const AllowanceCharge* self, DiplomatWrite* write);

typedef struct fatoora_AllowanceCharge_rate_result {union { BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_rate_result;
fatoora_AllowanceCharge_rate_result fatoora_AllowanceCharge_rate(const AllowanceCharge* self, DiplomatWrite* write);

uint8_t fatoora_AllowanceCharge_category(const AllowanceCharge* self);

typedef struct fatoora_AllowanceCharge_with_tax_category_count_result {union {AllowanceCharge* ok; BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_with_tax_category_count_result;
fatoora_AllowanceCharge_with_tax_category_count_result fatoora_AllowanceCharge_with_tax_category_count(const AllowanceCharge* self, size_t count);

size_t fatoora_AllowanceCharge_tax_category_count(const AllowanceCharge* self);

typedef struct fatoora_AllowanceCharge_reason_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_reason_result;
fatoora_AllowanceCharge_reason_result fatoora_AllowanceCharge_reason(const AllowanceCharge* self);

typedef struct fatoora_AllowanceCharge_reason_code_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_reason_code_result;
fatoora_AllowanceCharge_reason_code_result fatoora_AllowanceCharge_reason_code(const AllowanceCharge* self);

typedef struct fatoora_AllowanceCharge_exemption_result {union {TaxExemption* ok; BindingError* err;}; bool is_ok;} fatoora_AllowanceCharge_exemption_result;
fatoora_AllowanceCharge_exemption_result fatoora_AllowanceCharge_exemption(const AllowanceCharge* self);

void fatoora_AllowanceCharge_destroy(AllowanceCharge* self);





#endif // AllowanceCharge_H

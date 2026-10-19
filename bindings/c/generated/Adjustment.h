#ifndef Adjustment_H
#define Adjustment_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Text.d.h"

#include "Adjustment.d.h"






typedef struct fatoora_Adjustment_amount_result {union {Adjustment* ok; BindingError* err;}; bool is_ok;} fatoora_Adjustment_amount_result;
fatoora_Adjustment_amount_result fatoora_Adjustment_amount(DiplomatStringView value);

typedef struct fatoora_Adjustment_percentage_result {union {Adjustment* ok; BindingError* err;}; bool is_ok;} fatoora_Adjustment_percentage_result;
fatoora_Adjustment_percentage_result fatoora_Adjustment_percentage(DiplomatStringView base, DiplomatStringView percent);

typedef struct fatoora_Adjustment_with_supplied_amount_result {union {Adjustment* ok; BindingError* err;}; bool is_ok;} fatoora_Adjustment_with_supplied_amount_result;
fatoora_Adjustment_with_supplied_amount_result fatoora_Adjustment_with_supplied_amount(const Adjustment* self, DiplomatStringView value);

typedef struct fatoora_Adjustment_with_reason_result {union {Adjustment* ok; BindingError* err;}; bool is_ok;} fatoora_Adjustment_with_reason_result;
fatoora_Adjustment_with_reason_result fatoora_Adjustment_with_reason(const Adjustment* self, OptionStringView reason, OptionStringView code);

typedef struct fatoora_Adjustment_value_result {union { BindingError* err;}; bool is_ok;} fatoora_Adjustment_value_result;
fatoora_Adjustment_value_result fatoora_Adjustment_value(const Adjustment* self, DiplomatWrite* write);

typedef struct fatoora_Adjustment_base_amount_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_Adjustment_base_amount_result;
fatoora_Adjustment_base_amount_result fatoora_Adjustment_base_amount(const Adjustment* self);

typedef struct fatoora_Adjustment_percent_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_Adjustment_percent_result;
fatoora_Adjustment_percent_result fatoora_Adjustment_percent(const Adjustment* self);

typedef struct fatoora_Adjustment_reason_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_Adjustment_reason_result;
fatoora_Adjustment_reason_result fatoora_Adjustment_reason(const Adjustment* self);

typedef struct fatoora_Adjustment_reason_code_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_Adjustment_reason_code_result;
fatoora_Adjustment_reason_code_result fatoora_Adjustment_reason_code(const Adjustment* self);

void fatoora_Adjustment_destroy(Adjustment* self);





#endif // Adjustment_H

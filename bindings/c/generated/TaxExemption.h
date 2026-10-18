#ifndef TaxExemption_H
#define TaxExemption_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Text.d.h"

#include "TaxExemption.d.h"






typedef struct fatoora_TaxExemption_new_result {union {TaxExemption* ok; BindingError* err;}; bool is_ok;} fatoora_TaxExemption_new_result;
fatoora_TaxExemption_new_result fatoora_TaxExemption_new(OptionStringView code, OptionStringView reason);

typedef struct fatoora_TaxExemption_reason_code_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_TaxExemption_reason_code_result;
fatoora_TaxExemption_reason_code_result fatoora_TaxExemption_reason_code(const TaxExemption* self);

typedef struct fatoora_TaxExemption_reason_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_TaxExemption_reason_result;
fatoora_TaxExemption_reason_result fatoora_TaxExemption_reason(const TaxExemption* self);

void fatoora_TaxExemption_destroy(TaxExemption* self);





#endif // TaxExemption_H

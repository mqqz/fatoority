#ifndef VatBreakdown_H
#define VatBreakdown_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "TaxExemption.d.h"

#include "VatBreakdown.d.h"






bool fatoora_VatBreakdown_has_vat_rate(const VatBreakdown* self);

typedef struct fatoora_VatBreakdown_without_vat_rate_result {union {VatBreakdown* ok; BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_without_vat_rate_result;
fatoora_VatBreakdown_without_vat_rate_result fatoora_VatBreakdown_without_vat_rate(const VatBreakdown* self);

typedef struct fatoora_VatBreakdown_new_result {union {VatBreakdown* ok; BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_new_result;
fatoora_VatBreakdown_new_result fatoora_VatBreakdown_new(uint8_t category, DiplomatStringView rate, DiplomatStringView taxable_amount, DiplomatStringView tax_amount, const TaxExemption* exemption);

uint8_t fatoora_VatBreakdown_category(const VatBreakdown* self);

typedef struct fatoora_VatBreakdown_rate_result {union { BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_rate_result;
fatoora_VatBreakdown_rate_result fatoora_VatBreakdown_rate(const VatBreakdown* self, DiplomatWrite* write);

typedef struct fatoora_VatBreakdown_taxable_amount_result {union { BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_taxable_amount_result;
fatoora_VatBreakdown_taxable_amount_result fatoora_VatBreakdown_taxable_amount(const VatBreakdown* self, DiplomatWrite* write);

typedef struct fatoora_VatBreakdown_tax_amount_result {union { BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_tax_amount_result;
fatoora_VatBreakdown_tax_amount_result fatoora_VatBreakdown_tax_amount(const VatBreakdown* self, DiplomatWrite* write);

typedef struct fatoora_VatBreakdown_exemption_result {union {TaxExemption* ok; BindingError* err;}; bool is_ok;} fatoora_VatBreakdown_exemption_result;
fatoora_VatBreakdown_exemption_result fatoora_VatBreakdown_exemption(const VatBreakdown* self);

void fatoora_VatBreakdown_destroy(VatBreakdown* self);





#endif // VatBreakdown_H

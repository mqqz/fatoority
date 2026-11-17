#ifndef ZatcaRuleSource_H
#define ZatcaRuleSource_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"

#include "ZatcaRuleSource.d.h"






typedef struct fatoora_ZatcaRuleSource_stylesheet_result {union { BindingError* err;}; bool is_ok;} fatoora_ZatcaRuleSource_stylesheet_result;
fatoora_ZatcaRuleSource_stylesheet_result fatoora_ZatcaRuleSource_stylesheet(const ZatcaRuleSource* self, DiplomatWrite* write);

typedef struct fatoora_ZatcaRuleSource_sha256_result {union { BindingError* err;}; bool is_ok;} fatoora_ZatcaRuleSource_sha256_result;
fatoora_ZatcaRuleSource_sha256_result fatoora_ZatcaRuleSource_sha256(const ZatcaRuleSource* self, DiplomatWrite* write);

void fatoora_ZatcaRuleSource_destroy(ZatcaRuleSource* self);





#endif // ZatcaRuleSource_H

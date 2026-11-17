#ifndef ZatcaValidationReport_H
#define ZatcaValidationReport_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "ZatcaStageReport.d.h"

#include "ZatcaValidationReport.d.h"






uint32_t fatoora_ZatcaValidationReport_schema_version(const ZatcaValidationReport* self);

typedef struct fatoora_ZatcaValidationReport_profile_result {union { BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_profile_result;
fatoora_ZatcaValidationReport_profile_result fatoora_ZatcaValidationReport_profile(const ZatcaValidationReport* self, DiplomatWrite* write);

typedef struct fatoora_ZatcaValidationReport_evaluated_at_result {union { BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_evaluated_at_result;
fatoora_ZatcaValidationReport_evaluated_at_result fatoora_ZatcaValidationReport_evaluated_at(const ZatcaValidationReport* self, DiplomatWrite* write);

bool fatoora_ZatcaValidationReport_is_complete(const ZatcaValidationReport* self);

bool fatoora_ZatcaValidationReport_has_errors(const ZatcaValidationReport* self);

bool fatoora_ZatcaValidationReport_is_valid(const ZatcaValidationReport* self);

size_t fatoora_ZatcaValidationReport_stages_len(const ZatcaValidationReport* self);

typedef struct fatoora_ZatcaValidationReport_stage_result {union {ZatcaStageReport* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_stage_result;
fatoora_ZatcaValidationReport_stage_result fatoora_ZatcaValidationReport_stage(const ZatcaValidationReport* self, size_t index);

typedef struct fatoora_ZatcaValidationReport_to_json_result {union { BindingError* err;}; bool is_ok;} fatoora_ZatcaValidationReport_to_json_result;
fatoora_ZatcaValidationReport_to_json_result fatoora_ZatcaValidationReport_to_json(const ZatcaValidationReport* self, DiplomatWrite* write);

void fatoora_ZatcaValidationReport_destroy(ZatcaValidationReport* self);





#endif // ZatcaValidationReport_H

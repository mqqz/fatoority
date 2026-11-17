#ifndef ZatcaStageReport_H
#define ZatcaStageReport_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Text.d.h"
#include "ZatcaFinding.d.h"
#include "ZatcaRuleSource.d.h"
#include "ZatcaStage.d.h"
#include "ZatcaStageStatus.d.h"

#include "ZatcaStageReport.d.h"






ZatcaStage fatoora_ZatcaStageReport_stage(const ZatcaStageReport* self);

ZatcaStageStatus fatoora_ZatcaStageReport_status(const ZatcaStageReport* self);

typedef struct fatoora_ZatcaStageReport_provenance_result {union {ZatcaRuleSource* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaStageReport_provenance_result;
fatoora_ZatcaStageReport_provenance_result fatoora_ZatcaStageReport_provenance(const ZatcaStageReport* self);

size_t fatoora_ZatcaStageReport_evaluated_assertions_len(const ZatcaStageReport* self);

typedef struct fatoora_ZatcaStageReport_evaluated_assertion_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaStageReport_evaluated_assertion_result;
fatoora_ZatcaStageReport_evaluated_assertion_result fatoora_ZatcaStageReport_evaluated_assertion(const ZatcaStageReport* self, size_t index);

size_t fatoora_ZatcaStageReport_findings_len(const ZatcaStageReport* self);

typedef struct fatoora_ZatcaStageReport_finding_result {union {ZatcaFinding* ok; BindingError* err;}; bool is_ok;} fatoora_ZatcaStageReport_finding_result;
fatoora_ZatcaStageReport_finding_result fatoora_ZatcaStageReport_finding(const ZatcaStageReport* self, size_t index);

void fatoora_ZatcaStageReport_destroy(ZatcaStageReport* self);





#endif // ZatcaStageReport_H

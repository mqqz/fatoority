#ifndef ZatcaFinding_H
#define ZatcaFinding_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Severity.d.h"
#include "ValidationLayer.d.h"
#include "ValidationLocationKind.d.h"

#include "ZatcaFinding.d.h"






typedef struct fatoora_ZatcaFinding_assertion_site_result { bool is_ok;} fatoora_ZatcaFinding_assertion_site_result;
fatoora_ZatcaFinding_assertion_site_result fatoora_ZatcaFinding_assertion_site(const ZatcaFinding* self, DiplomatWrite* write);

ValidationLayer fatoora_ZatcaFinding_layer(const ZatcaFinding* self);

typedef struct fatoora_ZatcaFinding_code_result {union { BindingError* err;}; bool is_ok;} fatoora_ZatcaFinding_code_result;
fatoora_ZatcaFinding_code_result fatoora_ZatcaFinding_code(const ZatcaFinding* self, DiplomatWrite* write);

Severity fatoora_ZatcaFinding_severity(const ZatcaFinding* self);

typedef struct fatoora_ZatcaFinding_message_result {union { BindingError* err;}; bool is_ok;} fatoora_ZatcaFinding_message_result;
fatoora_ZatcaFinding_message_result fatoora_ZatcaFinding_message(const ZatcaFinding* self, DiplomatWrite* write);

typedef struct fatoora_ZatcaFinding_location_kind_result {union {ValidationLocationKind ok; }; bool is_ok;} fatoora_ZatcaFinding_location_kind_result;
fatoora_ZatcaFinding_location_kind_result fatoora_ZatcaFinding_location_kind(const ZatcaFinding* self);

typedef struct fatoora_ZatcaFinding_location_path_result { bool is_ok;} fatoora_ZatcaFinding_location_path_result;
fatoora_ZatcaFinding_location_path_result fatoora_ZatcaFinding_location_path(const ZatcaFinding* self, DiplomatWrite* write);

typedef struct fatoora_ZatcaFinding_location_line_result {union {uint32_t ok; }; bool is_ok;} fatoora_ZatcaFinding_location_line_result;
fatoora_ZatcaFinding_location_line_result fatoora_ZatcaFinding_location_line(const ZatcaFinding* self);

typedef struct fatoora_ZatcaFinding_location_column_result {union {uint32_t ok; }; bool is_ok;} fatoora_ZatcaFinding_location_column_result;
fatoora_ZatcaFinding_location_column_result fatoora_ZatcaFinding_location_column(const ZatcaFinding* self);

void fatoora_ZatcaFinding_destroy(ZatcaFinding* self);





#endif // ZatcaFinding_H

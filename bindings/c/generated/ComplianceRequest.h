#ifndef ComplianceRequest_H
#define ComplianceRequest_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Bytes.d.h"
#include "ComplianceCsid.d.h"

#include "ComplianceRequest.d.h"






typedef struct fatoora_ComplianceRequest_method_result {union { BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_method_result;
fatoora_ComplianceRequest_method_result fatoora_ComplianceRequest_method(const ComplianceRequest* self, DiplomatWrite* write);

typedef struct fatoora_ComplianceRequest_url_result {union { BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_url_result;
fatoora_ComplianceRequest_url_result fatoora_ComplianceRequest_url(const ComplianceRequest* self, DiplomatWrite* write);

typedef struct fatoora_ComplianceRequest_headers_json_result {union { BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_headers_json_result;
fatoora_ComplianceRequest_headers_json_result fatoora_ComplianceRequest_headers_json(const ComplianceRequest* self, DiplomatWrite* write);

typedef struct fatoora_ComplianceRequest_body_result {union {Bytes* ok; BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_body_result;
fatoora_ComplianceRequest_body_result fatoora_ComplianceRequest_body(const ComplianceRequest* self);

typedef struct fatoora_ComplianceRequest_decode_result {union {ComplianceCsid* ok; BindingError* err;}; bool is_ok;} fatoora_ComplianceRequest_decode_result;
fatoora_ComplianceRequest_decode_result fatoora_ComplianceRequest_decode(const ComplianceRequest* self, uint16_t status, DiplomatU8View body);

void fatoora_ComplianceRequest_destroy(ComplianceRequest* self);





#endif // ComplianceRequest_H

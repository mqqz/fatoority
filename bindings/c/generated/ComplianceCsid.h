#ifndef ComplianceCsid_H
#define ComplianceCsid_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Environment.d.h"
#include "Text.d.h"

#include "ComplianceCsid.d.h"






typedef struct fatoora_ComplianceCsid_new_result {union {ComplianceCsid* ok; BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_new_result;
fatoora_ComplianceCsid_new_result fatoora_ComplianceCsid_new(Environment environment, OptionStringView request_id, DiplomatStringView token, DiplomatStringView secret);

typedef struct fatoora_ComplianceCsid_env_result {union {Environment ok; BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_env_result;
fatoora_ComplianceCsid_env_result fatoora_ComplianceCsid_env(const ComplianceCsid* self);

typedef struct fatoora_ComplianceCsid_request_id_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_request_id_result;
fatoora_ComplianceCsid_request_id_result fatoora_ComplianceCsid_request_id(const ComplianceCsid* self);

typedef struct fatoora_ComplianceCsid_binary_security_token_result {union { BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_binary_security_token_result;
fatoora_ComplianceCsid_binary_security_token_result fatoora_ComplianceCsid_binary_security_token(const ComplianceCsid* self, DiplomatWrite* write);

typedef struct fatoora_ComplianceCsid_secret_result {union { BindingError* err;}; bool is_ok;} fatoora_ComplianceCsid_secret_result;
fatoora_ComplianceCsid_secret_result fatoora_ComplianceCsid_secret(const ComplianceCsid* self, DiplomatWrite* write);

void fatoora_ComplianceCsid_destroy(ComplianceCsid* self);





#endif // ComplianceCsid_H

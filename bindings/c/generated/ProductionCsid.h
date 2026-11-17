#ifndef ProductionCsid_H
#define ProductionCsid_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Environment.d.h"
#include "Text.d.h"

#include "ProductionCsid.d.h"






typedef struct fatoora_ProductionCsid_new_result {union {ProductionCsid* ok; BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_new_result;
fatoora_ProductionCsid_new_result fatoora_ProductionCsid_new(Environment environment, OptionStringView request_id, DiplomatStringView token, DiplomatStringView secret);

typedef struct fatoora_ProductionCsid_env_result {union {Environment ok; BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_env_result;
fatoora_ProductionCsid_env_result fatoora_ProductionCsid_env(const ProductionCsid* self);

typedef struct fatoora_ProductionCsid_request_id_result {union {Text* ok; BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_request_id_result;
fatoora_ProductionCsid_request_id_result fatoora_ProductionCsid_request_id(const ProductionCsid* self);

typedef struct fatoora_ProductionCsid_binary_security_token_result {union { BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_binary_security_token_result;
fatoora_ProductionCsid_binary_security_token_result fatoora_ProductionCsid_binary_security_token(const ProductionCsid* self, DiplomatWrite* write);

typedef struct fatoora_ProductionCsid_secret_result {union { BindingError* err;}; bool is_ok;} fatoora_ProductionCsid_secret_result;
fatoora_ProductionCsid_secret_result fatoora_ProductionCsid_secret(const ProductionCsid* self, DiplomatWrite* write);

void fatoora_ProductionCsid_destroy(ProductionCsid* self);





#endif // ProductionCsid_H

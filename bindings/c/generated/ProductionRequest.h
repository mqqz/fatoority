#ifndef ProductionRequest_H
#define ProductionRequest_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Bytes.d.h"
#include "ProductionCsid.d.h"

#include "ProductionRequest.d.h"






typedef struct fatoora_ProductionRequest_method_result {union { BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_method_result;
fatoora_ProductionRequest_method_result fatoora_ProductionRequest_method(const ProductionRequest* self, DiplomatWrite* write);

typedef struct fatoora_ProductionRequest_url_result {union { BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_url_result;
fatoora_ProductionRequest_url_result fatoora_ProductionRequest_url(const ProductionRequest* self, DiplomatWrite* write);

typedef struct fatoora_ProductionRequest_headers_json_result {union { BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_headers_json_result;
fatoora_ProductionRequest_headers_json_result fatoora_ProductionRequest_headers_json(const ProductionRequest* self, DiplomatWrite* write);

typedef struct fatoora_ProductionRequest_body_result {union {Bytes* ok; BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_body_result;
fatoora_ProductionRequest_body_result fatoora_ProductionRequest_body(const ProductionRequest* self);

typedef struct fatoora_ProductionRequest_decode_result {union {ProductionCsid* ok; BindingError* err;}; bool is_ok;} fatoora_ProductionRequest_decode_result;
fatoora_ProductionRequest_decode_result fatoora_ProductionRequest_decode(const ProductionRequest* self, uint16_t status, DiplomatU8View body);

void fatoora_ProductionRequest_destroy(ProductionRequest* self);





#endif // ProductionRequest_H

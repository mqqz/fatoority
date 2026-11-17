#ifndef Config_H
#define Config_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "BindingError.d.h"
#include "Environment.d.h"

#include "Config.d.h"






typedef struct fatoora_Config_new_result {union {Config* ok; BindingError* err;}; bool is_ok;} fatoora_Config_new_result;
fatoora_Config_new_result fatoora_Config_new(Environment env);

typedef struct fatoora_Config_env_result {union {Environment ok; BindingError* err;}; bool is_ok;} fatoora_Config_env_result;
fatoora_Config_env_result fatoora_Config_env(const Config* self);

void fatoora_Config_destroy(Config* self);





#endif // Config_H

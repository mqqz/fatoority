#ifndef ValidationStatus_D_H
#define ValidationStatus_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ValidationStatus {
  ValidationStatus_Unknown = 0,
  ValidationStatus_Pass = 1,
  ValidationStatus_Warning = 2,
  ValidationStatus_Error = 3,
} ValidationStatus;

typedef struct ValidationStatus_option {union { ValidationStatus ok; }; bool is_ok; } ValidationStatus_option;



#endif // ValidationStatus_D_H

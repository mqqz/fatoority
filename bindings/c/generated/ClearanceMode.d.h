#ifndef ClearanceMode_D_H
#define ClearanceMode_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ClearanceMode {
  ClearanceMode_Disabled = 0,
  ClearanceMode_Enabled = 1,
} ClearanceMode;

typedef struct ClearanceMode_option {union { ClearanceMode ok; }; bool is_ok; } ClearanceMode_option;



#endif // ClearanceMode_D_H

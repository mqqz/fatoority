#ifndef AdjustmentKind_D_H
#define AdjustmentKind_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum AdjustmentKind {
  AdjustmentKind_Allowance = 0,
  AdjustmentKind_Charge = 1,
} AdjustmentKind;

typedef struct AdjustmentKind_option {union { AdjustmentKind ok; }; bool is_ok; } AdjustmentKind_option;



#endif // AdjustmentKind_D_H

#ifndef ClearanceStatus_D_H
#define ClearanceStatus_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ClearanceStatus {
  ClearanceStatus_Unknown = 0,
  ClearanceStatus_Cleared = 1,
  ClearanceStatus_NotCleared = 2,
} ClearanceStatus;

typedef struct ClearanceStatus_option {union { ClearanceStatus ok; }; bool is_ok; } ClearanceStatus_option;



#endif // ClearanceStatus_D_H

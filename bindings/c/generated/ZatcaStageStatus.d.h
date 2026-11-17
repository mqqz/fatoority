#ifndef ZatcaStageStatus_D_H
#define ZatcaStageStatus_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ZatcaStageStatus {
  ZatcaStageStatus_Unknown = 0,
  ZatcaStageStatus_NotRun = 1,
  ZatcaStageStatus_Completed = 2,
  ZatcaStageStatus_NotApplicable = 3,
  ZatcaStageStatus_ContextRequired = 4,
  ZatcaStageStatus_EvaluationFailed = 5,
} ZatcaStageStatus;

typedef struct ZatcaStageStatus_option {union { ZatcaStageStatus ok; }; bool is_ok; } ZatcaStageStatus_option;



#endif // ZatcaStageStatus_D_H

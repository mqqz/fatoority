#ifndef ReportingStatus_D_H
#define ReportingStatus_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ReportingStatus {
  ReportingStatus_Unknown = 0,
  ReportingStatus_Reported = 1,
  ReportingStatus_NotReported = 2,
} ReportingStatus;

typedef struct ReportingStatus_option {union { ReportingStatus ok; }; bool is_ok; } ReportingStatus_option;



#endif // ReportingStatus_D_H

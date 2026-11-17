#ifndef Severity_D_H
#define Severity_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum Severity {
  Severity_Warning = 0,
  Severity_Error = 1,
} Severity;

typedef struct Severity_option {union { Severity ok; }; bool is_ok; } Severity_option;



#endif // Severity_D_H

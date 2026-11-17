#ifndef ValidationLocationKind_D_H
#define ValidationLocationKind_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ValidationLocationKind {
  ValidationLocationKind_Unknown = 0,
  ValidationLocationKind_Field = 1,
  ValidationLocationKind_XPath = 2,
  ValidationLocationKind_Xml = 3,
} ValidationLocationKind;

typedef struct ValidationLocationKind_option {union { ValidationLocationKind ok; }; bool is_ok; } ValidationLocationKind_option;



#endif // ValidationLocationKind_D_H

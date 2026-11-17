#ifndef VatCategory_D_H
#define VatCategory_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum VatCategory {
  VatCategory_Exempt = 0,
  VatCategory_Standard = 1,
  VatCategory_Zero = 2,
  VatCategory_OutOfScope = 3,
} VatCategory;

typedef struct VatCategory_option {union { VatCategory ok; }; bool is_ok; } VatCategory_option;



#endif // VatCategory_D_H

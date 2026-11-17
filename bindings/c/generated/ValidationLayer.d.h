#ifndef ValidationLayer_D_H
#define ValidationLayer_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ValidationLayer {
  ValidationLayer_Unknown = 0,
  ValidationLayer_FieldChecks = 1,
  ValidationLayer_Xsd = 2,
  ValidationLayer_BusinessRules = 3,
  ValidationLayer_Signature = 4,
  ValidationLayer_Qr = 5,
  ValidationLayer_PreviousInvoiceHash = 6,
} ValidationLayer;

typedef struct ValidationLayer_option {union { ValidationLayer ok; }; bool is_ok; } ValidationLayer_option;



#endif // ValidationLayer_D_H

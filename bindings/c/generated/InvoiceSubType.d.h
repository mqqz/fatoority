#ifndef InvoiceSubType_D_H
#define InvoiceSubType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum InvoiceSubType {
  InvoiceSubType_Standard = 0,
  InvoiceSubType_Simplified = 1,
} InvoiceSubType;

typedef struct InvoiceSubType_option {union { InvoiceSubType ok; }; bool is_ok; } InvoiceSubType_option;



#endif // InvoiceSubType_D_H

#ifndef InvoiceFlag_D_H
#define InvoiceFlag_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum InvoiceFlag {
  InvoiceFlag_ThirdParty = 1,
  InvoiceFlag_Nominal = 2,
  InvoiceFlag_Export = 4,
  InvoiceFlag_Summary = 8,
  InvoiceFlag_SelfBilled = 16,
} InvoiceFlag;

typedef struct InvoiceFlag_option {union { InvoiceFlag ok; }; bool is_ok; } InvoiceFlag_option;



#endif // InvoiceFlag_D_H

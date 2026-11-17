#ifndef InvoiceTypeKind_D_H
#define InvoiceTypeKind_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum InvoiceTypeKind {
  InvoiceTypeKind_Tax = 0,
  InvoiceTypeKind_Prepayment = 1,
  InvoiceTypeKind_CreditNote = 2,
  InvoiceTypeKind_DebitNote = 3,
} InvoiceTypeKind;

typedef struct InvoiceTypeKind_option {union { InvoiceTypeKind ok; }; bool is_ok; } InvoiceTypeKind_option;



#endif // InvoiceTypeKind_D_H

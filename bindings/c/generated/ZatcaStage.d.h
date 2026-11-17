#ifndef ZatcaStage_D_H
#define ZatcaStage_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ZatcaStage {
  ZatcaStage_Unknown = 0,
  ZatcaStage_Xsd = 1,
  ZatcaStage_Cen = 2,
  ZatcaStage_Ksa = 3,
  ZatcaStage_Signature = 4,
  ZatcaStage_Qr = 5,
  ZatcaStage_PreviousInvoiceHash = 6,
} ZatcaStage;

typedef struct ZatcaStage_option {union { ZatcaStage ok; }; bool is_ok; } ZatcaStage_option;



#endif // ZatcaStage_D_H

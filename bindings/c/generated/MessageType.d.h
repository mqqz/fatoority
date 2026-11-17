#ifndef MessageType_D_H
#define MessageType_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum MessageType {
  MessageType_Unknown = 0,
  MessageType_Info = 1,
  MessageType_Warning = 2,
  MessageType_Error = 3,
} MessageType;

typedef struct MessageType_option {union { MessageType ok; }; bool is_ok; } MessageType_option;



#endif // MessageType_D_H

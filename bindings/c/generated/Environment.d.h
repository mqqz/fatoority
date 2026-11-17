#ifndef Environment_D_H
#define Environment_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum Environment {
  Environment_NonProduction = 0,
  Environment_Simulation = 1,
  Environment_Production = 2,
} Environment;

typedef struct Environment_option {union { Environment ok; }; bool is_ok; } Environment_option;



#endif // Environment_D_H

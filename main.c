#include <stdio.h>
#include <stdlib.h>

#include "secp.h"

int main(int argc, char** argv) {
  if (argc < 2) return 2;
  u8 x = atoi(argv[1]);

  init();

  u8 result = Z_runZ_ii(x);

  return result;
}

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
  const uint8_t *bytes;
  size_t len;
} RustByteSlice;

RustByteSlice get_string_from_rust(void);

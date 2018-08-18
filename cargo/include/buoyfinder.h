#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct Store_AppState__Actions Store_AppState__Actions;

void store_free(Store_AppState__Actions *data);

Store_AppState__Actions *store_new(void);

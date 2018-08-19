#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct BuoyStation BuoyStation;

typedef struct ExploreViewData ExploreViewData;

typedef struct Store_AppState__Actions Store_AppState__Actions;

void buoy_station_free(BuoyStation *data);

const char *buoy_station_id(const BuoyStation *data);

const char *buoy_station_name(const BuoyStation *data);

BuoyStation *buoy_station_new(const char *station_id, const char *name, double lat, double lon);

void explore_view_data_free(ExploreViewData *data);

ExploreViewData *explore_view_data_new(void);

int64_t explore_view_data_station_count(const ExploreViewData *data);

BuoyStation *explore_view_data_station_index(const ExploreViewData *data, int64_t index);

void store_free(Store_AppState__Actions *data);

Store_AppState__Actions *store_new(void);

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct BuoyStation BuoyStation;

typedef struct ExploreViewData ExploreViewData;

typedef struct ExploreViewModelHandle ExploreViewModelHandle;

typedef struct Store_AppState__Actions Store_AppState__Actions;

typedef struct {
  void (*new_view_data)(ExploreViewData*);
} explore_view;

bool buoy_station_active(const BuoyStation *data);

void buoy_station_free(BuoyStation *data);

const char *buoy_station_id(const BuoyStation *data);

const char *buoy_station_name(const BuoyStation *data);

BuoyStation *buoy_station_new(const char *station_id, const char *name, double lat, double lon);

ExploreViewModelHandle *explore_view_bind(explore_view view, Store_AppState__Actions *store);

void explore_view_data_free(ExploreViewData *data);

ExploreViewData *explore_view_data_new(void);

int64_t explore_view_data_station_count(const ExploreViewData *data);

BuoyStation *explore_view_data_station_index(const ExploreViewData *data, int64_t index);

void explore_view_unbind(ExploreViewModelHandle *view_model_handle, Store_AppState__Actions *store);

void store_free(Store_AppState__Actions *data);

Store_AppState__Actions *store_new(void);

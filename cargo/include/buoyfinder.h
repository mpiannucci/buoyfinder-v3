#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

typedef enum {
  FixedStation,
  Buoy,
  Tides,
  Tsunami,
  Unknown,
} BuoyStationIcon;

typedef struct BuoyStation BuoyStation;

typedef struct BuoyStationItemViewData BuoyStationItemViewData;

typedef struct ExploreViewData ExploreViewData;

typedef struct ExploreViewModelHandle ExploreViewModelHandle;

typedef struct Location Location;

typedef struct Store_AppState__Actions Store_AppState__Actions;

typedef struct {
  double red;
  double green;
  double blue;
} Color;

typedef struct {
  void *view;
  void (*new_view_data)(void*, ExploreViewData*);
} explore_view;

bool buoy_station_active(const BuoyStation *buoy_station);

void buoy_station_free(BuoyStation *buoy_station);

const char *buoy_station_id(const BuoyStation *buoy_station);

Color buoy_station_item_view_data_color(const BuoyStationItemViewData *data);

void buoy_station_item_view_data_free(BuoyStationItemViewData *data);

BuoyStationIcon buoy_station_item_view_data_icon(const BuoyStationItemViewData *data);

Location *buoy_station_item_view_data_location(const BuoyStationItemViewData *data);

const char *buoy_station_item_view_data_on_click_id(const BuoyStationItemViewData *data);

const char *buoy_station_item_view_data_subtitle(const BuoyStationItemViewData *data);

const char *buoy_station_item_view_data_title(const BuoyStationItemViewData *data);

const char *buoy_station_name(const BuoyStation *buoy_station);

BuoyStation *buoy_station_new(const char *station_id, const char *name, double lat, double lon);

ExploreViewModelHandle *explore_view_bind(explore_view view, Store_AppState__Actions *store);

void explore_view_data_free(ExploreViewData *data);

ExploreViewData *explore_view_data_new(void);

int64_t explore_view_data_station_count(const ExploreViewData *data);

BuoyStationItemViewData *explore_view_data_station_index(const ExploreViewData *data,
                                                         int64_t index);

void explore_view_unbind(ExploreViewModelHandle *view_model_handle, Store_AppState__Actions *store);

void fetch_buoy_stations(Store_AppState__Actions *store);

double location_altitude(const Location *data);

void location_free(Location *data);

double location_latitude(const Location *data);

double location_longitude(const Location *data);

const char *location_name(const Location *data);

Location *location_new(double latitude, double longitude, const char *name);

void store_free(Store_AppState__Actions *store);

Store_AppState__Actions *store_new(void);

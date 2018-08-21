use libc::size_t;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ops::Deref;
use std::boxed::Box;
use std::sync::Arc;
use std::sync::Mutex;
use redux::{Store};
use app::{Actions, DataState, AppState, app_reducer, fetch_buoy_stations_remote};
use vm::{ExploreViewData, ExploreView, ExploreViewModel};
use station::{BuoyStation};
use location::Location;

#[repr(C)]
pub struct RustByteSlice {
    pub bytes: *const u8,
    pub len: size_t,
}

pub fn c_char_to_string(cchar: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(cchar) };
    let r_str = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };
    r_str.to_string()
}

pub fn string_to_c_char(r_string: String) -> *mut c_char {
    CString::new(r_string).unwrap().into_raw()
}

#[no_mangle]
pub extern fn store_new() -> *mut Store<AppState, Actions> {
    let default_state = AppState::default();
    let store = Store::create(&default_state, app_reducer);
    let boxed_store = Box::new(store);
    Box::into_raw(boxed_store)
}

#[no_mangle]
pub unsafe extern fn store_free(store: *mut Store<AppState, Actions>) {
    let _ = Box::from_raw(store);
}

#[no_mangle]
pub unsafe extern fn fetch_buoy_stations(store: *mut Store<AppState, Actions>) {
    let store = &mut*store;
    let stations = fetch_buoy_stations_remote();
    store.dispatch(&Actions::SetBuoyStations(stations));
}

#[repr(C)]
pub struct explore_view {
    pub new_view_data: extern fn(view_data: *mut ExploreViewData),
}

struct ExploreViewWrapper(explore_view);

impl Deref for ExploreViewWrapper {
    type Target = explore_view;
    fn deref(&self) -> &explore_view {
        &self.0
    }
}

impl ExploreView for ExploreViewWrapper {
    fn new_view_data(&mut self, view_data: &ExploreViewData) {
        let view_data = Box::into_raw(Box::new(view_data.clone()));
        (self.new_view_data)(view_data);
    }
}

pub struct ExploreViewModelHandle(pub Arc<Mutex<ExploreViewModel>>);

#[no_mangle]
pub unsafe extern fn explore_view_bind(view: explore_view, store: *mut Store<AppState, Actions>) -> *mut ExploreViewModelHandle {
    let explore_view_wrapper = Box::new(ExploreViewWrapper(view));
    let explore_view_model = Arc::new(Mutex::new(ExploreViewModel::new(explore_view_wrapper)));
    let explore_view_model_handle = Box::new(ExploreViewModelHandle(explore_view_model));
    let store = &mut*store;
    store.subscribe(explore_view_model_handle.0.clone());
    Box::into_raw(explore_view_model_handle)
}

#[no_mangle]
pub unsafe extern fn explore_view_unbind(view_model_handle: *mut ExploreViewModelHandle, store: *mut Store<AppState, Actions>) {
    let explore_view_model_handle = Box::from_raw(view_model_handle);
    let store = &mut*store;
    store.unsubscribe(explore_view_model_handle.0);
}

#[no_mangle]
pub extern fn explore_view_data_new() -> *mut ExploreViewData {
    let view_data = ExploreViewData::from_state(&DataState::NoData);
    let boxed_view_data = Box::new(view_data);
    Box::into_raw(boxed_view_data)
}

#[no_mangle]
pub unsafe extern fn explore_view_data_free(data: *mut ExploreViewData) {
    let _ = Box::from_raw(data);
}

#[no_mangle]
pub unsafe extern fn explore_view_data_station_count(data: *const ExploreViewData) -> i64 {
    let view_data = &*data;
    view_data.stations.len() as i64
}

#[no_mangle]
pub unsafe extern fn explore_view_data_station_index(data: *const ExploreViewData, index: i64) -> *mut BuoyStation {
    let view_data = &*data;
    let boxed_station = Box::new(view_data.stations[index as usize].clone());
    Box::into_raw(boxed_station)
}

#[no_mangle]
pub unsafe extern fn buoy_station_new(station_id: *const c_char, name: *const c_char, lat: f64, lon: f64) -> *mut BuoyStation {
    let station_id = c_char_to_string(station_id);
    let name = c_char_to_string(name);

    let location = Location::new(lat, lon, name);
    let buoy_station = BuoyStation::new(station_id, location);
    let boxed_buoy_station = Box::new(buoy_station);
    Box::into_raw(boxed_buoy_station)
}

#[no_mangle]
pub unsafe extern fn buoy_station_free(buoy_station: *mut BuoyStation) {
    let _ = Box::from_raw(buoy_station);
}

#[no_mangle]
pub unsafe extern fn buoy_station_id(buoy_station: *const BuoyStation) -> *const c_char {
    let buoy_station = &*buoy_station;
    string_to_c_char(buoy_station.station_id.clone())
}

#[no_mangle]
pub unsafe extern fn buoy_station_name(buoy_station: *const BuoyStation) -> *const c_char {
    let buoy_station = &*buoy_station;
    string_to_c_char(buoy_station.location.name.clone())
}

#[no_mangle]
pub unsafe extern fn buoy_station_active(buoy_station: *const BuoyStation) -> bool {
    let buoy_station = &*buoy_station;
    buoy_station.active
}


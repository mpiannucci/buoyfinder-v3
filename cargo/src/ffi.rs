use libc::size_t;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::boxed::Box;
use std::sync::Arc;
use std::sync::Weak;
use std::cell::RefCell;
use redux::Store;
use app::{Actions, DataState, AppState, app_reducer};
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
pub unsafe extern fn store_free(data: *mut Store<AppState, Actions>) {
    let _ = Box::from_raw(data);
}

#[no_mangle]
pub extern fn explore_view_data_new() -> *mut ExploreViewData {
    let explore_view_data = ExploreViewData::from_state(&DataState::NoData);
    let boxed_explore_view_data = Box::new(explore_view_data);
    Box::into_raw(boxed_explore_view_data)
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
pub unsafe extern fn buoy_station_free(data: *mut BuoyStation) {
    let _ = Box::from_raw(data);
}

#[no_mangle]
pub unsafe extern fn buoy_station_id(data: *const BuoyStation) -> *const c_char {
    let buoy_station = &*data;
    string_to_c_char(buoy_station.station_id.clone())
}

#[no_mangle]
pub unsafe extern fn buoy_station_name(data: *const BuoyStation) -> *const c_char {
    let buoy_station = &*data;
    string_to_c_char(buoy_station.location.name.clone())
}
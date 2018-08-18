use libc::size_t;
use std::ffi::CStr;
use std::boxed::Box;
use std::sync::Arc;
use std::sync::Weak;
use std::cell::RefCell;
use redux::Store;
use app::{Actions, DataState, AppState, app_reducer};
use vm::{ExploreViewData, ExploreView, ExploreViewModel};

#[repr(C)]
pub struct RustByteSlice {
    pub bytes: *const u8,
    pub len: size_t,
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
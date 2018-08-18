use libc::size_t;
use std::boxed::Box;
use redux::Store;
use app::{Actions, AppState, app_reducer};

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
use libc::size_t;
use libc::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ops::Deref;
use std::boxed::Box;
use std::sync::Arc;
use std::sync::Mutex;
use redux::{Store};
use app::{Actions, DataState, AppState, AppReducer, fetch_buoy_stations_remote};
use vm::{ExploreViewData, ExploreView, ExploreViewModel, BuoyStationItemViewData, BuoyStationIcon, Color};
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
    let app_reducer = Box::new(AppReducer{});
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
    store.dispatch(Actions::SetBuoyStationsLoading);
    let stations = fetch_buoy_stations_remote();
    store.dispatch(Actions::SetBuoyStations(stations));
}

#[repr(C)]
pub struct explore_view {
    pub view: *mut c_void,

    pub new_view_data: extern fn(view: *mut c_void, view_data: *mut ExploreViewData),
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
        (self.new_view_data)(self.view, view_data);
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
pub unsafe extern fn explore_view_data_is_loading(data: *const ExploreViewData) -> bool {
    let view_data = &*data;
    view_data.is_loading
}

#[no_mangle]
pub unsafe extern fn explore_view_data_station_count(data: *const ExploreViewData) -> i64 {
    let view_data = &*data;
    view_data.stations.len() as i64
}

#[no_mangle]
pub unsafe extern fn explore_view_data_station_index(data: *const ExploreViewData, index: i64) -> *mut BuoyStationItemViewData {
    let view_data = &*data;
    let boxed_station = Box::new(view_data.stations[index as usize].clone());
    Box::into_raw(boxed_station)
}

#[no_mangle]
pub unsafe extern fn location_new(latitude: f64, longitude: f64, name: *const c_char) -> *mut Location {
    let boxed_location = Box::new(Location::new(latitude, longitude, c_char_to_string(name)));
    Box::into_raw(boxed_location)
}

#[no_mangle]
pub unsafe extern fn location_free(data: *mut Location) {
    let _ = Box::from_raw(data);
}

#[no_mangle]
pub unsafe extern fn location_name(data: *const Location) -> *const c_char {
    let data = &*data;
    string_to_c_char(data.name.clone())
}

#[no_mangle]
pub unsafe extern fn location_latitude(data: *const Location) -> f64 {
    let data = &*data;
    data.latitude
}

#[no_mangle]
pub unsafe extern fn location_longitude(data: *const Location) -> f64 {
    let data = &*data;
    data.longitude
}

#[no_mangle]
pub unsafe extern fn location_altitude(data: *const Location) -> f64 {
    let data = &*data;
    data.altitude
}

#[no_mangle]
pub unsafe extern fn buoy_station_item_view_data_free(data: *mut BuoyStationItemViewData) {
    let _ = Box::from_raw(data);
}

#[no_mangle]
pub unsafe extern fn buoy_station_item_view_data_title(data: *const BuoyStationItemViewData) -> *const c_char {
    let data = &*data;
    string_to_c_char(data.title.clone())
}

#[no_mangle]
pub unsafe extern fn buoy_station_item_view_data_subtitle(data: *const BuoyStationItemViewData) -> *const c_char {
    let data = &*data;
    string_to_c_char(data.subtitle.clone())
}

#[no_mangle]
pub unsafe extern fn buoy_station_item_view_data_location(data: *const BuoyStationItemViewData) -> *mut Location {
    let data = &*data;
    let boxed_location = Box::new(data.location.clone());
    Box::into_raw(boxed_location)
}

#[no_mangle]
pub unsafe extern fn buoy_station_item_view_data_on_click_id(data: *const BuoyStationItemViewData) -> *const c_char {
    let data = &*data;
    string_to_c_char(data.on_click_id.clone())
}

#[no_mangle]
pub unsafe extern fn buoy_station_item_view_data_icon(data: *const BuoyStationItemViewData) -> BuoyStationIcon {
    let data = &*data;
    data.icon.clone()
}

#[no_mangle]
pub unsafe extern fn buoy_station_item_view_data_color(data: *const BuoyStationItemViewData) -> Color {
    let data = &*data;
    data.color.clone()
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
    buoy_station.is_active()
}

/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    extern crate android_logger;
    extern crate log_panics;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::JavaVM;
    use self::jni::objects::{JClass, JString, JValue, JObject, GlobalRef};
    use self::jni::sys::{jint, jlong, jdouble, jboolean, jstring};
    use self::android_logger::Filter;

    use log::Level;

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_MainActivity_initLogger(_: JNIEnv, _: JClass) {
        android_logger::init_once(
            Filter::default()
                .with_min_level(Level::Trace),
            Some("com.mpiannucci.buoyfinder")
        );

        log_panics::init();

        trace!("Initilized rust logger!")
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Store_new(_: JNIEnv, _: JClass) -> jlong {
        store_new() as jlong
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Store_free(_: JNIEnv, _: JClass, ptr: jlong) {
        let store = ptr as *mut Store<AppState, Actions>;
        store_free(store);
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Store_fetchBuoyStations(_: JNIEnv, _: JClass, ptr: jlong) {
        let store = ptr as *mut Store<AppState, Actions>;
        fetch_buoy_stations(store);
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStation_new(env: JNIEnv, _: JClass, station_id: JString, name: JString, lat: jdouble, lon: jdouble) -> jlong {
        let station_id = env.get_string(station_id).expect("Invalid station id string");
        let name = env.get_string(name).expect("Invalid station name string");
        
        buoy_station_new(station_id.as_ptr(), name.as_ptr(), lat, lon) as jlong
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStation_free(_: JNIEnv, _: JClass, ptr: jlong) {
        let buoy_station = ptr as *mut BuoyStation;
        buoy_station_free(buoy_station);
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStation_active(_: JNIEnv, _: JClass, ptr: jlong) -> jboolean {
        let buoy_station = ptr as *mut BuoyStation;
        buoy_station_active(buoy_station) as jboolean
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStation_stationId(env: JNIEnv, _: JClass, ptr: jlong) -> jstring {
        let buoy_station = ptr as *mut BuoyStation;
        let output = env.new_string((*buoy_station).station_id.as_str()).expect("Failed to create station id string");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStation_name(env: JNIEnv, _: JClass, ptr: jlong) -> jstring {
        let buoy_station = ptr as *mut BuoyStation;
        let output = env.new_string((*buoy_station).location.name.as_str()).expect("Failed to create station id string");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_ExploreViewData_new(_: JNIEnv, _: JClass) -> jlong {
        explore_view_data_new() as jlong
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_ExploreViewData_free(_: JNIEnv, _: JClass, ptr: jlong) {
        let view_data = ptr as *mut ExploreViewData;
        explore_view_data_free(view_data);
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_ExploreViewData_isLoading(_: JNIEnv, _: JClass, ptr: jlong) -> jboolean {
        let view_data = ptr as *mut ExploreViewData;
        explore_view_data_is_loading(view_data) as jboolean
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_ExploreViewData_stationCount(_: JNIEnv, _: JClass, ptr: jlong) -> jlong {
        let view_data = ptr as *mut ExploreViewData;
        explore_view_data_station_count(view_data) as jlong
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_ExploreViewData_stationAtIndex(_: JNIEnv, _: JClass, ptr: jlong, index: jlong) -> jlong {
        let view_data = ptr as *mut ExploreViewData;
        explore_view_data_station_index(view_data, index as i64) as jlong
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Location_new(env: JNIEnv, _: JClass, lat: jdouble, lon: jdouble, name: JString) -> jlong {
        let name = env.get_string(name).expect("Invalid location name string");
        location_new(lat, lon, name.as_ptr()) as jlong
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Location_free(_: JNIEnv, _: JClass, ptr: jlong) {
        let loc = ptr as *mut Location;
        location_free(loc);
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Location_name(env: JNIEnv, _: JClass, ptr: jlong) -> jstring {
        let location = ptr as *mut Location;
        let output = env.new_string((*location).name.as_str()).expect("Failed to create location name string");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Location_latitude(env: JNIEnv, _: JClass, ptr: jlong) -> jdouble {
        let location = ptr as *mut Location;
        let location = &*location;
        location.latitude as jdouble
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Location_longitude(env: JNIEnv, _: JClass, ptr: jlong) -> jdouble {
        let location = ptr as *mut Location;
        let location = &*location;
        location.longitude as jdouble
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Location_altitude(env: JNIEnv, _: JClass, ptr: jlong) -> jdouble {
        let location = ptr as *mut Location;
        let location = &*location;
        location.altitude as jdouble
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStationItemViewData_free(_: JNIEnv, _: JClass, ptr: jlong) {
        let data = ptr as *mut BuoyStationItemViewData;
        buoy_station_item_view_data_free(data);
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStationItemViewData_title(env: JNIEnv, _: JClass, ptr: jlong) -> jstring {
        let buoy_station = ptr as *mut BuoyStationItemViewData;
        let output = env.new_string((*buoy_station).title.as_str()).expect("Failed to create buoy title string");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStationItemViewData_subtitle(env: JNIEnv, _: JClass, ptr: jlong) -> jstring {
        let buoy_station = ptr as *mut BuoyStationItemViewData;
        let output = env.new_string((*buoy_station).subtitle.as_str()).expect("Failed to create buoy subtitle string");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStationItemViewData_location(_: JNIEnv, _: JClass, ptr: jlong) -> jlong {
        let buoy_station = ptr as *mut BuoyStationItemViewData;
        buoy_station_item_view_data_location(buoy_station) as jlong
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStationItemViewData_onClickId(env: JNIEnv, _: JClass, ptr: jlong) -> jstring {
        let buoy_station = ptr as *mut BuoyStationItemViewData;
        let output = env.new_string((*buoy_station).title.as_str()).expect("Failed to create buoy title string");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStationItemViewData_icon(_: JNIEnv, _: JClass, ptr: jlong) -> jint {
        let buoy_station = ptr as *mut BuoyStationItemViewData;
        let buoy_station = &*buoy_station;
        buoy_station.icon.clone() as jint
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_BuoyStationItemViewData_color(_: JNIEnv, _: JClass, ptr: jlong) -> jlong {
        let buoy_station = ptr as *mut BuoyStationItemViewData;
        let buoy_station = &*buoy_station;
        let boxed_color = Box::new(buoy_station.color.clone());
        Box::into_raw(boxed_color) as jlong
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Color_free(_: JNIEnv, _: JClass, ptr: jlong) {
        let color = ptr as *mut Color;
        let _ = Box::from_raw(color);
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Color_red(_: JNIEnv, _: JClass, ptr: jlong) -> jdouble {
        let color = ptr as *mut Color;
        let color = &*color;
        color.red as jdouble
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Color_green(_: JNIEnv, _: JClass, ptr: jlong) -> jdouble {
        let color = ptr as *mut Color;
        let color = &*color;
        color.green as jdouble
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_Color_blue(_: JNIEnv, _: JClass, ptr: jlong) -> jdouble {
        let color = ptr as *mut Color;
        let color = &*color;
        color.blue as jdouble
    }

    struct ExploreViewJVMWrapper {
        jvm: JavaVM,
        view: GlobalRef,
    }

    impl ExploreView for ExploreViewJVMWrapper {
        fn new_view_data(&mut self, view_data: &ExploreViewData) {
            let view_data = Box::into_raw(Box::new(view_data.clone()));

            // TODO: Need to attach the correct thread when not running on main 
            if let Ok(env) = self.jvm.get_env() {
                let j_view_data_class = env.find_class("com/mpiannucci/buoyfinder/core/ExploreViewData")
                    .expect("Failed to find ExploreViewData class");
                let j_view_data = env.new_object(j_view_data_class, "(J)V", &[JValue::Long(view_data as jlong).into()])
                    .expect("Failed to create a view data jvm object");

                let j_view = self.view.as_obj();
            
                env.call_method(j_view, "newViewData", "(Lcom/mpiannucci/buoyfinder/core/ExploreViewData;)V", &[JValue::Object(j_view_data).into()])
                    .expect("Failed to call newViewData on the JVM receiver");
            } else {
                let env = self.jvm.attach_current_thread()
                    .expect("Failed to attach to the current thread");
                let j_view_data_class = env.find_class("com/mpiannucci/buoyfinder/core/ExploreViewData")
                    .expect("Failed to find ExploreViewData class");
                let j_view_data = env.new_object(j_view_data_class, "(J)V", &[JValue::Long(view_data as jlong).into()])
                    .expect("Failed to create a view data jvm object");

                let j_view = self.view.as_obj();
            
                env.call_method(j_view, "newViewData", "(Lcom/mpiannucci/buoyfinder/core/ExploreViewData;)V", &[JValue::Object(j_view_data).into()])
                    .expect("Failed to call newViewData on the JVM receiver");
            }
        }
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_ExploreViewHandle_bind(env: JNIEnv, _: JClass, callback: JObject<'static>, store_ptr: jlong) -> jlong {
        let explore_view_wrapper = Box::new(ExploreViewJVMWrapper{
            jvm: env.get_java_vm().expect("Failed to get the JVM when registering explore view"),
            view: env.new_global_ref(callback).expect("Failed to get a global ref from explore view callback"),
        });
        let explore_view_model = Arc::new(Mutex::new(ExploreViewModel::new(explore_view_wrapper)));
        let explore_view_model_handle = Box::new(ExploreViewModelHandle(explore_view_model));
        let store = store_ptr as *mut Store<AppState, Actions>;
        let store = &mut*store;
        store.subscribe(explore_view_model_handle.0.clone());
        Box::into_raw(explore_view_model_handle) as jlong
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mpiannucci_buoyfinder_core_ExploreViewHandle_unbind(_: JNIEnv, _: JClass, handle_ptr: jlong, store_ptr: jlong) {
        let view_model_handle = handle_ptr as *mut ExploreViewModelHandle;
        let explore_view_model_handle = Box::from_raw(view_model_handle);
        let store = store_ptr as *mut Store<AppState, Actions>;
        let store = &mut*store;
        store.unsubscribe(explore_view_model_handle.0);
    }
}
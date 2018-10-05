use libc::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use jni::{JavaVM, JNIEnv};
use jni::objects::{JClass, JString, JValue, JObject, GlobalRef};
use jni::sys::{jint, jlong, jdouble, jboolean, jstring};

use super::sort_items::SortItems;

#[no_mangle]
pub fn c_char_to_string(cchar: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(cchar) };
    let r_str = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string, 
    };
    r_str.to_string()
}

#[no_mangle]
pub fn string_to_c_char(r_string: String) -> *mut c_char {
    CString::new(r_string).unwrap().into_raw()
}

#[derive(Clone, Debug)]
pub struct ItemList {
    pub items: Vec<String>,
}

#[no_mangle]
pub unsafe extern fn item_list_free(item_list: *mut ItemList) {
    let _ = Box::from_raw(item_list);
}

#[no_mangle]
pub unsafe extern fn item_list_items_count(item_list: *mut ItemList) -> i32 {
    let item_list = &mut*item_list;
    item_list.items.len() as i32
}

#[no_mangle]
pub unsafe extern fn item_list_items_index(item_list: *mut ItemList, index: i32) -> *const c_char {
    let item_list = &mut*item_list;
    string_to_c_char(item_list.items[index as usize].clone())
}

#[no_mangle]
pub unsafe extern fn Java_com_mpiannucci_hello_ItemList_free(_: JNIEnv, _: JClass, ptr: jlong) {
    let item_list = ptr as *mut ItemList;
    item_list_free(item_list);
}

#[no_mangle]
pub unsafe extern fn Java_com_mpiannucci_hello_ItemList_itemsCount(_: JNIEnv, _: JClass, ptr: jlong) -> jint {
    let item_list = ptr as *mut ItemList;
    item_list_items_count(item_list) as jint
}

#[no_mangle]
pub unsafe extern fn Java_com_mpiannucci_hello_ItemList_itemsIndex(env: JNIEnv, _: JClass, ptr: jlong, index: i32) -> jstring  {
    let item_list = ptr as *mut ItemList;
    let output = env.new_string((*item_list).items[index as usize].as_str()).expect("Failed to create itemsIndex string");
    output.into_inner()
}

pub trait ItemsListener {
    fn update(&mut self, items: &ItemList);
}

#[repr(C)]
pub struct ItemsListenerContainer {
    pub listener: *mut c_void,
    pub update: extern fn(listener: *mut c_void, items: *mut ItemList),
}

struct ItemsListenerWrapper {
    listener: ItemsListenerContainer,
}

impl ItemsListener for ItemsListenerWrapper {
    fn update(&mut self, items: &ItemList) {
        let item_list = Box::into_raw(Box::new(items.clone()));
        (self.listener.update)(self.listener.listener, item_list);
    }
}

struct ItemsListenerJVMWrapper {
    jvm: JavaVM,
    listener: GlobalRef,
}

impl ItemsListener for ItemsListenerJVMWrapper {
    fn update(&mut self, items: &ItemList) {
        let item_list = Box::into_raw(Box::new(items.clone()));
        if let Ok(env) = self.jvm.get_env() {
            let j_items_list_class = env.find_class("com/mpiannucci/hello/ItemsList")
                .expect("Failed to find ItemsList class");
            let j_items_list = env.new_object(j_items_list_class, "(J)V", &[JValue::Long(item_list as jlong).into()])
                .expect("Failed to create a ItemsList jvm object");
            let j_listener = self.listener.as_obj();
            env.call_method(j_listener, "update", "(Lcom/mpiannucci/hello/ItemList;)V", &[JValue::Object(j_items_list).into()])
                .expect("Failed to call update on the JVM receiver");
        } else {
            let env = self.jvm.attach_current_thread()
                .expect("Failed to attach to the current thread");
            let j_items_list_class = env.find_class("com/mpiannucci/hello/ItemsList")
                .expect("Failed to find ItemsList class");
            let j_items_list = env.new_object(j_items_list_class, "(J)V", &[JValue::Long(item_list as jlong).into()])
                .expect("Failed to create a ItemsList jvm object");
            let j_listener = self.listener.as_obj();
            env.call_method(j_listener, "update", "(Lcom/mpiannucci/hello/ItemList;)V", &[JValue::Object(j_items_list).into()])
                .expect("Failed to call update on the JVM receiver");
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum SortOrder {
    Ascending,
    Descending,
    Random,
}

impl From<i32> for SortOrder {
    fn from(value: i32) -> SortOrder {
        match value {
            0 => SortOrder::Ascending,
            1 => SortOrder::Descending,
            2 => SortOrder::Random,
            _ => SortOrder::Ascending,
        }
    }
}

pub trait SortItemsInterface {
    fn new(listener: Box<ItemsListener>) -> Self;
    fn sort(&mut self, order: &SortOrder, items: &ItemList);
    fn listener_count(&self) -> i32;
}

#[no_mangle]
pub unsafe extern fn sort_items_new(listener: ItemsListenerContainer) -> *mut SortItems {
    let listener = Box::new(ItemsListenerWrapper{
        listener: listener,
    });

    Box::into_raw(Box::new(SortItems::new(listener)))
}

#[no_mangle]
pub unsafe extern fn sort_items_free(sort_items: *mut SortItems) {
    let _ = Box::from_raw(sort_items);
}

#[no_mangle]
pub unsafe extern fn sort_items_sort(sort_items: *mut SortItems, order: SortOrder, items: *mut ItemList) {
    let sort_items = &mut*sort_items;
    let items = &mut*items;
    sort_items.sort(&order, &items);
}

#[no_mangle]
pub unsafe extern fn sort_items_listener_count(sort_items: *mut SortItems) -> i32 {
    let sort_items = &mut*sort_items;
    sort_items.listener_count()
}

#[no_mangle]
pub unsafe extern fn Java_com_mpiannucci_hello_SortItems_new(env: JNIEnv, _: JClass, listener: JObject<'static>) -> jlong {
    let listener = Box::new(ItemsListenerJVMWrapper{
        jvm: env.get_java_vm().expect("Failed to get the JVM"),
        listener: env.new_global_ref(listener).expect("Failed to get a global ref from listener"),
    });

    Box::into_raw(Box::new(SortItems::new(listener))) as jlong
}

#[no_mangle]
pub unsafe extern fn Java_com_mpiannucci_hello_SortItems_free(_: JNIEnv, _: JClass, ptr: jlong) {
    let sort_items = ptr as *mut SortItems;
    sort_items_free(sort_items);
}

#[no_mangle]
pub unsafe extern fn Java_com_mpiannucci_hello_SortItems_sort(_: JNIEnv, _: JClass, ptr: jlong, order: i32, items: jlong) {
    let sort_items = ptr as *mut SortItems;
    let order = SortOrder::from(order);
    let items = items as *mut ItemList;
    sort_items_sort(sort_items, order, items);
}

#[no_mangle]
pub unsafe extern fn Java_com_mpiannucci_hello_SortItems_listenerCount(_: JNIEnv, _: JClass, ptr: jlong) -> jint {
    let sort_items = ptr as *mut SortItems;
    sort_items_listener_count(sort_items) as jint
}
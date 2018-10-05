use libc::c_void;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use super::sort_items;

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

#[derive(Clone, Debug)]
#[repr(C)]
pub enum SortOrder {
    Ascending,
    Descending,
    Random,
}

pub trait SortItemsInterface {
    fn new(listener: Box<ItemsListener>) -> Self;
    fn sort(&mut self, order: &SortOrder, items: &ItemList);
    fn listener_count(&self) -> i32;
}

#[no_mangle]
pub unsafe extern fn sort_items_new(listener: ItemsListenerContainer) -> *mut sort_items::SortItems {
    let listener = Box::new(ItemsListenerWrapper{
        listener: listener,
    });

    Box::into_raw(Box::new(sort_items::SortItems::new(listener)))
}

#[no_mangle]
pub unsafe extern fn sort_items_sort(sort_items: *mut sort_items::SortItems, order: SortOrder, items: *mut ItemList) {
    let sort_items = &mut*sort_items;
    let items = &mut*items;
    sort_items.sort(&order, &items);
}

#[no_mangle]
pub unsafe extern fn sort_items_listener_count(sort_items: *mut sort_items::SortItems) -> i32 {
    let sort_items = &mut*sort_items;
    sort_items.listener_count()
}
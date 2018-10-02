#[macro_use]
extern crate ffi_derive;

#[ffi]
struct Location {
    lat: f64,
    lon: f64,
    alt: f64,
    name: String,
}

#[test]
fn test_func_write() {
    // println!("{}", ffi_Location_new())
}
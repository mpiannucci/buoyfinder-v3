#[macro_use]
extern crate ffi_derive;

#[ffi]
pub struct Location {
    lat: f64,
    lon: f64, 
    alt: f64,
    name: String,
}

#[test]
fn test_func_write() {
    unsafe {
        let location = ffi_Location_new(41.0, -71.0, 0., String::from("Northeast Coast"));
        println!("{}", (&*location).lat);
    }
}
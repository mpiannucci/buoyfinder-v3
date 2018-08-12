extern crate libc;

use libc::size_t;


#[repr(C)]
pub struct RustByteSlice {
    pub bytes: *const u8,
    pub len: size_t,
}

#[no_mangle]
pub extern fn get_string_from_rust() -> RustByteSlice {
    let s = "This is a string from Rust.";
    RustByteSlice{
        bytes: s.as_ptr(),
        len: s.len() as size_t,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

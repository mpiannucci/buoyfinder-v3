extern crate libc;

use libc::size_t;


#[repr(C)]
pub struct RustByteSlice {
    pub bytes: *const u8,
    pub len: size_t,
}


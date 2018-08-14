#![allow(dead_code)]

extern crate libc;
#[macro_use] 
extern crate serde_derive;
extern crate serde_xml_rs;

pub mod strings;
pub mod location;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

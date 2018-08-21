#![allow(dead_code)]

extern crate libc;
#[macro_use] 
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;
extern crate futures;
extern crate reqwest;
extern crate tokio;

pub mod location;
pub mod station;
pub mod redux;
pub mod app;
pub mod vm;
pub mod api;
pub mod ffi;

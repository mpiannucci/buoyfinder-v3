#![allow(dead_code)]

extern crate libc;
#[macro_use] 
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;
extern crate futures;
extern crate reqwest;
extern crate tokio;
#[macro_use]
extern crate log;

pub mod data;
pub mod app;
pub mod ffi;
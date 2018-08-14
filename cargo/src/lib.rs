#![allow(dead_code)]

extern crate libc;
#[macro_use] 
extern crate serde_derive;
extern crate serde_xml_rs;

pub mod strings;
pub mod location;
pub mod station;

#[cfg(test)]
mod tests {
    use serde_xml_rs::deserialize;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn it_works() {
        let stations_xml_path = Path::new("mock/activestations.xml");
        let stations_xml_file = File::open(stations_xml_path).expect("file not found");
        let buoy_stations: station::BuoyStations = deserialize(stations_xml_file).unwrap();
        println!("{:?}", buoy_stations);
    }
}

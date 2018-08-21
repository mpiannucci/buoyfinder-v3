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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use station;

    #[test]
    fn read_stations_xml() {
        let stations_xml_path = Path::new("mock/activestations.xml");
        let mut stations_xml_file = File::open(stations_xml_path).expect("file not found");
        let mut raw_station_data = String::new();
        stations_xml_file.read_to_string(&mut raw_station_data).unwrap();
        let buoy_stations: station::BuoyStations = station::BuoyStations::from_raw_data(raw_station_data.as_ref());
        println!("{:?}", buoy_stations);
        assert_eq!(buoy_stations.station_count, buoy_stations.stations.len() as i64 - 1)
    }
}

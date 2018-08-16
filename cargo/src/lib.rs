#![allow(dead_code)]

extern crate libc;
#[macro_use] 
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

pub mod strings;
pub mod location;
pub mod station;

#[cfg(test)]
mod tests {
    use serde_xml_rs::de::from_reader;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use station;
    use location::Location;

    #[test]
    fn parse_location_xml() {
        let location_xml = r##"<stations><location lat="30" lon="-90" name="OTN201 - 4800922"/></stations>"##;
        let location_value: station::BuoyStations = from_reader(location_xml.as_bytes()).unwrap();
        println!("{:?}", location_value);
    }

    #[test]
    fn read_stations_xml() {
        let stations_xml_path = Path::new("mock/activestations.xml");
        let mut stations_xml_file = File::open(stations_xml_path).expect("file not found");
        let mut raw_station_data = String::new();
        stations_xml_file.read_to_string(&mut raw_station_data);
        let buoy_stations: station::BuoyStations = from_reader(raw_station_data.as_bytes()).unwrap();
        println!("{:?}", buoy_stations);
    }
}

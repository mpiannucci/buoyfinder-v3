use location::Location;
use std::string::String;
use serde::de::{Deserialize, Deserializer};
use serde_xml_rs::de::from_reader;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuoyType {
    None,
    Buoy, 
    Fixed,
    OilRig,
    Dart,
    Tao,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "station")]
pub struct BuoyStation {
    #[serde(rename = "id")]
    pub station_id: String,

    pub owner: String, 

    #[serde(rename = "pgm")]
    pub program: String,

    #[serde(rename = "type")]
    pub buoy_type: BuoyType,

    #[serde(rename = "met", deserialize_with = "bool_from_simple_str", default)]
    pub active: bool,

    #[serde(default, deserialize_with = "bool_from_simple_str")]
    pub currents: bool,

    #[serde(rename = "waterquality", deserialize_with = "bool_from_simple_str", default)]
    pub water_quality: bool,

    #[serde(default, deserialize_with = "bool_from_simple_str")]
    pub dart: bool,

    #[serde(flatten)]
    pub location: Location,
}

impl BuoyStation {
    pub fn new(station_id: String, location: Location) -> BuoyStation {
        BuoyStation {
            station_id: station_id,
            location: location,
            owner: String::from(""),
            program: String::from(""),
            buoy_type: BuoyType::Buoy,
            active: true,
            currents: false,
            water_quality: false,
            dart: false,
        }
    }

    pub fn name(&self) -> String {
        let mut name = self.location.name.split("-").map(|s| {
            s.trim()
        }).filter(|s| {
            match s.parse::<i64>() {
                Ok(_) => false, 
                _ => true
            }
        }).collect::<Vec<&str>>().join("");

        name = name.split_whitespace().filter(|s| {
            if s.starts_with("(") {
                false
            } else {
                true
            }
        }).collect::<Vec<&str>>().join(" ");

        name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuoyStations {
    #[serde(rename = "$value")]
    pub stations: Vec<BuoyStation>,

    #[serde(rename = "count")]
    pub station_count: i64,
}

impl BuoyStations {
    pub fn active_stations_url() -> String {
        String::from("https://www.ndbc.noaa.gov/activestations.xml")
    }

    pub fn from_raw_data(raw_data: &str) -> BuoyStations {
        from_reader(raw_data.as_bytes()).unwrap()
    }

    pub fn find_station(&self, station_id: &str) -> Option<BuoyStation> {
        match self.stations.iter().position(|x| x.station_id == station_id) {
            Some(index) => Some(self.stations[index].clone()),
            _ => None,
        }
    }
}

impl Default for BuoyStations {
    fn default() -> BuoyStations {
        BuoyStations {
            stations: vec![],
            station_count: 0,
        }
    }
}

fn bool_from_simple_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    match s.as_ref() {
        "y" => Ok(true),
        _ => Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use super::*;

    fn read_stations_mock() -> String {
        let stations_xml_path = Path::new("mock/activestations.xml");
        let mut stations_xml_file = File::open(stations_xml_path).expect("file not found");
        let mut raw_station_data = String::new();
        stations_xml_file.read_to_string(&mut raw_station_data).unwrap();
        raw_station_data
    }

    #[test]
    fn read_stations_xml() {
        let raw_station_data = read_stations_mock();
        let buoy_stations: BuoyStations = BuoyStations::from_raw_data(raw_station_data.as_ref());
        //println!("{:?}", buoy_stations);
        assert_eq!(buoy_stations.station_count, buoy_stations.stations.len() as i64 - 1);

        let bi_station_res = buoy_stations.find_station("44097");
        assert_eq!(bi_station_res.is_some(), true);

        if let Some(bi_station) = bi_station_res {
            assert_eq!(bi_station.name().as_str(), "Block Island, RI")
        }
    }
}
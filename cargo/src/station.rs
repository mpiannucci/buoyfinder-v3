use location::Location;
use std::string::String;
use serde::de::{Deserialize, Deserializer};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuoyStations {
    #[serde(rename = "$value")]
    pub stations: Vec<BuoyStation>,

    #[serde(rename = "count")]
    pub station_count: i64,
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
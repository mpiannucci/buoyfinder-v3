use location::Location;
use std::string::String;

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

    #[serde(rename = "met", default)]
    pub active: String,

    #[serde(default)]
    pub currents: String,

    #[serde(rename = "waterquality", default)]
    pub water_quality: String,

    #[serde(default)]
    pub dart: String,

    // #[serde(flatten)]
    // pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuoyStations {
    #[serde(rename = "$value")]
    pub stations: Vec<BuoyStation>,
}
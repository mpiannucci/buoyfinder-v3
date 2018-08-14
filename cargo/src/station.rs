use location::Location;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum BuoyType {
    None,
    Buoy, 
    Fixed,
    OilRig,
    Dart,
    Tao,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BuoyStation {
    #[serde(rename = "id")]
    station_id: String,

    #[serde(flatten)]
    location: Location,

    owner: String, 

    #[serde(rename = "pgm")]
    program: String,

    type: BuoyType,

    #[serde(rename = "met")]
    active: bool,

    currents: bool,

    #[serde(rename = "waterquality")]
    water_quality: bool,

    dart: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BuoyStations {
    #[serde(rename = "$value")]
    stations: Vec<BuoyStation>,
}
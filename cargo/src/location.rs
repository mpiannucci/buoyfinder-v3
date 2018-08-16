use std::string::String;
use std::f64;
use serde::de::{self, Deserialize, Deserializer};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Location {
    #[serde(default)]
    pub name: String,

    #[serde(rename = "lat", deserialize_with = "f64_from_str")]
    pub latitude: f64,

    #[serde(rename = "lon", deserialize_with = "f64_from_str")]
    pub longitude: f64,

    #[serde(rename = "elev", deserialize_with = "f64_from_str", default)]
    pub altitude: f64,
}

impl Location {
    pub fn new(lat: f64, lon: f64, name: String) -> Location {
        Location {
            name: name,
            latitude: lat,
            longitude: lon,
            altitude: 0.0
        }
    }

    pub fn relative_latitude(&self) -> f64 {
        if self.latitude > 90.0 {
            self.latitude - 180.0
        } else {
            self.latitude
        }

    
    }

    pub fn relative_longitude(&self) -> f64 {
        if self.longitude > 180.0 {
            self.longitude - 360.0
        } else {
            self.longitude
        }
    }

    pub fn absolute_latitude(&self) -> f64 {
        if self.latitude < 0.0 {
            self.latitude + 180.0
        } else {
            self.latitude
        }
    }

    pub fn absolute_longitude(&self) -> f64 {
        if self.longitude < 0.0 {
            self.longitude + 360.0
        } else {
            self.longitude
        }
    }
}

fn f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(de::Error::custom)
}


// struct F64Visitor;

// impl<'de> Visitor<'de> for F64Visitor {

//     type Value = f64;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("a float value wrapped in a string")
//     }

//     fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where E: de::Error, {
//         println!("{}", s);
//         Result::Ok(0.0)
//     }
// }
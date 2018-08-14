use std::string::String;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Location {
    #[serde(default)]
    name: String,

    #[serde(rename = "lat")]
    latitude: f64,

    #[serde(rename = "lon")]
    longitude: f64,

    #[serde(rename = "elev", default)]
    altitude: f64,
}

impl Location {
    fn new(lat: f64, lon: f64, name: String) -> Location {
        Location {
            name: name,
            latitude: lat,
            longitude: lon,
            altitude: 0.0
        }
    }

    fn relative_latitude(&self) -> f64 {
        if self.latitude > 90.0 {
            self.latitude - 180.0
        } else {
            self.latitude
        }
    }

    fn relative_longitude(&self) -> f64 {
        if self.longitude > 180.0 {
            self.longitude - 360.0
        } else {
            self.longitude
        }
    }

    fn absolute_latitude(&self) -> f64 {
        if self.latitude < 0.0 {
            self.latitude + 180.0
        } else {
            self.latitude
        }
    }

    fn absolute_longitude(&self) -> f64 {
        if self.longitude < 0.0 {
            self.longitude + 360.0
        } else {
            self.longitude
        }
    }
}

pub enum Measurement {
    Length, 
    Speed,
    Temperature,
    Pressure,
    Visibility,
    Direction,
}

impl Measurement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Measurement::Length => "length",
            Measurement::Speed => "speed",
            Measurement::Temperature => "temperature",
            Measurement::Pressure => "pressure",
            Measurement::Visibility => "visibility",
            Measurement::Direction => "direction"
        }
    }
}

pub enum Units {
    Metric,
    English,
    Knots,
    Kelvin,
}

impl Units {
    pub fn as_str(&self) -> &'static str {
        match self {
            Units::Metric => "metric",
            Units::English => "english",
            Units::Knots => "knots",
            Units::Kelvin => "kelvin"
        }
    }

    pub fn label(&self, measurement: &Measurement, abbrev: bool) -> &'static str {
        let unit_tuple = (self, measurement, abbrev);
        match unit_tuple {
            (Units::Metric, Measurement::Length, true) => "m",
            (Units::Metric, Measurement::Length, false) => "meters",
            (Units::Metric, Measurement::Speed, true) => "m/s",
            (Units::Metric, Measurement::Speed, false) => "meters per second",
            (Units::Metric, Measurement::Temperature, true) => "°C",
            (Units::Metric, Measurement::Temperature, false) => "° celsius",
            (Units::Metric, Measurement::Pressure, true) => "hPa",
            (Units::Metric, Measurement::Pressure, false) => "hecta pascals",

            (Units::English, Measurement::Length, true) => "ft",
            (Units::English, Measurement::Length, false) => "feet",
            (Units::English, Measurement::Speed, true) => "mph",
            (Units::English, Measurement::Speed, false) => "miles per hour",
            (Units::English, Measurement::Temperature, true) => "°F",
            (Units::English, Measurement::Temperature, false) => "° fahrenheit",
            (Units::English, Measurement::Pressure, true) => "in HG",
            (Units::English, Measurement::Pressure, false) => "inches mercury",

            (Units::Knots, Measurement::Speed, true) => "knt",
            (Units::Knots, Measurement::Speed, false) => "knots",

            (Units::Kelvin, Measurement::Temperature, true) => "K",
            (Units::Kelvin, Measurement::Temperature, false) => "kelvin",

            (_, Measurement::Visibility, true) => "nmi",
            (_, Measurement::Visibility, false) => "nautical miles",
            (_, Measurement::Direction, _) => "°",
            _ => ""
        }
    }

    pub fn convert(&self, measurement: &Measurement, destination: &Units, value: f64) -> f64 {
        let unit_tuple = (self, measurement, destination);
        match unit_tuple {
            (Units::Metric, Measurement::Length, Units::English) => value * 3.28,
            (Units::Metric, Measurement::Speed, Units::English) => value * 2.237,
            (Units::Metric, Measurement::Speed, Units::Knots) => value * 1.944,
            (Units::Metric, Measurement::Temperature, Units::English) => value * (9.0 / 5.0) + 32.0,
            (Units::Metric, Measurement::Temperature, Units::Kelvin) => value + 273.0,
            (Units::Metric, Measurement::Pressure, Units::English) => value / 33.8638,

            (Units::English, Measurement::Length, Units::Metric) => value / 3.28,
            (Units::English, Measurement::Speed, Units::Metric) => value / 2.237,
            (Units::English, Measurement::Speed, Units::Knots) => value / 1.15,
            (Units::English, Measurement::Temperature, Units::Metric) => value - 32.0 * (5.0 / 9.0),
            (Units::English, Measurement::Temperature, Units::Kelvin) => (value + 459.67) * (5.0 / 9.0),
            (Units::English, Measurement::Pressure, Units::Metric) => value * 33.8638,

            (Units::Knots, Measurement::Speed, Units::Metric) => value * 0.514,
            (Units::Knots, Measurement::Speed, Units::English) => value * 1.15,

            (Units::Kelvin, Measurement::Temperature, Units::Metric) => value - 273.0,
            (Units::Kelvin, Measurement::Temperature, Units::English) => value * (9.0 / 5.0) - 459.67,

            _ => value
        }
    }

    pub fn earths_radius(&self) -> f64 {
        match self {
            Units::Metric => 6371.0,
            Units::English => 3956.0,
            _ => 0.0,
        }
    }
}

pub enum Direction {
    North,
    NorthNorthEast,
    NorthEast,
    EastNorthEast,
    East,
    EastSouthEast,
    SouthEast,
    SouthSouthEast,
    South,
    SouthSouthWest,
    SouthWest,
    WestSouthWest,
    West,
    WestNorthWest,
    NorthWest,
    NorthNorthWest,
}

impl Direction {

    pub fn from_degree(degree: f64) -> Direction {
        Direction::North
    }

    pub fn label(&self, abbrev: bool) -> &'static str {
         let dir_tuple = (self, abbrev);
         match dir_tuple {
             (Direction::North, true) => "n",
             (Direction::North, false) => "north",
             (Direction::NorthNorthEast, true) => "nne",
             (Direction::NorthNorthEast, false) => "north-northeast",
             (Direction::NorthEast, true) => "ne",
             (Direction::NorthEast, false) => "northeast",
             (Direction::EastNorthEast, true) => "ene",
             (Direction::EastNorthEast, false) => "east-northeast",
             (Direction::East, true) => "e",
             (Direction::East, false) => "east",
             (Direction::EastSouthEast, true) => "ese",
             (Direction::EastSouthEast, false) => "east-southeast",
             (Direction::SouthEast, true) => "se",
             (Direction::SouthEast, false) => "southeast",
             (Direction::SouthSouthEast, true) => "sse",
             (Direction::SouthSouthEast, false) => "south-southeast",
             (Direction::South, true) => "s",
             (Direction::South, false) => "south",
             (Direction::SouthSouthWest, true) => "ssw",
             (Direction::SouthSouthWest, false) => "south-southwest",
             (Direction::SouthWest, true) => "sw",
             (Direction::SouthWest, false) => "southwest",
             (Direction::WestSouthWest, true) => "wsw",
             (Direction::WestSouthWest, false) => "west-southwest",
             (Direction::West, true) => "w",
             (Direction::West, false) => "west",
             (Direction::WestNorthWest, true) => "wnw",
             (Direction::WestNorthWest, false) => "west-northwest",
             (Direction::NorthWest, true) => "nw",
             (Direction::NorthWest, false) => "northwest",
             (Direction::NorthNorthWest, true) => "nnw",
             (Direction::NorthNorthWest, false) => "north-northwest",
         }
    }
}
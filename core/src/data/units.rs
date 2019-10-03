use std::str::FromStr;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Measurement {
    Length, 
    Speed,
    Temperature,
    Pressure,
    Visibility,
    Direction,
    Time,
}

impl Measurement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Measurement::Length => "length",
            Measurement::Speed => "speed",
            Measurement::Temperature => "temperature",
            Measurement::Pressure => "pressure",
            Measurement::Visibility => "visibility",
            Measurement::Direction => "direction",
            Measurement::Time => "time"
        }
    }
}

#[derive(Clone, Debug)]
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
            (_, Measurement::Time, true) => "s",
            (_, Measurement::Time, false) => "seconds",
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

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Debug)]
pub enum CardinalDirection {
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
    Invalid,
}

impl CardinalDirection {
    pub fn from_degree(degree: i64) -> CardinalDirection {
        match degree {
            349 ..= 360 | 0 ..= 11 => CardinalDirection::North,
            12 ..= 33 => CardinalDirection::NorthNorthEast,
            34 ..= 56 => CardinalDirection::NorthEast,
            57 ..= 78 => CardinalDirection::EastNorthEast,
            79 ..= 101 => CardinalDirection::East,
            102 ..= 123 => CardinalDirection::EastSouthEast,
            124 ..= 146 => CardinalDirection::SouthEast,
            147 ..= 168 => CardinalDirection::SouthSouthEast,
            169 ..= 191 => CardinalDirection::South,
            192 ..= 213 => CardinalDirection::SouthSouthWest,
            214 ..= 236 => CardinalDirection::SouthWest,
            237 ..= 258 => CardinalDirection::WestSouthWest,
            259 ..= 281 => CardinalDirection::West,
            282 ..= 303 => CardinalDirection::WestNorthWest,
            304 ..= 326 => CardinalDirection::NorthWest,
            327 ..= 348 => CardinalDirection::NorthNorthWest,
            _ => CardinalDirection::Invalid,
        }
    }
}

impl fmt::Display for CardinalDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            CardinalDirection::North => "n",
            CardinalDirection::NorthNorthEast => "nne",
            CardinalDirection::NorthEast => "ne",
            CardinalDirection::EastNorthEast => "ene",
            CardinalDirection::East => "e",
            CardinalDirection::EastSouthEast => "ese",
            CardinalDirection::SouthEast => "se",
            CardinalDirection::SouthSouthEast => "sse",
            CardinalDirection::South => "s",
            CardinalDirection::SouthSouthWest => "ssw",
            CardinalDirection::SouthWest => "sw",
            CardinalDirection::WestSouthWest => "wsw",
            CardinalDirection::West => "w",
            CardinalDirection::WestNorthWest => "wnw",
            CardinalDirection::NorthWest => "nw",
            CardinalDirection::NorthNorthWest => "nnw",
            CardinalDirection::Invalid => ""
        })
    }
}

#[derive(Clone, Debug)]
pub struct Direction {
    pub direction: CardinalDirection,
    pub degree: Option<i64>,
}

impl Direction {
    pub fn from_degree(degree: i64) -> Direction {
        Direction {
            direction: CardinalDirection::from_degree(degree),
            degree: Some(degree),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.degree {
            Some(degree) => write!(f, "{}{} {}", degree, Units::Metric.label(&Measurement::Direction, true), self.direction),
            None => write!(f, "{}", self.direction)
        }
    }
}

impl From<Direction> for Measurement {
    fn from(_: Direction) -> Measurement {
        Measurement::Direction
    }
}

#[derive(Clone, Debug)]
pub enum Steepness {
    VerySteep,
    Steep,
    Average,
    Swell,
}

impl Steepness {
    pub fn as_str(&self) -> &'static str {
        match self {
            Steepness::VerySteep => "VERY_STEEP",
            Steepness::Steep => "STEEP",
            Steepness::Average => "AVERAGE",
            Steepness::Swell => "SWELL"
        }
    }
}

impl FromStr for Steepness {
    type Err = SteepnessParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VERY_STEEP" => Ok(Steepness::VerySteep),
            "STEEP" => Ok(Steepness::Steep),
            "Average" => Ok(Steepness::Average),
            "SWELL" => Ok(Steepness::Swell),
            _ => Err(SteepnessParseError::InvalidString)
        }
    }
}

impl fmt::Display for Steepness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub enum SteepnessParseError {
    InvalidString,
}

pub trait UnitConvertible<T> {
    fn to_units(&mut self, new_units: &Units);
}
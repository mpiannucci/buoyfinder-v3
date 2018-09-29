#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Direction {
    North(i64),
    NorthNorthEast(i64),
    NorthEast(i64),
    EastNorthEast(i64),
    East(i64),
    EastSouthEast(i64),
    SouthEast(i64),
    SouthSouthEast(i64),
    South(i64),
    SouthSouthWest(i64),
    SouthWest(i64),
    WestSouthWest(i64),
    West(i64),
    WestNorthWest(i64),
    NorthWest(i64),
    NorthNorthWest(i64),
    Invalid(i64),
}

impl Direction {

    pub fn from_degree(degree: i64) -> Direction {
        match degree {
            349 ... 360 | 0 ... 11 => Direction::North(degree),
            12 ... 33 => Direction::NorthNorthEast(degree),
            34 ... 56 => Direction::NorthEast(degree),
            57 ... 78 => Direction::EastNorthEast(degree),
            79 ... 101 => Direction::East(degree),
            102 ... 123 => Direction::EastSouthEast(degree),
            124 ... 146 => Direction::SouthEast(degree),
            147 ... 168 => Direction::SouthSouthEast(degree),
            169 ... 191 => Direction::South(degree),
            192 ... 213 => Direction::SouthSouthWest(degree),
            214 ... 236 => Direction::SouthWest(degree),
            237 ... 258 => Direction::WestSouthWest(degree),
            259 ... 281 => Direction::West(degree),
            282 ... 303 => Direction::WestNorthWest(degree),
            304 ... 326 => Direction::NorthWest(degree),
            327 ... 348 => Direction::NorthNorthWest(degree),
            _ => Direction::Invalid(degree),
        }
    }

    pub fn label(&self, abbrev: bool) -> &'static str {
         let dir_tuple = (self, abbrev);
         match dir_tuple {
             (Direction::North(_), true) => "n",
             (Direction::North(_), false) => "north",
             (Direction::NorthNorthEast(_), true) => "nne",
             (Direction::NorthNorthEast(_), false) => "north-northeast",
             (Direction::NorthEast(_), true) => "ne",
             (Direction::NorthEast(_), false) => "northeast",
             (Direction::EastNorthEast(_), true) => "ene",
             (Direction::EastNorthEast(_), false) => "east-northeast",
             (Direction::East(_), true) => "e",
             (Direction::East(_), false) => "east",
             (Direction::EastSouthEast(_), true) => "ese",
             (Direction::EastSouthEast(_), false) => "east-southeast",
             (Direction::SouthEast(_), true) => "se",
             (Direction::SouthEast(_), false) => "southeast",
             (Direction::SouthSouthEast(_), true) => "sse",
             (Direction::SouthSouthEast(_), false) => "south-southeast",
             (Direction::South(_), true) => "s",
             (Direction::South(_), false) => "south",
             (Direction::SouthSouthWest(_), true) => "ssw",
             (Direction::SouthSouthWest(_), false) => "south-southwest",
             (Direction::SouthWest(_), true) => "sw",
             (Direction::SouthWest(_), false) => "southwest",
             (Direction::WestSouthWest(_), true) => "wsw",
             (Direction::WestSouthWest(_), false) => "west-southwest",
             (Direction::West(_), true) => "w",
             (Direction::West(_), false) => "west",
             (Direction::WestNorthWest(_), true) => "wnw",
             (Direction::WestNorthWest(_), false) => "west-northwest",
             (Direction::NorthWest(_), true) => "nw",
             (Direction::NorthWest(_), false) => "northwest",
             (Direction::NorthNorthWest(_), true) => "nnw",
             (Direction::NorthNorthWest(_), false) => "north-northwest",
             _ => ""
         }
    }
}

impl From<Direction> for Measurement {
    fn from(_: Direction) -> Measurement {
        Measurement::Direction
    }
}

pub trait UnitConvertible<T> {
    fn to_units(&self, new_units: &Units) -> T;
}
use num_traits::Float;

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

    pub fn unit(&self, measurement: &Measurement, abbrev: bool) -> &'static str {
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

    pub fn convert<T>(&self, measurement: &Measurement, destination: &Units, value: T) -> T where T:Float, f64: Into<T> {
        let unit_tuple = (self, measurement, destination);
        match unit_tuple {
            (Units::Metric, Measurement::Length, Units::English) => value * 3.28.into(),
            (Units::Metric, Measurement::Speed, Units::English) => value * 2.237.into(),
            (Units::Metric, Measurement::Speed, Units::Knots) => value * 1.944.into(),
            (Units::Metric, Measurement::Temperature, Units::English) => (value * (9.0 / 5.0).into()) + 32.0.into(),
            (Units::Metric, Measurement::Temperature, Units::Kelvin) => value + 273.0.into(),
            (Units::Metric, Measurement::Pressure, Units::English) => value / 33.8638.into(),

            (Units::English, Measurement::Length, Units::Metric) => value / 3.28.into(),
            (Units::English, Measurement::Speed, Units::Metric) => value / 2.237.into(),
            (Units::English, Measurement::Speed, Units::Knots) => value / 1.15.into(),
            (Units::English, Measurement::Temperature, Units::Metric) => (value - 32.0.into()) * (5.0 / 9.0).into(),
            (Units::English, Measurement::Temperature, Units::Kelvin) => (value + 459.67.into()) * (5.0 / 9.0).into(),
            (Units::English, Measurement::Pressure, Units::Metric) => value * 33.8638.into(),

            (Units::Knots, Measurement::Speed, Units::Metric) => value * 0.514.into(),
            (Units::Knots, Measurement::Speed, Units::English) => value * 1.15.into(),

            (Units::Kelvin, Measurement::Temperature, Units::Metric) => value - 273.0.into(),
            (Units::Kelvin, Measurement::Temperature, Units::English) => value * (9.0 / 5.0).into() - 459.67.into(),

            _ => value
        }
    }
}
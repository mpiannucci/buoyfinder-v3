use data::dimensional_data::DimensionalData;
use data::units::{Units, Measurement, Direction, UnitConvertible};
use std::fmt;

#[derive(Clone)]
pub struct Swell {
    wave_height: DimensionalData<f64>,
    period: DimensionalData<f64>,
    direction: DimensionalData<Direction>,
}

impl Swell {
    pub fn new(units: &Units, wave_height: f64, period: f64, direction: Direction) -> Swell {
        Swell {
            wave_height: DimensionalData<f64> {
                value: Some(wave_height),
                measurement: Measurement::Length,
                units: units.clone(),
            },
            period: DimensionalData<f64> {
                value: Some(period),
                measurement: Measurement::Time,
                units: units.clone(),
            },
            direction: DimensionalData<Direction> {
                value: Some(direction),
                measurement: Measurement::Direction,
                units: units.clone(),
            }
        }
    }
}

impl UnitConvertible<Swell> for Swell {
    fn to_units(&self, new_units: &Units) -> Swell {
        Swell {
            wave_height: self.wave_height.to_units(new_units),
            period: self.wave_height.to_units(new_units),
            direction: self.wave_height.to_units(new_units),
        }
    }
}

impl fmt::Display for Swell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @ {} {}", self.wave_height, self.period, self.direction)
    }
}
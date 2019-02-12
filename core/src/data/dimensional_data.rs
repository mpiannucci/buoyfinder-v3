use crate::data::units::{Units, Measurement, Direction, UnitConvertible};
use std::option::Option;
use std::fmt;

#[derive(Clone, Debug)]
pub struct DimensionalData<T> {
    pub value: Option<T>,
    pub variable_name: &'static str,
    pub measurement: Measurement,
    pub unit: Units,
}

impl UnitConvertible<DimensionalData<f64>> for DimensionalData<f64> {
    fn to_units(&mut self, new_units: &Units) {
        self.value = match self.value {
            Some(val) => Some(self.unit.convert(&self.measurement, new_units, val)),
            None => None,
        };
    }
}

impl UnitConvertible<DimensionalData<i64>> for DimensionalData<i64> {
    fn to_units(&mut self, new_units: &Units) {
        self.value = match self.value {
               Some(val) => Some(self.unit.convert(&self.measurement, new_units, val as f64) as i64),
               None => None,
        };
    }
}

impl UnitConvertible<DimensionalData<Direction>> for DimensionalData<Direction> {
    fn to_units(&mut self, new_units: &Units) {
        // Do nothing
    }
}

impl <T> fmt::Display for DimensionalData<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(ref val) => write!(f, "{} {}", val, self.unit.label(&self.measurement, true)),
            None => write!(f, "N/A"),
        }
        
    }
}


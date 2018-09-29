use data::units::{Units, Measurement, Direction, UnitConvertible};

#[derive(Clone, Debug)]
pub struct DimensionalData<T> {
    pub value: T,
    pub measurement: Measurement,
    pub unit: Units,
}

impl UnitConvertible<DimensionalData<f64>> for DimensionalData<f64> {
    fn to_units(&self, new_units: &Units) -> Self {
        DimensionalData {
            value: self.unit.convert(&self.measurement, new_units, self.value),
            measurement: self.measurement.clone(),
            unit: new_units.clone(),
        }
    }
}

impl UnitConvertible<DimensionalData<i64>> for DimensionalData<i64> {
    fn to_units(&self, new_units: &Units) -> Self {
        DimensionalData {
            value: self.unit.convert(&self.measurement, new_units, self.value as f64) as i64,
            measurement: self.measurement.clone(),
            unit: new_units.clone(),
        }
    }
}

impl UnitConvertible<DimensionalData<Direction>> for DimensionalData<Direction> {
    fn to_units(&self, new_units: &Units) -> Self {
        DimensionalData {
            value: self.value.clone(),
            measurement: self.measurement.clone(),
            unit: new_units.clone(),
        }
    }
}



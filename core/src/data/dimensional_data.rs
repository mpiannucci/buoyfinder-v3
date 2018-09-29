use data::units::{Units, Measurement};

pub struct DimensionalData {
    pub value: f64,
    pub measurement: Measurement,
    pub unit: Units,
}

impl DimensionalData {
    fn change_units(&mut self, new_units: &Units) {
        self.value = self.unit.convert(&self.measurement, new_units, self.value);
        self.unit = new_units.clone();
    }
}



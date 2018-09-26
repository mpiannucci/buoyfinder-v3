use data::units::Units;

trait DimensionalData {
    pub fn change_units(&mut self, new_units: &Units);
    pub fn units(&self) -> &Units;
}




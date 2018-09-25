use data::units::Units;

trait BuoyData {
    pub fn change_units(&mut self, new_units: &Units);
}
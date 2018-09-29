pub mod location;
pub mod station;
pub mod units;
pub mod dimensional_data;
pub mod buoy_data;

use data::units::Units;

pub trait DataRecord {
    fn change_units(&mut self, new_units: &Units);
}
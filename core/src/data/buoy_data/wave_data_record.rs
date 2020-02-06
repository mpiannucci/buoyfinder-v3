use crate::data::dimensional_data::DimensionalData;
use crate::data::units::*;
use super::date_record::DateRecord;
use super::parseable_data_record::ParseableDataRecord;

use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct WaveDataRecord {
    pub date: DateRecord,
    pub wave_height: DimensionalData<f64>,
    pub swell_wave_height: DimensionalData<f64>,
    pub swell_wave_period: DimensionalData<f64>,
    pub wind_wave_height: DimensionalData<f64>,
    pub wind_wave_period: DimensionalData<f64>,
    pub swell_wave_direction: DimensionalData<Direction>,
    pub wind_wave_direction: DimensionalData<Direction>,
    pub steepness: Steepness,
    pub average_wave_period: DimensionalData<f64>,
    pub mean_wave_direction: DimensionalData<Direction>,
}

impl ParseableDataRecord for WaveDataRecord {
    fn from_data_row(row: &Vec<&str>) -> WaveDataRecord {
        WaveDataRecord {
            date: DateRecord::from_data_row(row),
            wave_height: DimensionalData::from_raw_data(row[5], "wave height", Measurement::Length, Units::Metric),
            swell_wave_height: DimensionalData::from_raw_data(row[6], "swell wave height", Measurement::Length, Units::Metric),
            swell_wave_period: DimensionalData::from_raw_data(row[7], "swell period", Measurement::Time, Units::Metric),
            wind_wave_height: DimensionalData::from_raw_data(row[8], "wind wave height", Measurement::Length, Units::Metric),
            wind_wave_period: DimensionalData::from_raw_data(row[9], "wind period", Measurement::Time, Units::Metric),
            swell_wave_direction: DimensionalData::from_raw_data(row[10], "swell wave direction", Measurement::Direction, Units::Metric),
            wind_wave_direction: DimensionalData::from_raw_data(row[11], "wind wave direction", Measurement::Direction, Units::Metric),
            steepness: Steepness::from_str(row[12]).unwrap_or(Steepness::NA),
            average_wave_period: DimensionalData::from_raw_data(row[10], "average wave period", Measurement::Time, Units::Metric),
            mean_wave_direction: DimensionalData::from_raw_data(row[11], "mean wave direction", Measurement::Direction, Units::Metric),
        }
    }
}

impl UnitConvertible<WaveDataRecord> for WaveDataRecord {
    fn to_units(&mut self, new_units: &Units) {
        self.wave_height.to_units(new_units);
        self.average_wave_period.to_units(new_units);
        self.mean_wave_direction.to_units(new_units);
        self.swell_wave_height.to_units(new_units);
        self.swell_wave_period.to_units(new_units);
        self.swell_wave_direction.to_units(new_units);
        self.wind_wave_height.to_units(new_units);
        self.wind_wave_period.to_units(new_units);
        self.wind_wave_direction.to_units(new_units);
    }
}
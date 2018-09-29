use data::units::*;
use chrono::prelude::*;
use data::dimensional_data::DimensionalData;
use data::DataRecord;

pub struct MeteorologicalDataRecord {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub wind_direction: Option<Direction>,
    pub wind_speed: Option<DimensionalData>,
    pub wind_gust_speed: Option<DimensionalData>,
    pub wave_height: Option<DimensionalData>,
    pub dominant_wave_period: Option<DimensionalData>,
    pub average_wave_period: Option<DimensionalData>,
    pub mean_wave_direction: Option<Direction>,
    pub air_pressure: Option<DimensionalData>,
    pub air_pressure_tendency: Option<DimensionalData>,
    pub air_temperature: Option<DimensionalData>,
    pub water_temperature: Option<DimensionalData>, 
    pub dewpoint_temperature: Option<DimensionalData>,
    pub visibility: Option<DimensionalData>,
    pub tide: Option<DimensionalData>,
}

impl DataRecord for MeteorologicalDataRecord {
    fn change_units(&mut self, new_units: &Units) {

    }
}

pub struct DetailedWaveDataRecord {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub wave_height: Option<DimensionalData>,
    pub average_wave_period: Option<DimensionalData>,
    pub mean_wave_direction: Option<Direction>,
    pub swell_wave_height: Option<DimensionalData>,
    pub swell_period: Option<DimensionalData>, 
    pub swell_direction: Option<Direction>,
    pub wind_wave_height: Option<DimensionalData>,
    pub wind_wave_period: Option<DimensionalData>, 
    pub wind_wave_direction: Option<Direction>,
    pub steepness: Option<String>,
}

impl DataRecord for DetailedWaveDataRecord {
    fn change_units(&mut self, new_units: &Units) {
        self.wave_height.change_units(new_units);
    }
}

pub struct SprectralWaveDataRecord {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub separation_frequency: f64,
    pub value: Vec<f64>,
    pub frequency: Vec<f64>,
}

impl DataRecord for SprectralWaveDataRecord {
    fn change_units(&mut self, _: &Units) {
        // Nothing to change for now, use as a marker protocol
    }
}
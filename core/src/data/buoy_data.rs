use data::units::*;
use chrono::prelude::*;
use data::dimensional_data::DimensionalData;

pub struct MeteorologicalDataRecord {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub wind_direction: Option<DimensionalData<Direction>>,
    pub wind_speed: Option<DimensionalData<f64>>,
    pub wind_gust_speed: Option<DimensionalData<f64>>,
    pub wave_height: Option<DimensionalData<f64>>,
    pub dominant_wave_period: Option<DimensionalData<f64>>,
    pub average_wave_period: Option<DimensionalData<f64>>,
    pub mean_wave_direction: Option<DimensionalData<Direction>>,
    pub air_pressure: Option<DimensionalData<f64>>,
    pub air_pressure_tendency: Option<DimensionalData<f64>>,
    pub air_temperature: Option<DimensionalData<f64>>,
    pub water_temperature: Option<DimensionalData<f64>>, 
    pub dewpoint_temperature: Option<DimensionalData<f64>>,
    pub visibility: Option<DimensionalData<f64>>,
    pub tide: Option<DimensionalData<f64>>,
}

pub struct DetailedWaveDataRecord {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub wave_height: Option<DimensionalData<f64>>,
    pub average_wave_period: Option<DimensionalData<f64>>,
    pub mean_wave_direction: Option<DimensionalData<Direction>>,
    pub swell_wave_height: Option<DimensionalData<f64>>,
    pub swell_period: Option<DimensionalData<f64>>, 
    pub swell_direction: Option<DimensionalData<Direction>>,
    pub wind_wave_height: Option<DimensionalData<f64>>,
    pub wind_wave_period: Option<DimensionalData<f64>>, 
    pub wind_wave_direction: Option<DimensionalData<Direction>>,
    pub steepness: Option<Steepness>,
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

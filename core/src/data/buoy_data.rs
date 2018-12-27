use crate::data::units::*;
use chrono::prelude::*;
use crate::data::dimensional_data::DimensionalData;

#[derive(Clone, Debug)]
pub enum BuoyDataRecord {
    Latest(MeteorologicalDataRecord, WaveDataRecord),
    Meteorological(MeteorologicalDataRecord),
    Wave(WaveDataRecord),
    SprectralWave(SprectralWaveDataRecord),
}

impl UnitConvertible<BuoyDataRecord> for BuoyDataRecord {
    fn to_units(&self, new_units: &Units) -> BuoyDataRecord {
        match self {
            BuoyDataRecord::Latest(met_data, wave_data) => BuoyDataRecord::Latest(met_data.to_units(new_units), wave_data.to_units(new_units)),
            BuoyDataRecord::Meteorological(data) => BuoyDataRecord::Meteorological(data.to_units(new_units)),
            BuoyDataRecord::Wave(data) => BuoyDataRecord::Wave(data.to_units(new_units)),
            BuoyDataRecord::SprectralWave(data) => BuoyDataRecord::SprectralWave(data.to_units(new_units)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MeteorologicalDataRecord {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub wind_direction: DimensionalData<Direction>,
    pub wind_speed: DimensionalData<f64>,
    pub wind_gust_speed: DimensionalData<f64>,
    pub wave_height: DimensionalData<f64>,
    pub dominant_wave_period: DimensionalData<f64>,
    pub average_wave_period: DimensionalData<f64>,
    pub mean_wave_direction: DimensionalData<Direction>,
    pub air_pressure: DimensionalData<f64>,
    pub air_pressure_tendency: DimensionalData<f64>,
    pub air_temperature: DimensionalData<f64>,
    pub water_temperature: DimensionalData<f64>, 
    pub dewpoint_temperature: DimensionalData<f64>,
    pub visibility: DimensionalData<f64>,
    pub tide: DimensionalData<f64>,
}

impl UnitConvertible<MeteorologicalDataRecord> for MeteorologicalDataRecord {
    fn to_units(&self, new_units: &Units) -> MeteorologicalDataRecord {
        MeteorologicalDataRecord {
            year: self.year.clone(),
            month: self.month.clone(), 
            day: self.day.clone(),
            hour: self.hour.clone(), 
            minute: self.minute.clone(),
            wind_direction: self.wind_direction.to_units(new_units),
            wind_speed: self.wind_speed.to_units(new_units),
            wind_gust_speed: self.wind_gust_speed.to_units(new_units),
            wave_height: self.wave_height.to_units(new_units),
            dominant_wave_period: self.dominant_wave_period.to_units(new_units),
            average_wave_period: self.average_wave_period.to_units(new_units),
            mean_wave_direction: self.mean_wave_direction.to_units(new_units),
            air_pressure: self.air_pressure.to_units(new_units),
            air_pressure_tendency: self.air_pressure_tendency.to_units(new_units),
            air_temperature: self.air_temperature.to_units(new_units),
            water_temperature: self.water_temperature.to_units(new_units),
            dewpoint_temperature: self.dewpoint_temperature.to_units(new_units),
            visibility: self.visibility.to_units(new_units),
            tide: self.tide.to_units(new_units),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WaveDataRecord {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub wave_height: DimensionalData<f64>,
    pub average_wave_period: DimensionalData<f64>,
    pub mean_wave_direction: DimensionalData<Direction>,
    pub swell_wave_height: DimensionalData<f64>,
    pub swell_period: DimensionalData<f64>, 
    pub swell_direction: DimensionalData<Direction>,
    pub wind_wave_height: DimensionalData<f64>,
    pub wind_wave_period: DimensionalData<f64>, 
    pub wind_wave_direction: DimensionalData<Direction>,
    pub steepness: Steepness,
}

impl UnitConvertible<WaveDataRecord> for WaveDataRecord {
    fn to_units(&self, new_units: &Units) -> WaveDataRecord {
        WaveDataRecord {
            year: self.year.clone(),
            month: self.month.clone(), 
            day: self.day.clone(),
            hour: self.hour.clone(), 
            minute: self.minute.clone(),
            wave_height: self.wave_height.to_units(new_units),
            average_wave_period: self.average_wave_period.to_units(new_units),
            mean_wave_direction: self.mean_wave_direction.to_units(new_units),
            swell_wave_height: self.swell_wave_height.to_units(new_units),
            swell_period: self.swell_period.to_units(new_units),
            swell_direction: self.swell_direction.to_units(new_units),
            wind_wave_height: self.wind_wave_height.to_units(new_units),
            wind_wave_period: self.wind_wave_period.to_units(new_units),
            wind_wave_direction: self.wind_wave_direction.to_units(new_units),
            steepness: self.steepness.clone(),
        }
    }
}

#[derive(Clone, Debug)]
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

impl UnitConvertible<SprectralWaveDataRecord> for SprectralWaveDataRecord {
    fn to_units(&self, new_units: &Units) -> SprectralWaveDataRecord {
        // TODO: Maybe some conversion
        SprectralWaveDataRecord{
            year: self.year.clone(),
            month: self.month.clone(),
            day: self.day.clone(),
            hour: self.hour.clone(),
            minute: self.minute.clone(),
            separation_frequency: self.separation_frequency.clone(),
            value: self.value.clone(), 
            frequency: self.frequency.clone(),
        }
    }
}

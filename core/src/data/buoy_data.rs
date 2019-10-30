use crate::data::dimensional_data::DimensionalData;
use crate::data::units::*;

#[derive(Clone, Debug)]
pub enum BuoyDataRecord {
    Latest(MeteorologicalDataRecord, WaveDataRecord),
    Meteorological(MeteorologicalDataRecord),
    Wave(WaveDataRecord),
    SprectralWave(SprectralWaveDataRecord),
}

impl UnitConvertible<BuoyDataRecord> for BuoyDataRecord {
    fn to_units(&mut self, new_units: &Units) {
        match self {
            BuoyDataRecord::Latest(met_data, wave_data) => {
                met_data.to_units(new_units);
                wave_data.to_units(new_units);
            }
            BuoyDataRecord::Meteorological(data) => data.to_units(new_units),
            BuoyDataRecord::Wave(data) => data.to_units(new_units),
            BuoyDataRecord::SprectralWave(data) => data.to_units(new_units),
        }
    }
}

impl BuoyDataRecord {
    pub fn parse_from_meteorological_data(raw_data: &str) -> Option<BuoyDataRecord> {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b' ')
            .trim(csv::Trim::All)
            .comment(Some(b'#'))
            .has_headers(false)
            .flexible(true)
            .from_reader(raw_data.as_bytes());

        let data: Vec<Option<MeteorologicalDataRecord>> = reader.records().map(|result| -> Option<MeteorologicalDataRecord> {
            if let Ok(record) = result {
                let filtered_record: Vec<&str> = record.iter().filter(|data| !data.is_empty()).collect();
                let mut met_data = MeteorologicalDataRecord::from_data_row(&filtered_record);
                met_data.to_units(&Units::Metric);
                return Some(met_data);
            }
            None
        }).filter(|result| {
            match result {
                Some(_) => true,
                None => false
            }
        }).collect();

        match data.first() {
            Some(d) => Some(BuoyDataRecord::Meteorological(d.clone().unwrap())),
            None => None
        }
    }
}

#[derive(Clone, Debug)]
pub struct DateRecord {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
}

impl DateRecord {
    fn from_data_row(row: &Vec<&str>) -> DateRecord {
        DateRecord {
            year: row[0].clone().parse().unwrap(),
            month: row[1].clone().parse().unwrap(),
            day: row[2].clone().parse().unwrap(),
            hour: row[3].clone().parse().unwrap(),
            minute: row[4].clone().parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MeteorologicalDataRecord {
    pub date: DateRecord,
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

impl MeteorologicalDataRecord {
    fn from_data_row(row: &Vec<&str>) -> MeteorologicalDataRecord {
        MeteorologicalDataRecord {
            date: DateRecord::from_data_row(row),
            wind_direction: DimensionalData::from_raw_data(row[5], "wind direction", Measurement::Direction, Units::Metric),
            wind_speed: DimensionalData::from_raw_data(row[6], "wind speed", Measurement::Speed, Units::Metric),
            wind_gust_speed: DimensionalData::from_raw_data(row[7], "wind gust speed", Measurement::Speed, Units::Metric),
            wave_height: DimensionalData::from_raw_data(row[8], "wave height", Measurement::Length, Units::Metric),
            dominant_wave_period: DimensionalData::from_raw_data(row[9], "dominant wave period", Measurement::Time, Units::Metric),
            average_wave_period: DimensionalData::from_raw_data(row[10], "average wave period", Measurement::Time, Units::Metric),
            mean_wave_direction: DimensionalData::from_raw_data(row[11], "mean wave direction", Measurement::Direction, Units::Metric),
            air_pressure: DimensionalData::from_raw_data(row[12], "air pressure", Measurement::Pressure, Units::Metric),  
            air_temperature: DimensionalData::from_raw_data(row[13], "air temperature", Measurement::Temperature, Units::Metric),
            water_temperature: DimensionalData::from_raw_data(row[14], "water temperature", Measurement::Temperature, Units::Metric),
            dewpoint_temperature: DimensionalData::from_raw_data(row[15], "dewpoint temperature", Measurement::Temperature, Units::Metric),
            visibility: DimensionalData::from_raw_data(row[16], "", Measurement::Visibility, Units::Metric),
            air_pressure_tendency: DimensionalData::from_raw_data(row[17], "air pressure tendency", Measurement::Pressure, Units::Metric),
            tide: DimensionalData::from_raw_data(row[18], "tide", Measurement::Length, Units::English),
        }
    }
}

impl UnitConvertible<MeteorologicalDataRecord> for MeteorologicalDataRecord {
    fn to_units(&mut self, new_units: &Units) {
        self.wind_direction.to_units(new_units);
        self.wind_speed.to_units(new_units);
        self.wind_gust_speed.to_units(new_units);
        self.wave_height.to_units(new_units);
        self.dominant_wave_period.to_units(new_units);
        self.average_wave_period.to_units(new_units);
        self.mean_wave_direction.to_units(new_units);
        self.air_pressure.to_units(new_units);
        self.air_pressure_tendency.to_units(new_units);
        self.air_temperature.to_units(new_units);
        self.water_temperature.to_units(new_units);
        self.dewpoint_temperature.to_units(new_units);
        self.visibility.to_units(new_units);
        self.tide.to_units(new_units);
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
    fn to_units(&mut self, new_units: &Units) {
        self.wave_height.to_units(new_units);
        self.average_wave_period.to_units(new_units);
        self.mean_wave_direction.to_units(new_units);
        self.swell_wave_height.to_units(new_units);
        self.swell_period.to_units(new_units);
        self.swell_direction.to_units(new_units);
        self.wind_wave_height.to_units(new_units);
        self.wind_wave_period.to_units(new_units);
        self.wind_wave_direction.to_units(new_units);
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
    fn to_units(&mut self, new_units: &Units) {
        // TODO: Maybe some conversion
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn read_meteorological_data() {
        let raw_data = fs::read_to_string("mock/44017.met.txt").unwrap();
        let read_data = BuoyDataRecord::parse_from_meteorological_data(raw_data.as_str());
        assert_ne!(read_data.is_none(), true);
        if let Some(met_data) = read_data {
            match met_data {
                BuoyDataRecord::Meteorological(d) => {
                    assert_eq!(d.tide.value.is_none(), true);

                    // TODO: More tests
                },
                _ => {}
            };
        }
    }

}
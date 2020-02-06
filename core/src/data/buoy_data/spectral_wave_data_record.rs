use crate::data::units::*;

use super::date_record::DateRecord;
use super::parseable_data_record::ParseableDataRecord;

#[derive(Clone, Debug)]
pub struct SpectralWaveDataRecord {
    pub date: DateRecord,
    pub separation_frequency: f64,
    pub value: Vec<f64>,
    pub frequency: Vec<f64>,
}

impl ParseableDataRecord for SpectralWaveDataRecord {
    fn from_data_row(row: &Vec<&str>) -> SpectralWaveDataRecord {
        let freq_count = (row.len() - 6) / 2;

        let mut values: Vec<f64> = vec![0.0; freq_count];
        let mut freqs: Vec<f64> = vec![0.0; freq_count];

        for i in 0..freq_count {
            let index = i * 2;

            values[i] = row[index].parse().unwrap();
            freqs[i] = row[index+1].replace(&['(', ')'][..], "").parse().unwrap();
        }

        SpectralWaveDataRecord {
            date: DateRecord::from_data_row(row),
            separation_frequency: row[5].parse().unwrap_or(9.999),
            value: values,
            frequency: freqs
        }
    }
}

impl UnitConvertible<SpectralWaveDataRecord> for SpectralWaveDataRecord {
    fn to_units(&mut self, new_units: &Units) {
        // TODO: Maybe some conversion
    }
}

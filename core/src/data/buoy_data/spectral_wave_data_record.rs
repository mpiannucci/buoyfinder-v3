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
        const START_INDEX: usize = 6;
        let freq_count = (row.len() - START_INDEX) / 2;

        let mut values: Vec<f64> = vec![0.0; freq_count];
        let mut freqs: Vec<f64> = vec![0.0; freq_count];

        for i in 0..freq_count {
            let index = START_INDEX + i * 2;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_row_parse() {
        let raw_data = "2018 09 01 10 00 9.999 0.000 (0.033) 0.000 (0.038) 0.000 (0.043) 0.000 (0.048) 0.000 (0.053) 0.000 (0.058) 0.000 (0.063) 0.021 (0.068) 0.021 (0.073) 0.074 (0.078) 0.085 (0.083) 0.074 (0.088) 0.085 (0.093) 0.085 (0.100) 0.148 (0.110) 0.138 (0.120) 0.074 (0.130) 0.244 (0.140) 0.392 (0.150) 0.477 (0.160) 0.572 (0.170) 1.060 (0.180) 0.339 (0.190) 0.382 (0.200) 0.265 (0.210) 0.265 (0.220) 0.318 (0.230) 0.329 (0.240) 0.329 (0.250) 0.350 (0.260) 0.244 (0.270) 0.371 (0.280) 0.180 (0.290) 0.180 (0.300) 0.170 (0.310) 0.117 (0.320) 0.127 (0.330) 0.095 (0.340) 0.064 (0.350) 0.085 (0.365) 0.085 (0.385) 0.074 (0.405) 0.021 (0.425) 0.011 (0.445) 0.021 (0.465) 0.011 (0.485)";
        let data_row: Vec<&str> = raw_data.split_whitespace().collect();

        let spectral_data = SpectralWaveDataRecord::from_data_row(&data_row);
        
        assert!((spectral_data.separation_frequency - 9.999).abs() < 0.0001);
        assert_eq!(spectral_data.frequency.len(), 46);
        assert_eq!(spectral_data.value.len(), 46);
        assert!((spectral_data.frequency[2] - 0.043).abs() < 0.0001);
        assert!((spectral_data.value[9] - 0.074).abs() < 0.0001);
    }
}
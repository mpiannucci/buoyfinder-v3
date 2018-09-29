use std::str::FromStr;
use std::num::ParseIntError;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            red: r, 
            green: g,
            blue: b,
        }
    }
}

impl From<i64> for Color {
    fn from(value: i64) -> Self {
        Color {
            red: (((value >> 16) & 0xff) as f64) / 255.0,
            green: (((value >> 8) & 0xff) as f64) / 255.0, 
            blue: ((value & 0xff) as f64) / 255.0,
        }
    }
}

impl From<i32> for Color {
    fn from(value: i32) -> Self {
        Color::from(value as i64)
    }
}

impl FromStr for Color {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, stripped_color) = s.trim().split_at(1);
        let color_int = stripped_color.parse::<i64>()?;
        Ok(Color::from(color_int))
    }
}
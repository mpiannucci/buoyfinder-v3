use std::str::FromStr;
use std::num::ParseIntError;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            red: r, 
            green: g,
            blue: b,
            alpha: a,
        }
    }

    pub fn black() -> Self {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    }
}

impl From<i64> for Color {
    fn from(value: i64) -> Self {
        Color {
            red: ((value >> 24) & 0xff) as u8,
            green: ((value >> 16) & 0xff) as u8, 
            blue: ((value >> 8) & 0xff) as u8,
            alpha: (value & 0xff) as u8,
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

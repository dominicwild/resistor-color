use std::fmt::Display;

use enum_iterator::Sequence;
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Sequence, Clone, Copy, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl Display for ResistorColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(resistance_value: u32) -> String {
    match ResistorColor::from_int(resistance_value) {
        Ok(resistor_color) => resistor_color.to_string(),
        Err(_) => "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut collect: Vec<ResistorColor> = enum_iterator::all().collect();
    collect.sort_by_key(|a| a.int_value());
    collect
}

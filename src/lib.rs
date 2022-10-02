use std::fmt::Display;

use enum_iterator::Sequence;

#[derive(Debug, PartialEq, Eq, Sequence, Clone)]
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

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color as u32
}

pub fn value_to_color_string(resistance_value: u32) -> String {
    for resistor_color in enum_iterator::all::<ResistorColor>() {
        if color_to_value(resistor_color.clone()) == resistance_value {
            return resistor_color.to_string();
        }
    }

    "value out of range".to_string()
}

pub fn colors() -> Vec<ResistorColor> {
    let mut collect: Vec<ResistorColor> = enum_iterator::all().collect::<Vec<_>>();
    collect.sort_by(|a, b| color_to_value(a.clone()).cmp(&color_to_value(b.clone())));
    return collect;
}

#![allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorName {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Yellow = 6,
    White = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Dim(ColorName),
    Bright(ColorName),
}

impl Color {
    pub(super) const fn as_byte(&self) -> u8 {
        match &self {
            Color::Dim(color_name) => *color_name as u8,
            Color::Bright(color_name) => (*color_name as u8) | 8,
        }
    }
}

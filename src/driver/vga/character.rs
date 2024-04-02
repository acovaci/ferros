use super::attribute;
use super::color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Character {
    pub codepoint: u8,
    pub attribute: u8,
}

const UNPRINTABLE_CHAR_VALUE: u8 = 0xfe;

impl Character {
    pub(super) fn new(codepoint: u8, style: attribute::Attribute) -> Self {
        let value = match codepoint {
            0x20..=0x7e | b'\n' => codepoint,
            _ => UNPRINTABLE_CHAR_VALUE,
        };

        Self {
            codepoint: value,
            attribute: style.as_byte(),
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            codepoint: b'\0',
            attribute: attribute::Attribute::new(
                color::Color::Dim(color::ColorName::Black),
                color::Color::Bright(color::ColorName::White),
            )
            .as_byte(),
        }
    }
}

impl Into<u16> for Character {
    fn into(self) -> u16 {
        let codepoint = self.codepoint as u16;
        let attribute = self.attribute as u16;
        codepoint | (attribute << 8)
    }
}

impl From<u16> for Character {
    fn from(value: u16) -> Self {
        Self {
            codepoint: value as u8,
            attribute: (value >> 8) as u8,
        }
    }
}

impl From<u8> for Character {
    fn from(value: u8) -> Self {
        Self {
            codepoint: value as u8,
            attribute: attribute::Attribute::default().as_byte(),
        }
    }
}

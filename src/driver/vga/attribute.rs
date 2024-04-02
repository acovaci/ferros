use super::color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Attribute {
    pub(super) background: color::Color,
    pub(super) foreground: color::Color,
}

impl Attribute {
    pub(super) const fn new(background: color::Color, foreground: color::Color) -> Attribute {
        Attribute {
            background,
            foreground,
        }
    }

    pub(super) const fn as_byte(self) -> u8 {
        (self.background.as_byte() << 4) | (self.foreground.as_byte())
    }
}

impl Default for Attribute {
    fn default() -> Self {
        Attribute::new(
            color::Color::Dim(color::ColorName::Black),
            color::Color::Bright(color::ColorName::White),
        )
    }
}

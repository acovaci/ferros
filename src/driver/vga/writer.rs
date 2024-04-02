use super::attribute;
use super::buffer;
use super::character;
use super::color;

pub struct Writer {
    buffer: &'static mut buffer::VGABuffer,
    position: (usize, usize),
    style: attribute::Attribute,
}

impl Writer {
    pub fn write_str(&mut self, string: &str) {
        self.write_bytes(string.as_bytes());
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.position = (0, 0);
    }

    pub fn new_line(&mut self) {
        if self.position.0 >= buffer::VGABuffer::HEIGHT - 1 {
            self.buffer.move_up(1);
            self.position.1 = 0;
            return;
        }

        self.position.0 += 1;
        self.position.1 = 0;
    }

    pub fn set_style(&mut self, background: color::Color, foreground: color::Color) {
        self.style = attribute::Attribute::new(background, foreground);
    }

    pub fn reset_style(&mut self) {
        self.style = attribute::Attribute::default();
    }

    pub fn set_background(&mut self, background: color::Color) {
        let attribute::Attribute {
            background: _,
            foreground,
        } = self.style;
        self.set_style(background, foreground)
    }

    pub fn set_foreground(&mut self, foreground: color::Color) {
        let attribute::Attribute {
            background,
            foreground: _,
        } = self.style;
        self.set_style(background, foreground)
    }

    pub(super) fn new() -> Writer {
        Writer {
            buffer: buffer::VGABuffer::get(),
            position: (0, 0),
            style: attribute::Attribute::default(),
        }
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        for k in 0..bytes.len() {
            self.write_byte(bytes[k]);
        }
    }

    fn write_byte(&mut self, value: u8) {
        if value == b'\n' {
            self.new_line();
            return;
        }

        if self.position.1 >= buffer::VGABuffer::WIDTH {
            self.new_line();
        }

        self.write_byte_at(value, (self.position.0, self.position.1));

        self.position.1 += 1;
    }

    fn write_byte_at(&mut self, value: u8, position: (usize, usize)) {
        self.buffer
            .set(position, character::Character::new(value, self.style));
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        self.write_bytes(string.as_bytes());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Writer {
        pub(crate) fn assert_buffer_text_eq(&mut self, expected: buffer::VGABuffer) {
            let buffer = self.buffer.get_text();
            let expected = expected.get_text();

            assert_eq!(buffer, expected);
        }
    }
}

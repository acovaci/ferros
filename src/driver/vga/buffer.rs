use super::character;

pub(super) const VGA_BUFFER_WIDTH: usize = 80;
pub(super) const VGA_BUFFER_HEIGHT: usize = 25;

pub(crate) struct VGABuffer {
    buffer: crate::libs::buffer::GridBuffer<u16, VGA_BUFFER_WIDTH, VGA_BUFFER_HEIGHT>,
}

impl VGABuffer {
    pub const WIDTH: usize = VGA_BUFFER_WIDTH;
    pub const HEIGHT: usize = VGA_BUFFER_HEIGHT;

    pub fn get() -> &'static mut VGABuffer {
        unsafe { &mut *(0xb8000 as *mut VGABuffer) }
    }

    pub fn set(&mut self, position: (usize, usize), char: character::Character) {
        self.buffer.content[position.0][position.1] = char.into();
    }

    pub fn move_up(&mut self, count: usize) {
        self.buffer.shift((0, -(count as isize)));
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn get_text(&self) -> [[u8; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT] {
        let mut text = [[b'\0'; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT];

        for line_pos in 0..VGA_BUFFER_HEIGHT {
            for char_pos in 0..VGA_BUFFER_WIDTH {
                let codepoint =
                    character::Character::from(self.buffer.content[line_pos][char_pos]).codepoint;
                text[line_pos][char_pos] = codepoint;
            }
        }

        text
    }
}

#[cfg(test)]
pub mod testing {
    use super::*;

    pub fn construct_buffer_from_strings(lines: &[&str]) -> VGABuffer {
        let mut buffer = VGABuffer {
            buffer: crate::libs::buffer::GridBuffer::new_with_default(
                character::Character::default().into(),
            ),
        };

        for (line_pos, line) in lines.iter().enumerate() {
            for (char_pos, char) in line.chars().enumerate() {
                buffer.set((line_pos, char_pos), character::Character::from(char as u8));
            }
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_construct_buffer_from_strings() {
        let lines = ["abc", "defgh", ""];
        let buffer = testing::construct_buffer_from_strings(&lines);

        let mut expected_buffer = [[b'\0' as u16 | 0xF00; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT];
        expected_buffer[0][0] = b'a' as u16 | 0xF00;
        expected_buffer[0][1] = b'b' as u16 | 0xF00;
        expected_buffer[0][2] = b'c' as u16 | 0xF00;
        expected_buffer[1][0] = b'd' as u16 | 0xF00;
        expected_buffer[1][1] = b'e' as u16 | 0xF00;
        expected_buffer[1][2] = b'f' as u16 | 0xF00;
        expected_buffer[1][3] = b'g' as u16 | 0xF00;
        expected_buffer[1][4] = b'h' as u16 | 0xF00;

        assert_eq!(buffer.buffer.content, expected_buffer);
    }
}

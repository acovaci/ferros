mod attribute;
mod buffer;
mod character;
mod color;
mod writer;

pub use color::{Color, ColorName};

lazy_static::lazy_static! {
    pub static ref VGA_WRITER: spin::Mutex<writer::Writer> = spin::Mutex::new(writer::Writer::new());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::driver::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    VGA_WRITER.lock().write_fmt(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_println() {
        VGA_WRITER.lock().clear();

        println!("Some {} chars", 42);
        println!("Some more text");

        let expected_lines = ["Some 42 chars", "Some more text"];
        let expected_buffer = buffer::testing::construct_buffer_from_strings(&expected_lines);

        VGA_WRITER.lock().assert_buffer_text_eq(expected_buffer);
    }
}

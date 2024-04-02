const QEMU_STDIO_PORT: u16 = 0x3F8;

lazy_static::lazy_static! {
    pub static ref QEMU_STDIO: spin::Mutex<uart_16550::SerialPort> = {
        let mut serial_port = unsafe { uart_16550::SerialPort::new(QEMU_STDIO_PORT) };
        serial_port.init();
        spin::Mutex::new(serial_port)
    };
}

macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::libs::testing::serial::_print(format_args!($($arg)*));
    };
}

macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::libs::testing::serial::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::libs::testing::serial::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

pub(crate) use {serial_print, serial_println};

pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    QEMU_STDIO
        .lock()
        .write_fmt(args)
        .expect("Unexpected error while printing to serial");
}

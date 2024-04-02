#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(ferros::libs::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ferros::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]
    main();

    #[cfg(test)]
    test_main();

    loop {}
}

pub fn main() {
    ferros::driver::vga::VGA_WRITER
        .lock()
        .set_foreground(ferros::driver::vga::Color::Bright(
            ferros::driver::vga::ColorName::Green,
        ));
    for i in 0..100 {
        println!("line {}", i)
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    ferros::libs::testing::panic(info)
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn test_run() {
        assert!(true);
    }
}

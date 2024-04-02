#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::libs::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod driver;
pub mod libs;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::libs::testing::panic(info)
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn test_run_lib() {
        assert!(true);
    }
}

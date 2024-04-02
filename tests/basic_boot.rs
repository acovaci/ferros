#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ferros::libs::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    ferros::libs::testing::panic(info)
}

#[test_case]
fn test_basic_boot() {
    ferros::println!("Boot successful!");
}

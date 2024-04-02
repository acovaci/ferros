mod qemu;
mod serial;
mod traits;

pub fn panic(info: &core::panic::PanicInfo) -> ! {
    serial::serial_println!("[failed]\n");
    serial::serial_println!("Error: {}\n", info);
    qemu::fail();
    loop {}
}

pub fn test_runner(tests: &[&dyn traits::Testable]) {
    serial::serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    qemu::success();
}

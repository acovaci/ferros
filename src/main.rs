#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(ferros::libs::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ferros::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // ferros::init();

    // x86_64::instructions::interrupts::int3();

    #[cfg(not(test))]
    main();

    #[cfg(test)]
    test_main();

    loop {}
}

pub fn main() {
    println!(
        "{:#?}",
        ferros::nucleus::interrupt::idt::INTERRUPT_DESCRIPTOR_TABLE.entries
            [ferros::nucleus::interrupt::idt::InterruptDescriptorTableIndex::DivisionError
                as usize]
    );

    ferros::nucleus::interrupt::idt::INTERRUPT_DESCRIPTOR_TABLE.load();
    // println!("Loaded IDT");

    divide_by_zero();
}

fn divide_by_zero() {
    unsafe {
        core::arch::asm!(
            "mov dx, 0",
            "div dx",
            options(nostack, nomem, preserves_flags)
        );
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

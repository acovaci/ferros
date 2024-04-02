enum QEMUExitCodes {
    Success = 0x10,
    Failure = 0x11,
}

const QEMU_PORT: u16 = 0xf4;

pub fn success() {
    exit(QEMUExitCodes::Success);
}

pub fn fail() {
    exit(QEMUExitCodes::Failure);
}

fn exit(exit_code: QEMUExitCodes) {
    unsafe {
        let mut port = x86_64::instructions::port::Port::new(QEMU_PORT);
        port.write(exit_code as u32);
    }
}

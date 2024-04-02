use x86_64::registers::segmentation::Segment;

#[repr(u8)]
pub enum InterruptDescriptorTableIndex {
    DivisionError = 0,
    Debug = 1,
    NonMaskableInterrupt = 2,
    Breakpoint = 3,
    Overflow = 4,
    BoundRangeExceeded = 5,
    InvalidOpcode = 6,
    DeviceNotAvailable = 7,
    DoubleFault = 8,
    InvalidTaskStateSegment = 10,
    SegmentNotPresent = 11,
    StackSegmentFault = 12,
    GeneralProtectionFault = 13,
    PageFault = 14,
    X87FloatingPointException = 16,
    AlignmentCheck = 17,
    MachineCheck = 18,
    SingleInstructionMultipleDataException = 19,
    VirtualizationException = 20,
    ControlProtectionException = 21,
    HypervisorInjectionException = 28,
    VirtualMachineMonitorCommunicationException = 29,
    SecurityException = 30,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct InterruptDescriptorTableEntry {
    pub function_pointer_low: u16,
    pub global_descriptor_selector: u16,
    pub options: u16,
    pub function_pointer_middle: u16,
    pub function_pointer_high: u32,
}

impl InterruptDescriptorTableEntry {
    pub const fn empty() -> Self {
        Self {
            function_pointer_low: 0,
            global_descriptor_selector: 0,
            options: InterruptDescriptorOptions::empty().as_u16(),
            function_pointer_middle: 0,
            function_pointer_high: 0,
        }
    }
}

impl Default for InterruptDescriptorTableEntry {
    fn default() -> Self {
        Self::empty()
    }
}

type HandlerFunction = extern "C" fn() -> !;

pub struct InterruptDescriptor {
    pub function: HandlerFunction,
    pub global_descriptor_selector: u16,
    pub options: InterruptDescriptorOptions,
}

impl InterruptDescriptor {
    pub fn new(function: HandlerFunction, options: InterruptDescriptorOptions) -> Self {
        Self {
            function,
            global_descriptor_selector: x86_64::registers::segmentation::CS::get_reg().0,
            options,
        }
    }
}

pub enum InterruptGateType {
    Trap,
    Interrupt,
}

impl From<bool> for InterruptGateType {
    fn from(byte: bool) -> Self {
        if byte {
            Self::Trap
        } else {
            Self::Interrupt
        }
    }
}

pub struct InterruptDescriptorOptions {
    pub present: bool,
    pub descriptor_privilege_level: u8,
    pub gate_type: InterruptGateType,
    pub interrupt_stack_table: u8,
}

impl InterruptDescriptorOptions {
    pub fn new(
        present: bool,
        descriptor_privilege_level: u8,
        gate_type: InterruptGateType,
        interrupt_stack_table: u8,
    ) -> Self {
        Self {
            present,
            descriptor_privilege_level,
            gate_type: gate_type.into(),
            interrupt_stack_table,
        }
    }
}

impl InterruptDescriptorOptions {
    pub const fn empty() -> Self {
        Self {
            present: false,
            descriptor_privilege_level: 0,
            gate_type: InterruptGateType::Interrupt,
            interrupt_stack_table: 0,
        }
    }
}

impl InterruptDescriptorOptions {
    pub const fn as_u16(&self) -> u16 {
        let options = match self.present {
            true => 0b1000_0000_0000_0000,
            false => 0b0000_0000_0000_0000,
        };

        let options = options | ((self.descriptor_privilege_level as u16) << 13);

        let options = options & 0b1110_1111_1111_1111;
        let options = options | 0b0000_1110_0000_0000;

        let options = match self.gate_type {
            InterruptGateType::Trap => options | 0b0000_0001_0000_0000,
            InterruptGateType::Interrupt => options & 0b1111_1110_1111_1111,
        };

        let options = options | (self.interrupt_stack_table as u16);

        options
    }
}

impl Default for InterruptDescriptorOptions {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<u16> for InterruptDescriptorOptions {
    fn from(options: u16) -> Self {
        let present = options & 0x8000 == 0x8000;
        let descriptor_privilege_level = ((options & 0x6000) >> 13) as u8;

        let zeros = options & 0x1000 == 0;
        let ones = options & 0x0C00 == 0x0C00;

        let gate_type = (options & 0x0100) == 0x0100;
        let interrupt_stack_table = (options & 0x0003) as u8;

        if !zeros {
            panic!("Invalid Interrupt Descriptor Entry: Zeros are not set");
        }

        if !ones {
            panic!("Invalid Interrupt Descriptor Entry: Ones are not set");
        }

        Self {
            present,
            descriptor_privilege_level,
            gate_type: gate_type.into(),
            interrupt_stack_table,
        }
    }
}

impl Into<u16> for InterruptDescriptorOptions {
    fn into(self) -> u16 {
        self.as_u16()
    }
}

impl Into<InterruptDescriptorTableEntry> for InterruptDescriptor {
    fn into(self) -> InterruptDescriptorTableEntry {
        let function_pointer = self.function as u64;

        let function_pointer_low = function_pointer as u16;
        let function_pointer_middle = (function_pointer >> 16) as u16;
        let function_pointer_high = (function_pointer >> 32) as u32;

        InterruptDescriptorTableEntry {
            function_pointer_low,
            global_descriptor_selector: self.global_descriptor_selector,
            options: self.options.into(),
            function_pointer_middle,
            function_pointer_high,
        }
    }
}

const INTERRUPT_DESCRIPTOR_TABLE_SIZE: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct InterruptDescriptorTable<const N: usize> {
    pub entries: [InterruptDescriptorTableEntry; N],
}

impl<const N: usize> InterruptDescriptorTable<N> {
    pub const fn empty() -> Self {
        Self {
            entries: [InterruptDescriptorTableEntry::empty(); N],
        }
    }

    pub fn load(&'static self) {
        use core::mem::size_of;
        use x86_64::instructions::tables::{lidt, DescriptorTablePointer};

        let ptr = DescriptorTablePointer {
            base: x86_64::VirtAddr::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { lidt(&ptr) };
    }

    pub fn set_handler(&mut self, index: InterruptDescriptorTableIndex, handler: HandlerFunction) {
        let descriptor = InterruptDescriptor::new(
            handler,
            InterruptDescriptorOptions::new(true, 0, InterruptGateType::Interrupt, 0),
        );
        self.entries[index as usize] = descriptor.into();
    }
}

impl<const N: usize> Default for InterruptDescriptorTable<N> {
    fn default() -> Self {
        Self::empty()
    }
}

extern "C" fn handle_zero_division() -> ! {
    use crate::println;

    println!("Handling zero division");

    loop {}
}

lazy_static::lazy_static!(
    pub static ref INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable<INTERRUPT_DESCRIPTOR_TABLE_SIZE> = {
        let mut idt = InterruptDescriptorTable::empty();
        idt.set_handler(InterruptDescriptorTableIndex::DivisionError, handle_zero_division);
        idt
    };
);

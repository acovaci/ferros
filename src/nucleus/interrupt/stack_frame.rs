pub struct InterruptStackFrame {
    instruction_pointer: u64,
    stack_pointer: u64,
    frame_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_segment: u64,
    source_segment: u64,
    source_offset: u64,
}

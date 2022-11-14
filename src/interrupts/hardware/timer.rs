use crate::interrupts::hardware::{HwInterruptIndex, PICS};
use crate::print;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub(super) fn set_handler(idt: &mut InterruptDescriptorTable) {
    idt[HwInterruptIndex::Timer.as_usize()].set_handler_fn(timer_handler);
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(HwInterruptIndex::Timer.as_u8())
    };
}

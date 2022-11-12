#[cfg(test)]
use core::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref INTERRUPT_DT: InterruptDescriptorTable = new_idt();
}

/// Initializes the [Interrupt Descriptor Table](https://en.wikipedia.org/wiki/Interrupt_descriptor_table),
/// so the CPU will be able to call its handler functions.
pub fn init_idt() { INTERRUPT_DT.load() }

/// Creates a new instance of the [Interrupt Descriptor Table](https://en.wikipedia.org/wiki/Interrupt_descriptor_table)
/// and sets the required handlers.
fn new_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt
}

#[cfg(not(test))]
extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    use core::fmt::Write;

    crate::io::print::VGA_OUTPUT
        .lock()
        .write_str("> breakpoint\n")
        .unwrap();
}

#[cfg(test)]
static BREAK_POINTER_INVOKED: AtomicBool = AtomicBool::new(false);

#[cfg(test)]
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::serial_println!("> breakpoint: {:?}", stack_frame);
    BREAK_POINTER_INVOKED.store(true, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::BREAK_POINTER_INVOKED;
    use core::sync::atomic::Ordering;

    #[test_case]
    fn test_breakpoint_dont_panic() {
        /// This invokes a breakpoint exception.
        x86_64::instructions::interrupts::int3();
        assert!(BREAK_POINTER_INVOKED.load(Ordering::Relaxed));
    }
}

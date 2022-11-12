#[cfg(test)]
use core::sync::atomic::{AtomicBool, Ordering};
use x86_64::structures::idt::InterruptStackFrame;

#[cfg(not(test))]
pub(super) extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::println!("EXCEPTION: BREAKPOINT");
    crate::println!("{stack_frame:?}");
}

#[cfg(test)]
static BREAK_POINT_INVOKED: AtomicBool = AtomicBool::new(false);

#[cfg(test)]
pub(super) extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::serial_println!("EXCEPTION: BREAKPOINT");
    crate::serial_println!("{stack_frame:?}");
    BREAK_POINT_INVOKED.store(true, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::BREAK_POINT_INVOKED;
    use core::sync::atomic::Ordering;

    #[test_case]
    fn test_breakpoint_dont_panic() {
        // This invokes a breakpoint exception.
        x86_64::instructions::interrupts::int3();
        assert!(BREAK_POINT_INVOKED.load(Ordering::Relaxed));
    }
}

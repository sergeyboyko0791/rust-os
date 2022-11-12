use crate::utils::PAGE_SIZE;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::VirtAddr;

/// The zero element within the [`TaskStateSegment::interrupt_stack_table`] list.
pub(crate) const DOUBLE_FAULT_IST_INDEX: u16 = 0;
const DOUBLE_FAULT_STACK_SIZE: usize = PAGE_SIZE * 4;

/// Returns the pointer to the beginning of the double-fault exception stack.
pub(crate) fn allocate_double_fault_stack() -> VirtAddr {
    // Must be mutable data because otherwise the bootloader will map it to a read-only page.
    static mut DOUBLE_FAULT_STACK: [u8; DOUBLE_FAULT_STACK_SIZE] = [0; DOUBLE_FAULT_STACK_SIZE];

    let stack_bottom = VirtAddr::from_ptr(unsafe { &DOUBLE_FAULT_STACK });
    // The stack is growing downward at the virtual memory scheme: https://i.stack.imgur.com/dvK8G.png
    // But as the stack grows, the virtual address decreases.
    // So the beginning of the stack is the actual end of the allocated data.
    let stack_top = stack_bottom + DOUBLE_FAULT_STACK_SIZE;
    stack_top
}

// TODO https://i.stack.imgur.com/dvK8G.png
pub(super) extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    crate::println!("EXCEPTION: DOUBLE_FAULT error_code={error_code}");
    crate::println!("{stack_frame:?}");

    loop {}
}

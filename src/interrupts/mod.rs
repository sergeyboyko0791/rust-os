use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

mod breakpoint;
mod double_fault;
mod gdt;

lazy_static! {
    static ref INTERRUPT_DT: InterruptDescriptorTable = new_idt();
}

/// Initializes the [Interrupt Descriptor Table](https://en.wikipedia.org/wiki/Interrupt_descriptor_table),
/// so the CPU will be able to call its handler functions.
pub fn init() {
    gdt::init();
    INTERRUPT_DT.load();
}

/// Creates a new instance of the [Interrupt Descriptor Table](https://en.wikipedia.org/wiki/Interrupt_descriptor_table)
/// and sets the required handlers.
fn new_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint
        .set_handler_fn(breakpoint::breakpoint_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault::double_fault_handler)
            .set_stack_index(double_fault::DOUBLE_FAULT_IST_INDEX);
    }
    idt
}

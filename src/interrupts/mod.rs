use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

mod breakpoint;
mod double_fault;
mod gdt;
mod hardware;

lazy_static! {
    static ref INTERRUPT_DT: InterruptDescriptorTable = new_idt();
}

/// Initializes the [Interrupt Descriptor Table](https://en.wikipedia.org/wiki/Interrupt_descriptor_table),
/// so the CPU will be able to call its handler functions.
pub fn init() {
    gdt::init();
    INTERRUPT_DT.load();
    hardware::init();
}

/// Creates a new instance of the [Interrupt Descriptor Table](https://en.wikipedia.org/wiki/Interrupt_descriptor_table)
/// and sets the required handlers.
fn new_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();
    breakpoint::set_handler(&mut idt);
    double_fault::set_handler(&mut idt);
    hardware::set_handlers(&mut idt);
    idt
}

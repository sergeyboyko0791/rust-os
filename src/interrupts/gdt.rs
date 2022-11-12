use crate::interrupts::double_fault::{allocate_double_fault_stack, DOUBLE_FAULT_IST_INDEX};
use lazy_static::lazy_static;
use x86_64::instructions::segmentation::{Segment, CS};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;

lazy_static! {
    /// GDT is no longer used by the x86_64 systems for its initial purposes,
    /// but it's still used to initialize a `Task State Segment` and its `Interrupt Stack Table`.
    static ref GDT: GlobalDescriptorTableWithSelectors = GlobalDescriptorTableWithSelectors::new();
    static ref TSS: TaskStateSegment = new_tss();
}

pub fn init() {
    GDT.gdt.load();
    unsafe {
        // Overwrite the `Code Segment` selector.
        CS::set_reg(GDT.cs_selector);
        // Overwrite the `Task State Segment` selector.
        load_tss(GDT.tss_selector);
    }
}

/// The wrapper over `GlobalDescriptorTable` and its selector descriptors.
struct GlobalDescriptorTableWithSelectors {
    gdt: GlobalDescriptorTable,
    /// `Code Segment` selector.
    cs_selector: SegmentSelector,
    /// `Task State Segment` selector.
    tss_selector: SegmentSelector,
}

impl GlobalDescriptorTableWithSelectors {
    fn new() -> GlobalDescriptorTableWithSelectors {
        let mut gdt = GlobalDescriptorTable::new();
        let cs_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));

        GlobalDescriptorTableWithSelectors {
            gdt,
            cs_selector,
            tss_selector,
        }
    }
}

fn new_tss() -> TaskStateSegment {
    let mut tss = TaskStateSegment::new();
    init_interrupt_stack_table(&mut tss);
    tss
}

/// Initializes the `Interrupt Stack Table` to avoid `Third Page Fault` if the kernel's stack is overflowed.
fn init_interrupt_stack_table(tss: &mut TaskStateSegment) {
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = allocate_double_fault_stack();
}

use bootloader::BootInfo;
use x86_64::structures::paging::page_table::FrameError;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::{registers::control::Cr3, structures::paging::PageTable, PhysAddr, VirtAddr};

pub struct MemoryCtx {
    boot_info: &'static BootInfo,
}

impl MemoryCtx {
    pub fn new(boot_info: &'static BootInfo) -> MemoryCtx { MemoryCtx { boot_info } }

    pub fn physical_memory_offset(&self) -> VirtAddr {
        VirtAddr::new(self.boot_info.physical_memory_offset)
    }

    // fn active_page_table_level_4(&self) -> &'static mut PageTable {
    //
    // }
}

unsafe fn read_active_page_table_level_4(ctx: &MemoryCtx) -> &'static mut PageTable {
    let (table_phys_frame, _flags) = Cr3::read();

    // Get an address of the first byte of the Page Table Level 4.
    let table_phys_addr = table_phys_frame.start_address();
    // Get a virtual address that points to the first byte of the Page Table Level 4.
    // Please note that starting from `ctx.physical_memory_offset()` we have a virtual memory that maps the physical memory.
    let table_virt_addr = ctx.physical_memory_offset() + table_phys_addr.as_u64();

    let page_table_level_4: *mut PageTable = table_virt_addr.as_mut_ptr();
    &mut *page_table_level_4
}

unsafe fn translate_virt_addr(ctx: &MemoryCtx, virt_addr: VirtAddr) -> Option<PhysAddr> {
    // At this moment, `frame` points to the physical frame of the Page Table Level 4.
    let (mut frame, _flags) = Cr3::read();

    let page_indexes = [
        virt_addr.p4_index(),
        virt_addr.p3_index(),
        virt_addr.p2_index(),
        virt_addr.p1_index(),
    ];

    for page_entry_index in page_indexes {
        // Get an address of the first byte of this Page Table.
        let table_phys_addr = frame.start_address();
        // Get a virtual address that points to the first byte of the Page Table Level 4.
        // Please note that starting from `ctx.physical_memory_offset()` we have a virtual memory that maps the physical memory.
        let table_virt_addr = ctx.physical_memory_offset() + table_phys_addr.as_u64();

        let page_table = &mut *table_virt_addr.as_mut_ptr::<PageTable>();

        frame = match page_table[page_entry_index].frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            // Huge frames are not supported yet.
            Err(FrameError::HugeFrame) => return None,
        };
    }

    let page_offset = u64::from(virt_addr.page_offset());
    Some(frame.start_address() + page_offset)
}

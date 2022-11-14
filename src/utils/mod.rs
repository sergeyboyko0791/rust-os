pub mod sync;

pub const PAGE_SIZE: usize = 4096;

pub fn halt_endless_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

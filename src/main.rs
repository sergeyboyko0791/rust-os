#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_impl::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rust_kernel::println;

rust_kernel::impl_test_runner!();

#[cfg(not(test))]
bootloader::entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static bootloader::BootInfo) -> ! {
    let memory_ctx = rust_kernel::memory::MemoryCtx::new(boot_info);
    println!("> RUN");
    println!(
        "> PHYSICAL_MEMORY_OFFSET={:?}",
        memory_ctx.physical_memory_offset()
    );

    rust_kernel::init();
    rust_kernel::utils::halt_endless_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC:");
    println!("{info}");

    rust_kernel::utils::halt_endless_loop()
}

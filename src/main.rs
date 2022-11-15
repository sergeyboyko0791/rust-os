#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_impl::test_runner)]
#![reexport_test_harness_main = "test_main"]

rust_kernel::impl_test_runner!();

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() { kernel_main(); }

#[cfg(not(test))]
fn kernel_main() -> ! {
    rust_kernel::init();

    rust_kernel::utils::halt_endless_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rust_kernel::println!("PANIC:");
    rust_kernel::println!("{info}");

    rust_kernel::utils::halt_endless_loop()
}

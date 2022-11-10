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
    use rust_kernel::io;

    let chars = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b' ',
    ];

    let mut current = 0;

    for i in 0..200_000_000u64 {
        if i % 30_000 == 0 {
            if current == chars.len() {
                current = 0;
            }

            io::print::VGA_OUTPUT.lock().push_byte(
                chars[current],
                io::vga_writer::Color::White,
                io::vga_writer::Color::Black,
            );

            current += 1;
        }
    }

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    use rust_kernel::io;

    io::print::VGA_OUTPUT
        .lock()
        .write_str("Panic acquired")
        .ok();
    loop {}
}

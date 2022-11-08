#![no_main]
#![no_std]

mod io;
mod utils;

#[no_mangle]
pub extern "C" fn _start() {
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
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

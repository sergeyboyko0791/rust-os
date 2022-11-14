use crate::io::vga_writer::VgaWriter;
use crate::utils::sync::SpinLockWithoutInterrupts;
use core::fmt::{self, Write};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VGA_OUTPUT: SpinLockWithoutInterrupts<VgaWriter> =
        SpinLockWithoutInterrupts::new(VgaWriter::new());
}

/// Later consider using `alloc::print` instead.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::print::_print(format_args!($($arg)*)));
}

/// Later consider using `alloc::print` instead.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints the given formatted string to the VGA text buffer through the global `VGA_OUTPUT` instance.
pub fn _print(args: fmt::Arguments) { VGA_OUTPUT.lock().write_fmt(args).unwrap(); }

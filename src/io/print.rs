use crate::io::vga_writer::VgaWriter;
use crate::utils::sync::SpinLock;
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref VGA_OUTPUT: SpinLock<VgaWriter> = SpinLock::new(VgaWriter::new());
}

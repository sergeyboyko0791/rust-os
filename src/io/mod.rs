pub mod print;
pub mod serial;
pub mod vga_writer;

#[allow(dead_code)]
pub type Result<T> = core::result::Result<T, Error>;

#[allow(dead_code)]
pub enum Error {
    InvalidData,
}

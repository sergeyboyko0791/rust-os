//! `pic` stands for `Programmable Interrupt Controller`.
//! This module is used to initialize Hardware Interrupts following the `8259 PIC` scheme.

use crate::utils::sync::SpinLock;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::InterruptDescriptorTable;

mod keyboard;
mod timer;

const PRIMARY_PIC_OFFSET: u8 = 32;
const SECONDARY_PIC_OFFSET: u8 = PRIMARY_PIC_OFFSET + 7;

lazy_static! {
    static ref PICS: SpinLock<ChainedPics> = SpinLock::new(new_pics());
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub(crate) enum HwInterruptIndex {
    Timer = PRIMARY_PIC_OFFSET,
    Keyboard,
}

impl HwInterruptIndex {
    fn as_u8(self) -> u8 { self as u8 }

    fn as_usize(self) -> usize { self.as_u8() as usize }
}

pub fn init() {
    unsafe { PICS.lock().initialize() };
    // Enable the Hardware Interrupts.
    x86_64::instructions::interrupts::enable();
}

pub fn set_handlers(idt: &mut InterruptDescriptorTable) {
    keyboard::set_handler(idt);
    timer::set_handler(idt);
}

const fn new_pics() -> ChainedPics {
    unsafe { ChainedPics::new(PRIMARY_PIC_OFFSET, SECONDARY_PIC_OFFSET) }
}

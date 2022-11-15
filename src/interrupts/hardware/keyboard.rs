//! This module has been completely copied.

use crate::interrupts::hardware::{HwInterruptIndex, PICS};
use crate::print;
use crate::utils::sync::SpinLock;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref KEYBOARD: SpinLock<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        SpinLock::new(Keyboard::new(HandleControl::Ignore));
}

pub(super) fn set_handler(idt: &mut InterruptDescriptorTable) {
    idt[HwInterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_handler);
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(HwInterruptIndex::Keyboard.as_u8());
    }
}

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_impl::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod interrupts;
pub mod io;
pub mod test_impl;
pub mod utils;

impl_test_runner!();

/// Initializes everything needed for proper kernel operation.
pub fn init() { interrupts::init(); }

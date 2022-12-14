use crate::serial_println;
use core::panic::PanicInfo;

/// Use this macro to implement a start point for the test binary.
/// It requires for the root module to add the following features:
///
/// ```ignore
/// #![cfg_attr(test, no_main)]
/// #![feature(custom_test_frameworks)]
/// #![test_runner(rust_kernel::test_impl::test_runner)]
/// #![reexport_test_harness_main = "test_main"]
/// ```
#[macro_export]
macro_rules! impl_test_runner {
    () => {
        #[cfg(test)]
        bootloader::entry_point!(test_kernel_main);

        #[cfg(test)]
        fn test_kernel_main(_boot_info: &'static bootloader::BootInfo) -> ! {
            $crate::init();
            test_main();

            $crate::utils::halt_endless_loop()
        }

        #[cfg(test)]
        #[panic_handler]
        fn panic(info: &core::panic::PanicInfo) -> ! { $crate::test_impl::test_panic_handler(info) }
    };
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_println!();
        serial_println!("{} ...", core::any::type_name::<T>());
        self();
        serial_println!("\t[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {info}\n");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

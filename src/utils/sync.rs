use core::ops::{Deref, DerefMut};
use x86_64::instructions::interrupts;

pub use spin::{Mutex as SpinLock, MutexGuard as SpinLockGuard};

/// The wrapper over `SpinLock` that ensures that there will not be interrupts handled while it's locked.
/// This structure was inspired by [`x86_64::instructions::interrupts::without_interrupts`].
pub struct SpinLockWithoutInterrupts<T> {
    spin_lock: SpinLock<T>,
}

impl<T> SpinLockWithoutInterrupts<T> {
    pub const fn new(value: T) -> Self {
        SpinLockWithoutInterrupts {
            spin_lock: SpinLock::new(value),
        }
    }

    pub fn lock(&self) -> NoInterruptsSpinGuard<'_, T> {
        // First, we need to disable the interrupts.
        let interrupts_were_enabled = interrupts::are_enabled();
        if interrupts_were_enabled {
            interrupts::disable();
        }

        // The we can lock the spin-lock.
        let guard = Some(self.spin_lock.lock());

        NoInterruptsSpinGuard {
            guard,
            interrupts_were_enabled,
        }
    }
}

pub struct NoInterruptsSpinGuard<'a, T> {
    /// The `Option` helps us to drop the `SpinLockGuard` before interrupts are enabled.
    guard: Option<SpinLockGuard<'a, T>>,
    interrupts_were_enabled: bool,
}

impl<'a, T> Deref for NoInterruptsSpinGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { self.guard.as_deref().unwrap() }
}

impl<'a, T> DerefMut for NoInterruptsSpinGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.guard.as_deref_mut().unwrap() }
}

impl<'a, T> Drop for NoInterruptsSpinGuard<'a, T> {
    fn drop(&mut self) {
        // First, we must to drop the `SpinLockGuard`.
        self.guard.take();
        // And then we can enable interrupts if it's required.
        if self.interrupts_were_enabled {
            interrupts::enable();
        }
    }
}

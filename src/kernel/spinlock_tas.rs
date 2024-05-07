use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct Mutex<T> {
    name: &'static str,
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub const fn new(value: T, name: &'static str) -> Mutex<T> {
        Mutex {
            locked: AtomicBool::new(false), // Initialize unlocked
            data: UnsafeCell::new(value),
            name,
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        loop {
            // Try to set the lock to true (acquire lock)
            if !self
                .locked
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .unwrap()
            {
                // Lock acquired
                break MutexGuard { mutex: self };
            }
            // Yield to prevent spinning
            core::hint::spin_loop();
        }
    }

    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.data.get()
    }
}

unsafe impl<T: Send> Sync for Mutex<T> {}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // Release the lock
        self.mutex.locked.store(false, Ordering::Release);
    }
}

impl<'a, T: 'a> MutexGuard<'a, T> {
    // Returns a reference to the original 'Mutex' object.
    pub fn lock(&self) -> &'a Mutex<T> {
        self.mutex
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

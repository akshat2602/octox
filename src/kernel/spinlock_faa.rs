use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Mutex<T> {
    name: &'static str,
    locked: AtomicUsize,
    data: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub const fn new(value: T, name: &'static str) -> Mutex<T> {
        Mutex {
            locked: AtomicUsize::new(0), // Initialize unlocked
            data: UnsafeCell::new(value),
            name,
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        loop {
            // Attempt to acquire the lock
            let current = self.locked.fetch_add(1, Ordering::Acquire);
            if current == 0 {
                // Lock acquired
                break MutexGuard { mutex: self };
            } else {
                // Lock is held by another thread, revert the increment
                self.locked.fetch_sub(1, Ordering::Relaxed);
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
        self.mutex.locked.fetch_sub(1, Ordering::Release);
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

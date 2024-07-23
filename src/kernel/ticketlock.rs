use crate::riscv::intr_get;

use super::proc::{Cpus, IntrLock, CPUS};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut, Drop};
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct TicketLock<T> {
    name: &'static str,       // Name of lock
    next_ticket: AtomicUsize, // Next ticket number
    now_serving: AtomicUsize, // Current ticket being served
    data: UnsafeCell<T>,      // actual data
}

#[derive(Debug)]
pub struct TicketLockGuard<'a, T: 'a> {
    mutex: &'a TicketLock<T>,
    _intr_lock: IntrLock,
    my_ticket: usize,
}

impl<T> TicketLock<T> {
    pub const fn new(value: T, name: &'static str) -> TicketLock<T> {
        TicketLock {
            next_ticket: AtomicUsize::new(0),
            now_serving: AtomicUsize::new(0),
            data: UnsafeCell::new(value),
            name,
        }
    }

    pub fn lock(&self) -> TicketLockGuard<'_, T> {
        let _intr_lock = Cpus::lock_mycpu(self.name); // disable interrupts to avoid deadlock.

        let my_ticket = self.next_ticket.fetch_add(1, Ordering::Relaxed);
        while self.now_serving.load(Ordering::Relaxed) != my_ticket {
            core::hint::spin_loop();
        }

        TicketLockGuard {
            mutex: self,
            _intr_lock,
            my_ticket,
        }
    }

    // Check whether this cpu is holding the lock.
    // Interrupts must be off.
    unsafe fn holding(&self, guard: &TicketLockGuard<'_, T>) -> bool {
        self.now_serving.load(Ordering::Relaxed) == guard.my_ticket
    }

    pub fn unlock(guard: TicketLockGuard<'_, T>) -> &'_ TicketLock<T> {
        guard.lock()
    }

    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.data.get()
    }

    // It is only safe when used in functions such as fork_ret(),
    // where passing guards is difficult.
    pub unsafe fn force_unlock(&self, guard: &TicketLockGuard<'_, T>) {
        assert!(self.holding(guard), "force unlock {}", self.name);
        self.now_serving.fetch_add(1, Ordering::Release);
        (&mut *CPUS.mycpu()).unlock()
    }
}

unsafe impl<T: Send> Sync for TicketLock<T> {}

impl<'a, T: 'a> TicketLockGuard<'a, T> {
    // Returns a reference to the original 'TicketLock' object.
    pub fn lock(&self) -> &'a TicketLock<T> {
        self.mutex
    }

    pub fn holding(&self) -> bool {
        assert!(!intr_get(), "interrupts enabled");
        unsafe { self.mutex.holding(self) }
    }
}

impl<'a, T: 'a> Drop for TicketLockGuard<'a, T> {
    fn drop(&mut self) {
        assert!(self.holding(), "release {}", self.mutex.name);
        self.mutex.now_serving.fetch_add(1, Ordering::Release);
    }
}

impl<'a, T: 'a> Deref for TicketLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T: 'a> DerefMut for TicketLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

unsafe impl<T: Sync> Sync for TicketLockGuard<'_, T> {}

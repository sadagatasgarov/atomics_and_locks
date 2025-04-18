use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::thread;
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }

    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Təhlükəsizlik: Guard mövcuddursa, deməli `lock` eksklüziv şəkildə ələ keçirilib.
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Təhlükəsizlik: Guard mövcuddursa, deməli `lock` eksklüziv şəkildə ələ keçirilib.
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

fn main() {
    let x = SpinLock::new(Vec::new());

    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });

    let g = x.lock();
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
}

#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub struct Progress {
    indeterminate: AtomicBool,
    complete: AtomicBool,
    current: AtomicU64,
    maximum: AtomicU64,
}

impl Progress {
    pub fn new(indeterminate: bool, complete: bool, current: u64, maximum: u64) -> Self {
        Self {
            indeterminate: AtomicBool::new(indeterminate),
            complete: AtomicBool::new(complete),
            current: AtomicU64::new(current),
            maximum: AtomicU64::new(maximum),
        }
    }

    // Atomic get
    pub fn indeterminate(&self) -> bool {
        self.indeterminate.load(Ordering::Acquire)
    }
    pub fn complete(&self) -> bool {
        self.complete.load(Ordering::Acquire)
    }
    pub fn current(&self) -> u64 {
        self.current.load(Ordering::Acquire)
    }
    pub fn maximum(&self) -> u64 {
        self.maximum.load(Ordering::Acquire)
    }

    // Atomic set
    pub fn set_indeterminate(&self, val: bool) {
        self.indeterminate.store(val, Ordering::Release);
    }
    pub fn set_complete(&self, val: bool) {
        self.complete.store(val, Ordering::Release);
    }
    pub fn set_current(&self, val: u64) {
        self.current.store(val, Ordering::Release);
    }
    pub fn set_maximum(&self, val: u64) {
        self.maximum.store(val, Ordering::Release);
    }

    // Atomic add
    pub fn add_current(&self, val: u64) {
        self.current.fetch_add(val, Ordering::AcqRel);
    }
    pub fn add_maximum(&self, val: u64) {
        self.maximum.fetch_add(val, Ordering::AcqRel);
    }

    pub fn percent(&self) -> f64 {
        if self.complete() {
            1.0
        } else if self.indeterminate() {
            0.0
        } else {
            let current = self.current();
            let maximum = self.maximum();

            if current == 0 || maximum == 0 {
                0.0
            } else {
                current as f64 / maximum as f64
            }
        }
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::new(true, false, 0, 0)
    }
}

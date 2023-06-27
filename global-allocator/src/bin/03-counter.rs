use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicU64, Ordering},
};

// This is our counting allocator. It wraps a u64 that stores our actual count.
pub struct Counter(AtomicU64);

impl Counter {
    // A const initializer that starts the count at 0.
    pub const fn new() -> Self {
        Counter(AtomicU64::new(0))
    }

    // Returns the current count.
    pub fn count(&self) -> u64 {
        // We're using Relaxed since there is only 1 synchronization primitive.
        self.0.load(Ordering::Relaxed)
    }
}

unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Increment our counter by 1. See other comment on Ordering.
        self.0.fetch_add(1, Ordering::Relaxed);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // No modifications here! :)
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static A: Counter = Counter::new();

fn main() {
    // Track initial count and count after allocating once.
    let count = A.count();
    let _ = Box::new(1);
    let new_count = A.count();

    // count = 3, new_count = 4.
    dbg!(count);
    dbg!(new_count);
}

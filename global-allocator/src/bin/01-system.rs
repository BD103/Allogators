// Import the System allocator.
use std::alloc::System;

// This attribute registers the global allocater.
#[global_allocator]
static A: System = System;

fn main() {
    // Let's allocate on the heap to prove that it works.
    let _ = Box::new(103);
}

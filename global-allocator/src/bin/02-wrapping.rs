use std::alloc::{GlobalAlloc, Layout, System};

// This is our custom allocator!
pub struct MyAlloc;

unsafe impl GlobalAlloc for MyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Pass everything to System.
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

// Register our custom allocator.
#[global_allocator]
static A: MyAlloc = MyAlloc;

fn main() {
    // String, like Box, also allocates on the heap.
    let _ = String::from("Boo!");
}

use std::mem::size_of;

fn main() {
    // `usize` and pointers are the same size...
    assert_eq!(size_of::<usize>(), size_of::<&u8>());
    assert_eq!(size_of::<usize>(), size_of::<*const u8>());

    // ...until they're not?!
    assert_ne!(size_of::<usize>(), size_of::<&str>());
}

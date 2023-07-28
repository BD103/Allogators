#![allow(dead_code)]
#![feature(ptr_metadata)]

use std::ptr::metadata;

fn main() {
    let slice: &[u8] = &[1, 2, 3];
    assert_eq!(metadata(slice), 3);

    let string = "Boo!";
    assert_eq!(metadata(string), 4);

    // Create a DST wrapper type.
    struct Wrapper<T: ?Sized> {
        foo: bool,
        bar: T,
    }

    // The metadata is the size of `bar`.
    let wrapper: &Wrapper<[bool]> = &Wrapper {
        foo: true,
        bar: [false, true],
    };
    assert_eq!(metadata(wrapper), 2);

    // Thin pointers have no metadata, so it returns a unit type.
    let thin: u8 = 2;
    assert_eq!(metadata(&thin), ());
}

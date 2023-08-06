fn main() {
    // Has a known size to the compiler.
    let source: [u8; 5] = [2, 4, 8, 16, 32];

    // Has an unknown size to the compiler.
    let dst: &[u8] = &source[0..3];

    // And yet we can somehow still find its size as runtime.
    assert_eq!(dst.len(), 3);
}

# Allogators

> A play on the words "Alligators" and "Allocators."

This is the source code for various project I've written in Rust related to low-level memory management.

## `global-allocator`

Examples of registering and using the [`GlobalAlloc`](https://doc.rust-lang.org/stable/std/alloc/trait.GlobalAlloc.html) trait. The companion article can be found here: [Intercepting Allocations with the Global Allocator](https://bd103.github.io/blog/2023-06-27-global-allocators). Tested using `rustc 1.72.0-nightly (101fa903b 2023-06-04)`.

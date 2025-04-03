# alias: demos of aliasing in C and Rust
Bart Massey 2025

These demos show how C doesn't check for aliasing, and how
safe Rust does at compile time, and how unsafe Rust can
unsafely subvert it.

## Build and Run

* C code: Use `make`
* Unsafe rust: Use `cargo build`
* Safe rust: Use `cargo build --features=safe`. The compile
  will fail as intended.

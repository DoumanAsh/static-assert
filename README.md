# static-assert

![](https://github.com/DoumanAsh/static-assert/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/sa.svg)](https://crates.io/crates/sa)
[![Documentation](https://docs.rs/sa/badge.svg)](https://docs.rs/crate/sa/)

Simple `static_assert` macro for compile time  assertions.

Uses `const_panic` within `const` variable to produce compile error hence only usable in `const` context

## Requirements

- Rust 1.57

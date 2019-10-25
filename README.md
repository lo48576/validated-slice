# validated-slice

[![Build Status](https://travis-ci.com/lo48576/validated-slice.svg?branch=develop)](https://travis-ci.com/lo48576/validated-slice)
[![Latest version](https://img.shields.io/crates/v/validated-slice.svg)](https://crates.io/crates/validated-slice)
[![Documentation](https://docs.rs/validated-slice/badge.svg)](https://docs.rs/validated-slice)
![Minimum rustc version: 1.37](https://img.shields.io/badge/rustc-1.37+-lightgray.svg)

Helper macros to implement std traits for custom validated slice types in Rust.

See [`tests/ascii_str.rs`](tests/ascii_str.rs) and [`tests/plain_str.rs`](tests/plain_str.rs) for
example.
These examples defines custom string types and implement basic std traits for them by this crate.

## Custom slice

To define opaque type aliases for data with specific characteristics, you may want to define custom slice types and vector types.
For example:

* String with only ASCII characters.
* String which is a valid URI.
* Escaped HTML and unescaped HTML.
* String with case-insensitive comparison by `PartialEq` and `PartialOrd`).
* Non-empty array.
* Sorted array.

However, primitive types `[T]` and `str` have many trait impls, and custom array types might be non-user-friendly without such trait impls.
(For example, if you want ASCII string `&AsciiStr`, you may also want `Default for &AsciiStr`, `std::convert::TryFrom<&str> for &AsciiStr`, `PartialEq<str> for &AsciiStr`, `PartialOrd<AsciiStr> for Cow<'_, AsciiStr>`, `std::fmt::Display for `AsciiStr`, etc.)

`validated-slice` helps users to automatically implement these traits common for array and string with less boilerplates.

## Current status

This crate is at an early stage, and experimental.
Breaking changes would be introduced for syntax and features.

This crate follows semver, so you can check crate version to know breaking change.

### Features
* nostd support
    + See docs of the macros for detail.

### TODO
For desired features without detailed ideas, see [TODO.md](TODO.md).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE.txt](LICENSE-APACHE.txt) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT.txt](LICENSE-MIT.txt) or
  <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

# validated-slice

[![Build Status](https://travis-ci.com/lo48576/validated-slice.svg?branch=develop)](https://travis-ci.com/lo48576/validated-slice)
[![Latest version](https://img.shields.io/crates/v/validated-slice.svg)](https://crates.io/crates/validated-slice)
[![Documentation](https://docs.rs/validated-slice/badge.svg)](https://docs.rs/validated-slice)
![Minimum rustc version: 1.37](https://img.shields.io/badge/rustc-1.37+-lightgray.svg)

Helper macros to implement std traits for custom validated slice types in Rust.

See [`tests/ascii_str.rs`](tests/ascii_str.rs) and [`tests/plain_str.rs`](tests/plain_str.rs) for
example.
These examples defines custom string types and implement basic std traits for them by this crate.

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

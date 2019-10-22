# Change Log

## [Unreleased]

* nostd is supported.
    + You don't need to enable or disable any features such as `std`.
      Macros and traits exported from this crate can be used both with std and without std.

### Added
* nostd support is added.
    * `impl_{std_traits,cmp}_for_{,owned}_slice!` macro now accepts optional arguments to specify
      `core` and `alloc` crate.
      If omitted, `std` is used as default for both.

## [0.1.0]

First release.

[Unreleased]: <https://github.com/lo48576/validated-slice/compare/v0.1.0...develop>
[0.1.0]: <https://github.com/lo48576/validated-slice/releases/tag/v0.1.0>

# Change Log

## [Unreleased]

* Make spec types uninhabited in tests and docs.

### Changed (non-breaking)

* Make spec types uninhabited in tests and docs.
    + Now `enum FooSpec {}` is recommended rather than `struct FooSpec;`, because the former
      prevents accidentally creating spec type value, which is meaningless.
    + `struct FooSpec(!);` would be better, but `!` (never type) is currently unstable.

## [0.1.1]

* nostd is supported.
    + You don't need to enable or disable any features such as `std`.
      Macros and traits exported from this crate can be used both with std and without std.

### Added

* nostd support is added (329bad44bfaf60fc9ca65639940d3e241dad2e48).
    * `impl_{std_traits,cmp}_for_{,owned}_slice!` macro now accepts optional arguments to specify
      `core` and `alloc` crate.
      If omitted, `std` is used as default for both.

## [0.1.0]

First release.

[Unreleased]: <https://github.com/lo48576/validated-slice/compare/v0.1.1...develop>
[0.1.1]: <https://github.com/lo48576/validated-slice/releases/tag/v0.1.1>
[0.1.0]: <https://github.com/lo48576/validated-slice/releases/tag/v0.1.0>

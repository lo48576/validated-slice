# Change Log

## [Unreleased]

## [0.2.0]

* Add support for conversion from custom type into inner type.
* Add a new trait method `into_inner()` to `OwnedSliceSpec`.
* Make spec types uninhabited in tests and docs.

### Changed (breaking)

* Add a new trait method `into_inner()` to `OwnedSliceSpec`.
    + You need to implement it. It would be quite easy, because all you have to do is to extract the
      inner field from the custom type.

### Changed (non-breaking)

* Make spec types uninhabited in tests and docs.
    + Now `enum FooSpec {}` is recommended rather than `struct FooSpec;`, because the former
      prevents accidentally creating spec type value, which is meaningless.
    + `struct FooSpec(!);` would be better, but `!` (never type) is currently unstable.

### Added

* Add support for conversion from custom type into inner type.
    + `From<&{Custom}> for &{Inner}` for borrowed slice.
    + `From<&mut {Custom}> for &mut {Inner}` for borrowed slice.
    + `From<{Custom}> for {Inner}` for owned slice.

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

[Unreleased]: <https://github.com/lo48576/validated-slice/compare/v0.2.0...develop>
[0.2.0]: <https://github.com/lo48576/validated-slice/releases/tag/v0.2.0>
[0.1.1]: <https://github.com/lo48576/validated-slice/releases/tag/v0.1.1>
[0.1.0]: <https://github.com/lo48576/validated-slice/releases/tag/v0.1.0>

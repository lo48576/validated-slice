# TODO

This is a list of (possibly) desired features, but without concrete ideas for syntax and/or behavior.

If you want to discuss them, provide some ideas, or prefer some specific syntax and usage, feel free to create an issue to the issue tracker.

## Features

### Generics support
To easily implement std traits for array types, generics (more specifically, type parameters) should be supported by `impl_std_traits_for_{,owned}_slice!` macros.

For example, think `struct SortedArray<T: ?Sized>(T);`.

```rust
// `T: Ord` is required for all operations (even for constructor), but
// it is not specified at type definition.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SortedArray<T: ?Sized>([T]);

// Require `T: Ord` here, but not `T: Default`.
impl<T: ?Sized + Ord> Default for SortedArray<T> {
    /* omitted */
}

// Require `T: Ord` and also `T: std::fmt::Debug` here.
impl<T: ?Sized + Ord> std::fmt::Debug for SortedArray<T> {
    /* omitted */
}

validated_slice::impl_std_traits_for_slice! {
    /* omitted */
}
```

Problems are:

* How it should look like to specify default (common) trait bounds, and impl-specific trait bounds?

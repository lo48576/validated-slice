//! Macros for borrowed custom slice types.

/// Implements std traits for the given custom slice type.
///
/// To implement `PartialEq` and `PartialOrd`, use [`impl_cmp_for_owned_slice!`] macro.
///
/// # Usage
///
/// ## Examples
///
/// Assume you want to implement `str` and `String` types manually by yourself.
/// Then you will have the type definitions below:
///
/// ```ignore
/// /// My `str` type.
/// #[repr(transparent)]
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// pub struct MyStr([u8]);
///
/// /// Spec for `MyStr` type.
/// struct MyStrSpec;
///
/// impl validated_slice::SliceSpec for MyStrSpec {
///     // My `str` type.
///     type Custom = MyStr;
///     // Backend type of `MyStr`.
///     type Inner = [u8];
///     // My `std::str::Utf8Error`.
///     type Error = MyUtf8Error;
///
///     /* ... and methods. */
/// }
///
/// /// My `String` type.
/// #[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// pub struct AsciiString(Vec<u8>);
///
/// /// Spec for `MyString` type.
/// struct MyStringSpec;
///
/// impl validated_slice::OwnedSliceSpec for MyStringSpec {
///     // My `String` type.
///     type Custom = MyString;
///     // Backend type of `MyString`.
///     type Inner = Vec<u8>;
///     // My `std::str::Utf8Error`.
///     type Error = MyFromUtf8Error;
///     // Spec of custom borrowed slice type, `MyStr` for this example.
///     type SliceSpec = MyStrSpec;
///     // Custom borrowed slice type.
///     // This should be same as `MyStrSpec::Custom`.
///     type SliceCustom = MyStr;
///     // Backend type of the custom borrowed slice type.
///     // This should be same as `MyStrSpec::Inner`.
///     type SliceInner = [u8];
///     // My `std::string::FromUtf8Error`.
///     // This should be same as `MyStrSpec::Error`.
///     type SliceError = MyFromUtf8Error;
///
///     /* ... and methods. */
/// }
/// ```
///
/// Then you can implement std traits as below:
///
/// ```ignore
/// validated_slice::impl_std_traits_for_owned_slice! {
///     Spec {
///         spec: MyStringSpec,
///         custom: MyString,
///         inner: Vec<u8>,
///         error: MyFromUtf8Error,
///         slice_custom: MyStr,
///         slice_inner: [u8],
///         slice_error: MyUtf8Error,
///     };
///     { AsRef<[u8]> };
///     { AsRef<str> };
///     { AsRef<{Custom}> };
///     { ToOwned<Owned = {Custom}> for {SliceCustom} };
///     { TryFrom<&{SliceInner}> };
///     { TryFrom<{Inner}> };
///     /* ... and more traits you want! */
/// }
/// ```
///
/// ## Type names
///
/// As type name, you can use `{Custom}` and `{Inner}` instead of a real type name.
/// They are replaced to the specified custom and inner types.
///
/// `Arc<ty>`, `Box<ty>`, `Cow<ty>`, and `Rc<ty>` will be also replaced to `std::sync::Arc<ty>`,
/// `std::boxed::Box<ty>`, `std::borrow::Cow<'_, ty>`, and `std::rc::Rc<ty>`, respectively.
/// They are checked symbolically, so they cannot be specified by type aliases, or
/// path names such as `std::sync::Arc<ty>`.
///
/// ## Supported trait impls
///
/// **NOTE**: To implemente `PartialEq` and `PartialOrd`, use `impl_cmp_for_owned_slice!` macro.
///
/// Each trait impl is specified by `{ TraitName<TyParams> for TyImplTarget };` format.
/// `<TyParams>` part and `for TyImplTarget` part is optional.
///
/// Default impl target is `{Custom}`, and it should NOT be specified explicitly.
/// Explicit `for {Custom}` is not supported and will cause compile error.
///
/// Supported trait impls are:
///
/// * `std::borrow`
///     + `{ Borrow<{SliceCustom}> };`
///     + `{ Borrow<any_ty> };`
///     + `{ BorrowMut<{SliceCustom}> };`
///     + `{ BorrowMut<any_ty> };`
///     + `{ ToOwned<Owned = {Custom}> for {SliceCustom} };`
/// * `std::convert`
///     + `{ AsMut<{SliceCustom}> };`
///     + `{ AsMut<any_ty> };`
///     + `{ AsRef<{SliceCustom}> };`
///     + `{ AsRef<any_ty> };`
///     + `{ From<&{SliceInner}> };`
///     + `{ From<&{SliceCustom}> };`
///     + `{ From<{Inner}> };`
///     + `{ TryFrom<&{SliceInner}> };`
///     + `{ TryFrom<{Inner}> };`
/// * `std::default`
///     + `{ Default };`
/// * `std::fmt`
///     + `{ Debug };`
///     + `{ Display };`
/// * `std::ops`
///     + `{ Deref<Target = {SliceCustom}> };`
///     + `{ DerefMut<Target = {SliceCustom}> };`
///
/// [`impl_cmp_for_owned_slice!`]: macro.impl_cmp_for_owned_slice.html
#[macro_export]
macro_rules! impl_std_traits_for_owned_slice {
    (
        Spec {
            spec: $spec:ty,
            custom: $custom:ty,
            inner: $inner:ty,
            error: $error:ty,
            slice_custom: $slice_custom:ty,
            slice_inner: $slice_inner:ty,
            slice_error: $slice_error:ty,
        };
        $({$($rest:tt)*});* $(;)?
    ) => {
        $(
            $crate::impl_std_traits_for_owned_slice! {
                @impl; ($spec, $custom, $inner, $error,
                    <$spec as $crate::OwnedSliceSpec>::SliceSpec, $slice_custom, $slice_inner,
                    $slice_error);
                rest=[$($rest)*];
            }
        )*
    };

    // std::borrow::Borrow
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ Borrow<{SliceCustom}> ];
    ) => {
        impl std::borrow::Borrow<$slice_custom> for $custom {
            #[inline]
            fn borrow(&self) -> &$slice_custom {
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured when `self` is constructed.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    $crate::impl_std_traits_for_owned_slice!(@conv:as_slice, $spec, $slice_spec, self)
                }
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ Borrow<$param:ty> ];
    ) => {
        impl std::borrow::Borrow<$param> for $custom
        where
            $slice_inner: std::borrow::Borrow<$param>,
        {
            #[inline]
            fn borrow(&self) -> &$param {
                <$spec as $crate::OwnedSliceSpec>::as_slice_inner(self).borrow()
            }
        }
    };

    // std::borrow::BorrowMut
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ BorrowMut<{SliceCustom}> ];
    ) => {
        impl std::borrow::BorrowMut<$slice_custom> for $custom {
            #[inline]
            fn borrow_mut(&mut self) -> &mut $slice_custom {
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured when `self` is constructed.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    $crate::impl_std_traits_for_owned_slice!(@conv:as_mut_slice, $spec, $slice_spec, self)
                }
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ BorrowMut<$param:ty> ];
    ) => {
        impl std::borrow::BorrowMut<$param> for $custom
        where
            $slice_inner: std::borrow::BorrowMut<$param>,
        {
            #[inline]
            fn borrow_mut(&mut self) -> &mut $param {
                <$spec as $crate::OwnedSliceSpec>::as_slice_inner_mut(self).borrow_mut()
            }
        }
    };

    // std::borrow::ToOwned
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ ToOwned<Owned = {Custom}> for {SliceCustom} ];
    ) => {
        impl std::borrow::ToOwned for $slice_custom
        where
            for<'a> $inner: From<&'a $slice_inner>,
        {
            type Owned = $custom;

            fn to_owned(&self) -> Self::Owned {
                let inner = <$inner>::from(<$slice_spec as $crate::SliceSpec>::as_inner(self));
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(self)` returns `Ok(())`.
                    //     + This is ensured when `self` is created.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    <$spec as $crate::OwnedSliceSpec>::from_inner_unchecked(inner)
                }
            }
        }
    };

    // std::convert::AsMut
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ AsMut<{SliceCustom}> ];
    ) => {
        impl std::convert::AsMut<$slice_custom> for $custom {
            #[inline]
            fn as_mut(&mut self) -> &mut $slice_custom {
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured when `self` is constructed.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    $crate::impl_std_traits_for_owned_slice!(@conv:as_mut_slice, $spec, $slice_spec, self)
                }
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ AsMut<$param:ty> ];
    ) => {
        impl std::convert::AsMut<$param> for $custom
        where
            $slice_inner: std::convert::AsMut<$param>,
        {
            #[inline]
            fn as_mut(&self) -> &$param {
                <$spec as $crate::OwnedSliceSpec>::as_slice_inner_mut(self).as_mut()
            }
        }
    };

    // std::convert::AsRef
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ AsRef<{SliceCustom}> ];
    ) => {
        impl std::convert::AsRef<$slice_custom> for $custom {
            #[inline]
            fn as_ref(&self) -> &$slice_custom {
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured when `self` is constructed.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    $crate::impl_std_traits_for_owned_slice!(@conv:as_slice, $spec, $slice_spec, self)
                }
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ AsRef<$param:ty> ];
    ) => {
        impl std::convert::AsRef<$param> for $custom
        where
            $slice_inner: std::convert::AsRef<$param>,
        {
            #[inline]
            fn as_ref(&self) -> &$param {
                <$spec as $crate::OwnedSliceSpec>::as_slice_inner(self).as_ref()
            }
        }
    };

    // std::convert::From
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ From<&{SliceInner}> ];
    ) => {
        impl<'a> std::convert::From<&'a $slice_inner> for $custom
        where
            $inner: From<&'a $slice_inner>,
        {
            fn from(s: &'a $slice_inner) -> Self {
                assert!(
                    <$slice_spec as $crate::SliceSpec>::validate(s).is_ok(),
                    "Attempt to convert invalid data: `From<&{}> for {}`",
                    stringify!($slice_inner), stringify!($custom)
                );
                let inner = <$inner>::from(s);
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by the leading assert.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    <$spec as $crate::OwnedSliceSpec>::from_inner_unchecked(inner)
                }
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ From<&{SliceCustom}> ];
    ) => {
        impl<'a> std::convert::From<&'a $slice_custom> for $custom
        where
            $inner: From<&'a $slice_inner>,
        {
            fn from(s: &'a $slice_custom) -> Self {
                let inner = <$inner>::from(<$slice_spec as $crate::SliceSpec>::as_inner(s));
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured when `s` is created.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    <$spec as $crate::OwnedSliceSpec>::from_inner_unchecked(inner)
                }
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ From<{Inner}> ];
    ) => {
        impl std::convert::From<$inner> for $custom {
            fn from(inner: $inner) -> Self {
                assert!(
                    <$slice_spec as $crate::SliceSpec>::validate(
                        <$spec as $crate::OwnedSliceSpec>::inner_as_slice_inner(&inner)
                    ).is_ok(),
                    "Attempt to convert invalid data: `From<{}> for {}`",
                    stringify!($inner), stringify!($custom)
                );
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by the leading assert.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    <$spec as $crate::OwnedSliceSpec>::from_inner_unchecked(inner)
                }
            }
        }
    };

    // std::convert::TryFrom
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ TryFrom<&{SliceInner}> ];
    ) => {
        impl<'a> std::convert::TryFrom<&'a $slice_inner> for $custom
        where
            $inner: From<&'a $slice_inner>,
        {
            type Error = $slice_error;

            fn try_from(s: &'a $slice_inner) -> std::result::Result<Self, Self::Error> {
                <$slice_spec as $crate::SliceSpec>::validate(s)?;
                let inner = <$inner>::from(s);
                Ok(unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by the leading `validate()?` call.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    <$spec as $crate::OwnedSliceSpec>::from_inner_unchecked(inner)
                })
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ TryFrom<{Inner}> ];
    ) => {
        impl std::convert::TryFrom<$inner> for $custom {
            type Error = $error;

            fn try_from(inner: $inner) -> std::result::Result<Self, Self::Error> {
                if let Err(e) = <$slice_spec as $crate::SliceSpec>::validate(
                    <$spec as $crate::OwnedSliceSpec>::inner_as_slice_inner(&inner)
                ) {
                    return Err(<$spec as $crate::OwnedSliceSpec>::convert_validation_error(e, inner));
                }
                Ok(unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by the leading `validate()?` call.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    <$spec as $crate::OwnedSliceSpec>::from_inner_unchecked(inner)
                })
            }
        }
    };

    // std::default::Default
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ Default ];
    ) => {
        impl std::default::Default for $custom
        where
            for<'a> &'a $slice_custom: std::default::Default,
            $inner: std::convert::From<$inner>,
        {
            fn default() -> Self {
                let slice = <&$slice_custom>::default();
                let slice_inner = <$slice_spec as $crate::SliceSpec>::as_inner(slice);
                let inner = <$inner>::from(slice_inner);
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by `<&$slice_custom>::default()`.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    <$spec as $crate::OwnedSliceSpec>::from_inner_unchecked(inner)
                }
            }
        }
    };

    // std::fmt::Debug
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ Debug ];
    ) => {
        impl std::fmt::Debug for $custom
        where
            $slice_custom: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let slice = unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured when `self` is created.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    $crate::impl_std_traits_for_owned_slice!(@conv:as_slice, $spec, $slice_spec, self)
                };
                <$slice_custom as std::fmt::Debug>::fmt(slice, f)
            }
        }
    };

    // std::fmt::Display
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ Display ];
    ) => {
        impl std::fmt::Display for $custom
        where
            $slice_custom: std::fmt::Display,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let slice = unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured when `self` is created.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    $crate::impl_std_traits_for_owned_slice!(@conv:as_slice, $spec, $slice_spec, self)
                };
                <$slice_custom as std::fmt::Display>::fmt(slice, f)
            }
        }
    };

    // std::ops::Deref
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ Deref<Target = {SliceCustom}> ];
    ) => {
        impl std::ops::Deref for $custom {
            type Target = $slice_custom;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured when `self` is constructed.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    $crate::impl_std_traits_for_owned_slice!(@conv:as_slice, $spec, $slice_spec, self)
                }
            }
        }
    };

    // std::ops::DerefMut
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ DerefMut<Target = {SliceCustom}> ];
    ) => {
        impl std::ops::DerefMut for $custom {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured when `self` is constructed.
                    // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
                    $crate::impl_std_traits_for_owned_slice!(@conv:as_mut_slice, $spec, $slice_spec, self)
                }
            }
        }
    };

    // Helpers.

    // Converts `&$custom` into `&$slice_custom`.
    (@conv:as_slice, $spec:ty, $slice_spec:ty, $owned_ref:expr) => {
        <$slice_spec as $crate::SliceSpec>::from_inner_unchecked(
            <$spec as $crate::OwnedSliceSpec>::as_slice_inner($owned_ref)
        )
    };
    // Converts `&mut $custom` into `&mut $slice_custom`.
    (@conv:as_mut_slice, $spec:ty, $slice_spec:ty, $owned_ref:expr) => {
        <$slice_spec as $crate::SliceSpec>::from_inner_unchecked_mut(
            <$spec as $crate::OwnedSliceSpec>::as_slice_inner_mut($owned_ref)
        )
    };

    // Fallback.
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty,
            $slice_spec:ty, $slice_custom:ty, $slice_inner:ty, $slice_error:ty);
        rest=[ $($rest:tt)* ];
    ) => {
        compile_error!(concat!("Unsupported target: ", stringify!($($rest)*)));
    };
}

/// Implements `PartialEq` and `PartialOrd` for the given custom owned slice type.
///
/// # Usage
///
/// ## Examples
///
/// ```ignore
/// validated_slice::impl_cmp_for_owned_slice! {
///     Spec {
///         spec: AsciiStringSpec,
///         custom: AsciiString,
///         inner: String,
///         slice_custom: AsciiStr,
///         slice_inner: str,
///         base: Inner,
///     };
///     Cmp { PartialEq, PartialOrd };
///     // This is same as `#[derive(PartialEq, PartialOrd)]`.
///     { ({Custom}), ({Custom}) };
///     { ({Custom}), ({SliceCustom}), rev };
///     { ({Custom}), (&{SliceCustom}), rev };
///     // NOTE: `std::borrow::Borrow for AsciiString` is required by `Cow`.
///     { ({Custom}), (Cow<{SliceCustom}>), rev };
///     /* ... and more pairs! */
/// }
/// ```
///
/// ## Comparison base
///
/// The syntax of `Spec` part is very similar to [`impl_std_traits_for_owned_slice!`] macro.
///
/// As `base` field, specify `Custom` or `Inner` to decide which comparison should be used
/// internally.
/// If you don't define custom comparison, use `base: Inner`.
///
/// ## Traits to implement
///
/// Comparison traits to implement is specified by `Cmp { .. };` format.
/// Supproted formats are: `Cmp { PartialEq }`, `Cmp { PartialOrd }`, and
/// `Cmp { PartialEq, PartialOrd };`.
///
/// ## Operand type pairs
///
/// Comparisons are implemented between two types, so you should provide list of pairs to implement
/// comparison.
///
/// Supported syntaxes are: `{ (lhs_ty), (rhs_ty) };` and `{ (lhs_ty), (rhs_ty), rev };`.
///
/// Parentheses around types are not omittable.
///
/// With `, rev`, the macro implements not only `PartialXx<rhs_ty> for lhs_ty`, but also
/// `PartialXx<lhs_ty> for rhs_ty`.
///
/// ## Type names
///
/// `{Custom}`, `{Inner}`, `{SliceCustom}`, and `{SliceInner}` will be replaced to the custom slice
/// type, its inner type, custom borrowed slice type, and its inner type.
///
/// `&ty` and `Cow<ty>` are also supported.
///
/// Note that in case you specify arbitrary types (other than `{Custom}`, `{Inner}`,
/// `{SliceCustom}`, `{SliceInner}`, and its variations), that type should implement
/// `AsRef<base_type>`.
///
/// ## Supported types
///
/// * `{Custom}`
/// * `&{Custom}`
/// * `{SliceCustom}`
/// * `&{SliceCustom}`
/// * `Cow<{SliceCustom}>`
/// * `{Inner}`
/// * `&{Inner}`
/// * `{SliceInner}`
/// * `&{SliceInner}`
/// * `Cow<{SliceInner}>`
/// * ... and arbitrary types
///
/// Note that, with `base: Custom`, `{Inner}`, `{SliceInner}` and its variants are not supported
/// (because it does not make sense).
///
/// [`impl_std_traits_for_owned_slice!`]: macro.impl_std_traits_for_owned_slice.html
#[macro_export]
macro_rules! impl_cmp_for_owned_slice {
    (
        Spec {
            spec: $spec:ty,
            custom: $custom:ty,
            inner: $inner:ty,
            slice_custom: $slice_custom:ty,
            slice_inner: $slice_inner:ty,
            base: $base:ident,
        };
        Cmp { PartialEq, PartialOrd };
        $({ ($($lhs:tt)*), ($($rhs:tt)*) $(, $($opt:ident),*)? });* $(;)?
    ) => {
        $(
            $crate::impl_cmp_for_owned_slice! {
                @impl[PartialEq]; ($spec, $custom, $inner, $slice_custom, $slice_inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
            $crate::impl_cmp_for_owned_slice! {
                @impl[PartialOrd]; ($spec, $custom, $inner, $slice_custom, $slice_inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
        )*
    };
    (
        Spec {
            spec: $spec:ty,
            custom: $custom:ty,
            inner: $inner:ty,
            slice_custom: $slice_custom:ty,
            slice_inner: $slice_inner:ty,
            base: $base:ident,
        };
        Cmp { PartialEq };
        $({ ($($lhs:tt)*), ($($rhs:tt)*) $(, $($opt:ident),*)? });* $(;)?
    ) => {
        $(
            $crate::impl_cmp_for_owned_slice! {
                @impl[PartialEq]; ($spec, $custom, $inner, $slice_custom, $slice_inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
        )*
    };
    (
        Spec {
            spec: $spec:ty,
            custom: $custom:ty,
            inner: $inner:ty,
            slice_custom: $slice_custom:ty,
            slice_inner: $slice_inner:ty,
            base: $base:ident,
        };
        Cmp { PartialOrd };
        $({ ($($lhs:tt)*), ($($rhs:tt)*) $(, $($opt:ident),*)? });* $(;)?
    ) => {
        $(
            $crate::impl_cmp_for_owned_slice! {
                @impl[PartialOrd]; ($spec, $custom, $inner, $slice_custom, $slice_inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
        )*
    };

    (
        @impl[PartialEq]; ($spec:ty, $custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*) };
    ) => {
        impl std::cmp::PartialEq<
            $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($rhs)* })
        > for $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($lhs)* })
        {
            #[inline]
            fn eq(&self, other: &$crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($rhs)* }))
                -> bool
            {
                $crate::impl_cmp_for_owned_slice!(@cmp_fn[PartialEq]; ($slice_custom, $slice_inner, $base))(
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($rhs)* }; other),
                )
            }
        }
    };
    (
        @impl[PartialEq]; ($spec:ty, $custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*), rev };
    ) => {
        impl std::cmp::PartialEq<
            $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($rhs)* })
        > for $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($lhs)* })
        {
            #[inline]
            fn eq(&self, other: &$crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($rhs)* }))
                -> bool
            {
                $crate::impl_cmp_for_owned_slice!(@cmp_fn[PartialEq]; ($slice_custom, $slice_inner, $base))(
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($rhs)* }; other),
                )
            }
        }
        impl std::cmp::PartialEq<
            $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($lhs)* })
        > for $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($rhs)* })
        {
            #[inline]
            fn eq(&self, other: &$crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($lhs)* }))
                -> bool
            {
                $crate::impl_cmp_for_owned_slice!(@cmp_fn[PartialEq]; ($slice_custom, $slice_inner, $base))(
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($rhs)* }; self),
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($lhs)* }; other),
                )
            }
        }
    };
    (
        @impl[PartialOrd]; ($spec:ty, $custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*) };
    ) => {
        impl std::cmp::PartialOrd<
            $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($rhs)* })
        > for $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($lhs)* })
        {
            #[inline]
            fn partial_cmp(&self, other: &$crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($rhs)* }))
                -> std::option::Option<std::cmp::Ordering>
            {
                $crate::impl_cmp_for_owned_slice!(@cmp_fn[PartialOrd]; ($slice_custom, $slice_inner, $base))(
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($rhs)* }; other),
                )
            }
        }
    };
    (
        @impl[PartialOrd]; ($spec:ty, $custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*), rev };
    ) => {
        impl std::cmp::PartialOrd<
            $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($rhs)* })
        > for $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($lhs)* })
        {
            #[inline]
            fn partial_cmp(&self, other: &$crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($rhs)* }))
                -> std::option::Option<std::cmp::Ordering>
            {
                $crate::impl_cmp_for_owned_slice!(@cmp_fn[PartialOrd]; ($slice_custom, $slice_inner, $base))(
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($rhs)* }; other),
                )
            }
        }
        impl std::cmp::PartialOrd<
            $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($lhs)* })
        > for $crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($rhs)* })
        {
            #[inline]
            fn partial_cmp(&self, other: &$crate::impl_cmp_for_owned_slice!(@type; ($custom, $inner, $slice_custom, $slice_inner); { $($lhs)* }))
                -> std::option::Option<std::cmp::Ordering>
            {
                $crate::impl_cmp_for_owned_slice!(@cmp_fn[PartialOrd]; ($slice_custom, $slice_inner, $base))(
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($rhs)* }; self),
                    $crate::impl_cmp_for_owned_slice!(@expr[$base]; ($spec, $slice_custom, $slice_inner); { $($lhs)* }; other),
                )
            }
        }
    };

    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { {Custom} }) => {
        $custom
    };
    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { &{Custom} }) => {
        &$custom
    };
    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { {SliceCustom} }) => {
        $slice_custom
    };
    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { &{SliceCustom} }) => {
        &$slice_custom
    };
    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { Cow<{SliceCustom}> }) => {
        std::borrow::Cow<'_, $slice_custom>
    };
    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { {Inner} }) => {
        $inner
    };
    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { &{Inner} }) => {
        &$inner
    };
    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { {SliceInner} }) => {
        $slice_inner
    };
    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { &{SliceInner} }) => {
        &$slice_inner
    };
    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { Cow<{SliceInner}> }) => {
        std::borrow::Cow<'_, $slice_inner>
    };
    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { Cow<$ty:ty> }) => { &**$ty };
    (@type; ($custom:ty, $inner:ty, $slice_custom:ty, $slice_inner:ty); { $ty:ty }) => { $ty };

    (@cmp_fn[PartialEq]; ($slice_custom:ty, $slice_inner:ty, Inner)) => {
        <$slice_inner as std::cmp::PartialEq<$slice_inner>>::eq
    };
    (@cmp_fn[PartialEq]; ($slice_custom:ty, $slice_inner:ty, Custom)) => {
        <$slice_custom as std::cmp::PartialEq<$slice_custom>>::eq
    };
    (@cmp_fn[PartialOrd]; ($slice_custom:ty, $slice_inner:ty, Inner)) => {
        <$slice_inner as std::cmp::PartialOrd<$slice_inner>>::partial_cmp
    };
    (@cmp_fn[PartialOrd]; ($slice_custom:ty, $slice_inner:ty, Custom)) => {
        <$slice_custom as std::cmp::PartialOrd<$slice_custom>>::partial_cmp
    };

    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { {Custom} }; $expr:expr) => {
        <$spec as $crate::OwnedSliceSpec>::as_slice_inner($expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { &{Custom} }; $expr:expr) => {
        <$spec as $crate::OwnedSliceSpec>::as_slice_inner(*$expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { Cow<{Custom}> }; $expr:expr) => {
        <$spec as $crate::OwnedSliceSpec>::as_slice_inner(&**$expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { {SliceCustom} }; $expr:expr) => {
        <<$spec as $crate::OwnedSliceSpec>::SliceSpec as $crate::SliceSpec>::as_inner($expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { &{SliceCustom} }; $expr:expr) => {
        <<$spec as $crate::OwnedSliceSpec>::SliceSpec as $crate::SliceSpec>::as_inner(*$expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { Cow<{SliceCustom}> }; $expr:expr) => {
        <<$spec as $crate::OwnedSliceSpec>::SliceSpec as $crate::SliceSpec>::as_inner(&**$expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { {Inner} }; $expr:expr) => {
        <$spec as $crate::OwnedSliceSpec>::inner_as_slice_inner($expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { &{Inner} }; $expr:expr) => {
        <$spec as $crate::OwnedSliceSpec>::inner_as_slice_inner(*$expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { Cow<{Inner}> }; $expr:expr) => {
        <$spec as $crate::OwnedSliceSpec>::inner_as_slice_inner(&**$expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { {SliceInner} }; $expr:expr) => {
        $expr
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { &{SliceInner} }; $expr:expr) => {
        *$expr
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { Cow<{SliceInner}> }; $expr:expr) => {
        &**$expr
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { $ty:ty }; $expr:expr) => {
        std::convert::AsRef::<$inner>::as_ref($expr)
    };

    (@expr[Custom]; ($spec:ty, $custom:ty, $inner:ty); { {Custom} }; $expr:expr) => {
        unsafe {
            // This is safe only when all of the conditions below are met:
            //
            // * `$spec::validate(s)` returns `Ok(())`.
            //     + This is ensured when `$expr` is constructed.
            // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
            <<$spec as $crate::OwnedSliceSpec>::SliceSpec as $crate::SliceSpec>::from_inner_unchecked(
                <$spec as $crate::OwnedSliceSpec>::as_slice_inner($expr)
            )
        }
    };
    (@expr[Custom]; ($spec:ty, $custom:ty, $inner:ty); { &{Custom} }; $expr:expr) => {
        unsafe {
            // This is safe only when all of the conditions below are met:
            //
            // * `$spec::validate(s)` returns `Ok(())`.
            //     + This is ensured when `$expr` is constructed.
            // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
            <<$spec as $crate::OwnedSliceSpec>::SliceSpec as $crate::SliceSpec>::from_inner_unchecked(
                <$spec as $crate::OwnedSliceSpec>::as_slice_inner(*$expr)
            )
        }
    };
    (@expr[Custom]; ($spec:ty, $custom:ty, $inner:ty); { Cow<{Custom}> }; $expr:expr) => {
        unsafe {
            // This is safe only when all of the conditions below are met:
            //
            // * `$spec::validate(s)` returns `Ok(())`.
            //     + This is ensured when `$expr` is constructed.
            // * Safety condition for `<$spec as $crate::OwnedSliceSpec>` is satisfied.
            <<$spec as $crate::OwnedSliceSpec>::SliceSpec as $crate::SliceSpec>::from_inner_unchecked(
                <$spec as $crate::OwnedSliceSpec>::as_slice_inner(&**$expr)
            )
        }
    };
    (@expr[Custom]; ($spec:ty, $custom:ty, $inner:ty); { {SliceCustom} }; $expr:expr) => {
        $expr
    };
    (@expr[Custom]; ($spec:ty, $custom:ty, $inner:ty); { &{SliceCustom} }; $expr:expr) => {
        *$expr
    };
    (@expr[Custom]; ($spec:ty, $custom:ty, $inner:ty); { Cow<{SliceCustom}> }; $expr:expr) => {
        &**$expr
    };
    (@expr[Custom]; ($spec:ty, $custom:ty, $inner:ty); { $ty:ty }; $expr:expr) => {
        std::convert::AsRef::<$custom>::as_ref($expr)
    };

    ($($rest:tt)*) => {
        compile_error!(stringify!($($rest)*));
    };
}

//! Macros for borrowed custom slice types.

/// Implements some methods of [`SliceSpec`] trait automatically.
///
/// # Examples
///
/// ```
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// struct AsciiError {
///     valid_up_to: usize,
/// }
/// pub struct AsciiStr(str);
///
/// struct AsciiStrSpec;
///
/// impl validated_slice::SliceSpec for AsciiStrSpec {
///     type Custom = AsciiStr;
///     type Inner = str;
///     type Error = AsciiError;
///
///     #[inline]
///     fn validate(s: &Self::Inner) -> Result<(), Self::Error> {
///         match s.as_bytes().iter().position(|b| !b.is_ascii()) {
///             Some(pos) => Err(AsciiError { valid_up_to: pos }),
///             None => Ok(()),
///         }
///     }
///
///     validated_slice::impl_slice_spec_methods! {
///         field=0;
///         methods=[
///             as_inner,
///             as_inner_mut,
///             from_inner_unchecked,
///             from_inner_unchecked_mut,
///         ];
///     }
/// }
/// ```
///
/// ## Field
///
/// For tuple struct, `field` is the index of the inner slice field.
/// For usual struct, `field` is the identifier of the field.
///
/// ## Methods
///
/// List methods to implement automatically.
/// `validate` is not supported and should be manually implemented by the user.
///
/// [`SliceSpec`]: trait.SliceSpec.html
#[macro_export]
macro_rules! impl_slice_spec_methods {
    (
        field=$field:tt;
        methods=[$($method:ident),* $(,)?];
    ) => {
        $(
            $crate::impl_slice_spec_methods! {
                @impl; ($field);
                $method
            }
        )*
    };
    (@impl; ($field:tt); as_inner) => {
        #[inline]
        fn as_inner(s: &Self::Custom) -> &Self::Inner {
            &s.$field
        }
    };
    (@impl; ($field:tt); as_inner_mut) => {
        #[inline]
        fn as_inner_mut(s: &mut Self::Custom) -> &mut Self::Inner {
            &mut s.$field
        }
    };
    (@impl; ($field:tt); from_inner_unchecked) => {
        #[inline]
        unsafe fn from_inner_unchecked(s: &Self::Inner) -> &Self::Custom {
            &*(s as *const Self::Inner as *const Self::Custom)
        }
    };
    (@impl; ($field:tt); from_inner_unchecked_mut) => {
        #[inline]
        unsafe fn from_inner_unchecked_mut(s: &mut Self::Inner) -> &mut Self::Custom {
            &mut *(s as *mut Self::Inner as *mut Self::Custom)
        }
    };
}

/// Implements std traits for the given custom slice type.
///
/// To implement `PartialEq` and `PartialOrd`, use [`impl_cmp_for_slice!`] macro.
///
/// # Usage
///
/// ## Examples
///
/// Assume you want to implement `str` type manually by yourself.
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
/// ```
///
/// Then you can implement std traits as below:
///
/// ```ignore
/// validated_slice::impl_std_traits_for_slice! {
///     Spec {
///         spec: MyStrSpec,
///         custom: MyStr,
///         inner: [u8],
///         error: MyUtf8Error,
///     };
///     { AsRef<[u8]> };
///     { AsRef<str> };
///     { AsRef<{Custom}> };
///     { From<&{Custom}> for Arc<{Custom}> };
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
/// **NOTE**: To implemente `PartialEq` and `PartialOrd`, use `impl_cmp_for_slice!` macro.
///
/// Each trait impl is specified by `{ TraitName<TyParams> for TyImplTarget };` format.
/// `<TyParams>` part and `for TyImplTarget` part is optional.
///
/// Default impl target is `{Custom}`, and it should NOT be specified explicitly.
/// Explicit `for {Custom}` is not supported and will cause compile error.
///
/// Supported trait impls are:
///
/// * `std::convert`
///     + `{ AsMut<{Custom}> };`
///     + `{ AsMut<any_ty> };`
///     + `{ AsRef<{Custom}> };`
///     + `{ AsRef<{Custom}> for Cow<{Custom}> };`
///     + `{ AsRef<any_ty> };`
///     + `{ AsRef<any_ty> for Cow<{Custom}> };`
///     + `{ From<&{Inner}> for &{Custom} };
///     + `{ From<&mut {Inner}> for &mut {Custom} };
///     + `{ From<&{Custom}> for Arc<{Custom}> };
///     + `{ From<&{Custom}> for Box<{Custom}> };
///     + `{ From<&{Custom}> for Rc<{Custom}> };
///     + `{ TryFrom<&{Inner}> for &{Custom} };
///     + `{ TryFrom<&mut {Inner}> for &mut {Custom} };
/// * `std::default`
///     + `{ Default for &{Custom} };`
///     + `{ Default for &mut {Custom} };`
/// * `std::fmt`
///     + `{ Debug };`
///     + `{ Display };`
/// * `std::ops`
///     + `{ Deref<Target = {Inner}> };`
///     + `{ DerefMut<Target = {Inner}> };`
///
/// [`impl_cmp_for_slice!`]: macro.impl_cmp_for_slice.html
#[macro_export]
macro_rules! impl_std_traits_for_slice {
    (
        Spec {
            spec: $spec:ty,
            custom: $custom:ty,
            inner: $inner:ty,
            error: $error:ty,
        };
        $({$($rest:tt)*});* $(;)?
    ) => {
        $(
            $crate::impl_std_traits_for_slice! {
                @impl; ($spec, $custom, $inner, $error);
                rest=[$($rest)*];
            }
        )*
    };

    // std::convert::AsMut
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsMut<{Custom}> ];
    ) => {
        impl std::convert::AsMut<$custom> for $custom {
            #[inline]
            fn as_mut(&mut self) -> &mut $custom {
                self
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsMut<$param:ty> ];
    ) => {
        impl std::convert::AsMut<$param> for $custom
        where
            $inner: AsMut<$param>,
        {
            #[inline]
            fn as_mut(&mut self) -> &mut $param {
                <$spec as $crate::SliceSpec>::as_inner_mut(self).as_mut()
            }
        }
    };

    // std::convert::AsRef
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsRef<{Custom}> ];
    ) => {
        impl std::convert::AsRef<$custom> for $custom {
            #[inline]
            fn as_ref(&self) -> &$custom {
                self
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsRef<{Custom}> for Cow<{Custom}> ];
    ) => {
        impl<'a> std::convert::AsRef<$custom> for std::borrow::Cow<'a, $custom> {
            #[inline]
            fn as_ref(&self) -> &$custom {
                &**self
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsRef<$param:ty> ];
    ) => {
        impl std::convert::AsRef<$param> for $custom
        where
            $inner: AsRef<$param>,
        {
            #[inline]
            fn as_ref(&self) -> &$param {
                <$spec as $crate::SliceSpec>::as_inner(self).as_ref()
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsRef<$param:ty> for Cow<{Custom}> ];
    ) => {
        impl<'a> std::convert::AsRef<$param> for std::borrow::Cow<'a, $custom>
        where
            $inner: AsRef<$param>,
        {
            #[inline]
            fn as_ref(&self) -> &$param {
                <$spec as $crate::SliceSpec>::as_inner(&**self).as_ref()
            }
        }
    };

    // std::convert::From
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&{Inner}> for &{Custom} ];
    ) => {
        impl<'a> std::convert::From<&'a $inner> for &'a $custom {
            fn from(s: &'a $inner) -> Self {
                assert!(
                    <$spec as $crate::SliceSpec>::validate(s).is_ok(),
                    "Attempt to convert invalid data: `From<&{}> for &{}`",
                    stringify!($inner), stringify!($custom)
                );
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by the leading assert.
                    // * Safety condition for `<$spec as $crate::SliceSpec>` is satisfied.
                    <$spec as $crate::SliceSpec>::from_inner_unchecked(s)
                }
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&mut {Inner}> for &mut {Custom} ];
    ) => {
        impl<'a> std::convert::From<&'a mut $inner> for &'a mut $custom {
            fn from(s: &'a mut $inner) -> Self {
                assert!(
                    <$spec as $crate::SliceSpec>::validate(s).is_ok(),
                    "Attempt to convert invalid data: `From<&mut {}> for &mut {}`",
                    stringify!($inner), stringify!($custom)
                );
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by the leading assert.
                    // * Safety condition for `<$spec as $crate::SliceSpec>` is satisfied.
                    <$spec as $crate::SliceSpec>::from_inner_unchecked_mut(s)
                }
            }
        }
    };

    // std::convert::From for smart pointers
    (
        @impl [smartptr]; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&{Custom}> for $($smartptr:ident)::* <{Custom}> ];
    ) => {
        impl<'a> std::convert::From<&'a $custom> for $($smartptr)::* <$custom>
        where
            $($smartptr)::* <$inner>: std::convert::From<&'a $inner>,
        {
            fn from(s: &'a $custom) -> Self {
                let inner = <$spec as $crate::SliceSpec>::as_inner(s);
                let buf = $($smartptr)::* ::<$inner>::from(inner);
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by the leading assert.
                    // * Safety condition for `<$spec as $crate::SliceSpec>` is satisfied.
                    //     + This ensures that the memory layout of `into_raw(buf)` is also valid
                    //       as `$($smartptr)::* <$custom>`.
                    $($smartptr)::* ::<$custom>::from_raw(
                        $($smartptr)::* ::<$inner>::into_raw(buf) as *mut $custom
                    )
                }
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&{Custom}> for Arc<{Custom}> ];
    ) => {
        $crate::impl_std_traits_for_slice! {
            @impl [smartptr]; ($spec, $custom, $inner, $error);
            rest=[ From<&{Custom}> for std::sync::Arc <{Custom}> ];
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&{Custom}> for Box<{Custom}> ];
    ) => {
        $crate::impl_std_traits_for_slice! {
            @impl [smartptr]; ($spec, $custom, $inner, $error);
            rest=[ From<&{Custom}> for std::boxed::Box <{Custom}> ];
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&{Custom}> for Rc<{Custom}> ];
    ) => {
        $crate::impl_std_traits_for_slice! {
            @impl [smartptr]; ($spec, $custom, $inner, $error);
            rest=[ From<&{Custom}> for std::rc::Rc <{Custom}> ];
        }
    };

    // std::convert::TryFrom
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ TryFrom<&{Inner}> for &{Custom} ];
    ) => {
        impl<'a> std::convert::TryFrom<&'a $inner> for &'a $custom {
            type Error = $error;

            fn try_from(s: &'a $inner) -> std::result::Result<Self, Self::Error> {
                <$spec as $crate::SliceSpec>::validate(s)?;
                Ok(unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by the leading `validate()?` call.
                    // * Safety condition for `<$spec as $crate::SliceSpec>` is satisfied.
                    <$spec as $crate::SliceSpec>::from_inner_unchecked(s)
                })
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ TryFrom<&mut {Inner}> for &mut {Custom} ];
    ) => {
        impl<'a> std::convert::TryFrom<&'a mut $inner> for &'a mut $custom {
            type Error = $error;

            fn try_from(s: &'a mut $inner) -> std::result::Result<Self, Self::Error> {
                <$spec as $crate::SliceSpec>::validate(s)?;
                Ok(unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by the leading `validate()?` call.
                    // * Safety condition for `<$spec as $crate::SliceSpec>` is satisfied.
                    <$spec as $crate::SliceSpec>::from_inner_unchecked_mut(s)
                })
            }
        }
    };

    // std::default::Default
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ Default for &{Custom} ];
    ) => {
        impl<'a> std::default::Default for &'a $custom
        where
            &'a $inner: std::default::Default,
        {
            fn default() -> Self {
                let inner = <&'a $inner as std::default::Default>::default();
                assert!(
                    <$spec as $crate::SliceSpec>::validate(inner).is_ok(),
                    "Attempt to create invalid data: `Default for &{}`",
                    stringify!($custom)
                );
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by the leading assert.
                    // * Safety condition for `<$spec as $crate::SliceSpec>` is satisfied.
                    <$spec as $crate::SliceSpec>::from_inner_unchecked(inner)
                }
            }
        }
    };
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ Default for &mut {Custom} ];
    ) => {
        impl<'a> std::default::Default for &'a mut $custom
        where
            &'a mut $inner: std::default::Default,
        {
            fn default() -> Self {
                let inner = <&'a mut $inner as std::default::Default>::default();
                assert!(
                    <$spec as $crate::SliceSpec>::validate(inner).is_ok(),
                    "Attempt to create invalid data: `Default for &{}`",
                    stringify!($custom)
                );
                unsafe {
                    // This is safe only when all of the conditions below are met:
                    //
                    // * `$spec::validate(s)` returns `Ok(())`.
                    //     + This is ensured by the leading assert.
                    // * Safety condition for `<$spec as $crate::SliceSpec>` is satisfied.
                    <$spec as $crate::SliceSpec>::from_inner_unchecked_mut(inner)
                }
            }
        }
    };

    // std::fmt::Debug
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ Debug ];
    ) => {
        impl std::fmt::Debug for $custom
        where
            $inner: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let inner = <$spec as $crate::SliceSpec>::as_inner(self);
                <$inner as std::fmt::Debug>::fmt(inner, f)
            }
        }
    };

    // std::fmt::Display
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ Display ];
    ) => {
        impl std::fmt::Display for $custom
        where
            $inner: std::fmt::Display,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let inner = <$spec as $crate::SliceSpec>::as_inner(self);
                <$inner as std::fmt::Display>::fmt(inner, f)
            }
        }
    };

    // std::ops::Deref
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ Deref<Target = {Inner}> ];
    ) => {
        impl std::ops::Deref for $custom {
            type Target = $inner;

            #[inline]
            fn deref(&self) -> &Self::Target {
                <$spec as $crate::SliceSpec>::as_inner(self)
            }
        }
    };

    // std::ops::DerefMut
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ DerefMut<Target = {Inner}> ];
    ) => {
        impl std::ops::DerefMut for $custom {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                <$spec as $crate::SliceSpec>::as_inner_mut(self)
            }
        }
    };

    // Fallback.
    (
        @impl; ($spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ $($rest:tt)* ];
    ) => {
        compile_error!(concat!("Unsupported target: ", stringify!($($rest)*)));
    };
}

/// Implements `PartialEq` and `PartialOrd` for the given custom slice type.
///
/// # Usage
///
/// ## Examples
///
/// ```ignore
/// validated_slice::impl_cmp_for_slice! {
///     Spec {
///         spec: AsciiStrSpec,
///         custom: AsciiStr,
///         inner: str,
///         base: Inner,
///     };
///     Cmp { PartialEq, PartialOrd };
///     // This is same as `#[derive(PartialEq, PartialOrd)]`.
///     { ({Custom}), ({Custom}) };
///     { ({Custom}), (&{Custom}), rev };
///     // NOTE: `std::borrow::ToOwned for AsciiStr` is required by `Cow`.
///     { ({Custom}), (Cow<{Custom}>), rev };
///
///     { ({Custom}), ({Inner}), rev };
///     { ({Custom}), (&{Inner}), rev };
///     /* ... and more pairs! */
/// }
/// ```
///
/// ## Comparison base
///
/// The syntax of `Spec` part is very similar to [`impl_std_traits_for_slice!`] macro.
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
/// `{Custom}` and `{Inner}` will be replaced to the custom slice type and its inner type.
///
/// `&ty` and `Cow<ty>` are also supported.
///
/// Note that in case you specify arbitrary types (other than `{Custom}`, `{Inner}`, and its
/// variations), that type should implement `AsRef<base_type>`.
///
/// ## Supported types
///
/// * `{Custom}`
/// * `&{Custom}`
/// * `Cow<{Custom}>`
/// * `{Inner}`
/// * `&{Inner}`
/// * `Cow<{Inner}>`
/// * ... and arbitrary types
///
/// Note that, with `base: Custom`, `{Inner}` and its variants are not supported (because it does
/// not make sense).
///
/// [`impl_std_traits_for_slice!`]: macro.impl_std_traits_for_slice.html
#[macro_export]
macro_rules! impl_cmp_for_slice {
    (
        Spec {
            spec: $spec:ty,
            custom: $custom:ty,
            inner: $inner:ty,
            base: $base:ident,
        };
        Cmp { PartialEq, PartialOrd };
        $({ ($($lhs:tt)*), ($($rhs:tt)*) $(, $($opt:ident),*)? });* $(;)?
    ) => {
        $(
            $crate::impl_cmp_for_slice! {
                @impl[PartialEq]; ($spec, $custom, $inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
            $crate::impl_cmp_for_slice! {
                @impl[PartialOrd]; ($spec, $custom, $inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
        )*
    };
    (
        Spec {
            spec: $spec:ty,
            custom: $custom:ty,
            inner: $inner:ty,
            base: $base:ident,
        };
        Cmp { PartialEq };
        $({ ($($lhs:tt)*), ($($rhs:tt)*) $(, $($opt:ident),*)? });* $(;)?
    ) => {
        $(
            $crate::impl_cmp_for_slice! {
                @impl[PartialEq]; ($spec, $custom, $inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
        )*
    };
    (
        Spec {
            spec: $spec:ty,
            custom: $custom:ty,
            inner: $inner:ty,
            base: $base:ident,
        };
        Cmp { PartialOrd };
        $({ ($($lhs:tt)*), ($($rhs:tt)*) $(, $($opt:ident),*)? });* $(;)?
    ) => {
        $(
            $crate::impl_cmp_for_slice! {
                @impl[PartialOrd]; ($spec, $custom, $inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
        )*
    };

    (
        @impl[PartialEq]; ($spec:ty, $custom:ty, $inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*) };
    ) => {
        impl std::cmp::PartialEq<
            $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($rhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($lhs)* })
        {
            #[inline]
            fn eq(&self, other: &$crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($rhs)* })) -> bool {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialEq]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($rhs)* }; other),
                )
            }
        }
    };
    (
        @impl[PartialEq]; ($spec:ty, $custom:ty, $inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*), rev };
    ) => {
        impl std::cmp::PartialEq<
            $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($rhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($lhs)* })
        {
            #[inline]
            fn eq(&self, other: &$crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($rhs)* })) -> bool {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialEq]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($rhs)* }; other),
                )
            }
        }
        impl std::cmp::PartialEq<
            $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($lhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($rhs)* })
        {
            #[inline]
            fn eq(&self, other: &$crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($lhs)* })) -> bool {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialEq]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($rhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($lhs)* }; other),
                )
            }
        }
    };
    (
        @impl[PartialOrd]; ($spec:ty, $custom:ty, $inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*) };
    ) => {
        impl std::cmp::PartialOrd<
            $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($rhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($lhs)* })
        {
            #[inline]
            fn partial_cmp(&self, other: &$crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($rhs)* }))
                -> std::option::Option<std::cmp::Ordering>
            {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialOrd]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($rhs)* }; other),
                )
            }
        }
    };
    (
        @impl[PartialOrd]; ($spec:ty, $custom:ty, $inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*), rev };
    ) => {
        impl std::cmp::PartialOrd<
            $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($rhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($lhs)* })
        {
            #[inline]
            fn partial_cmp(&self, other: &$crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($rhs)* }))
                -> std::option::Option<std::cmp::Ordering>
            {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialOrd]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($rhs)* }; other),
                )
            }
        }
        impl std::cmp::PartialOrd<
            $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($lhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($rhs)* })
        {
            #[inline]
            fn partial_cmp(&self, other: &$crate::impl_cmp_for_slice!(@type; ($custom, $inner); { $($lhs)* }))
                -> std::option::Option<std::cmp::Ordering>
            {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialOrd]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($rhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ($spec, $custom, $inner); { $($lhs)* }; other),
                )
            }
        }
    };

    (@type; ($custom:ty, $inner:ty); { {Custom} }) => { $custom };
    (@type; ($custom:ty, $inner:ty); { &{Custom} }) => { &$custom };
    (@type; ($custom:ty, $inner:ty); { Cow<{Custom}> }) => { std::borrow::Cow<'_, $custom> };
    (@type; ($custom:ty, $inner:ty); { {Inner} }) => { $inner };
    (@type; ($custom:ty, $inner:ty); { &{Inner} }) => { &$inner };
    (@type; ($custom:ty, $inner:ty); { Cow<{Inner}> }) => { std::borrow::Cow<'_, $inner> };
    (@type; ($custom:ty, $inner:ty); { $ty:ty }) => { $ty };

    (@cmp_fn[PartialEq]; ($custom:ty, $inner:ty, Inner)) => { <$inner as std::cmp::PartialEq<$inner>>::eq };
    (@cmp_fn[PartialEq]; ($custom:ty, $inner:ty, Custom)) => { <$custom as std::cmp::PartialEq<$custom>>::eq };
    (@cmp_fn[PartialOrd]; ($custom:ty, $inner:ty, Inner)) => { <$inner as std::cmp::PartialOrd<$inner>>::partial_cmp };
    (@cmp_fn[PartialOrd]; ($custom:ty, $inner:ty, Custom)) => { <$custom as std::cmp::PartialOrd<$custom>>::partial_cmp };

    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { {Custom} }; $expr:expr) => {
        <$spec as $crate::SliceSpec>::as_inner($expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { &{Custom} }; $expr:expr) => {
        <$spec as $crate::SliceSpec>::as_inner(*$expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { Cow<{Custom}> }; $expr:expr) => {
        <$spec as $crate::SliceSpec>::as_inner(&**$expr)
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { {Inner} }; $expr:expr) => {
        $expr
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { &{Inner} }; $expr:expr) => {
        *$expr
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { Cow<{Inner}> }; $expr:expr) => {
        &**$expr
    };
    (@expr[Inner]; ($spec:ty, $custom:ty, $inner:ty); { $ty:ty }; $expr:expr) => {
        std::convert::AsRef::<$inner>::as_ref($expr)
    };
    (@expr[Custom]; ($spec:ty, $custom:ty, $inner:ty); { {Custom} }; $expr:expr) => {
        $expr
    };
    (@expr[Custom]; ($spec:ty, $custom:ty, $inner:ty); { &{Custom} }; $expr:expr) => {
        *$expr
    };
    (@expr[Custom]; ($spec:ty, $custom:ty, $inner:ty); { Cow<{Custom}> }; $expr:expr) => {
        &**$expr
    };
    (@expr[Custom]; ($spec:ty, $custom:ty, $inner:ty); { $ty:ty }; $expr:expr) => {
        std::convert::AsRef::<$custom>::as_ref($expr)
    };

    ($($rest:tt)*) => {
        compile_error!(stringify!($($rest)*));
    };
}

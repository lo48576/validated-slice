//! Macros for borrowed custom slice types.

/// Implements some methods of [`SliceSpec`] trait automatically.
///
/// This macro can be safely used in nostd environment.
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
/// enum AsciiStrSpec {}
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
/// ```
/// /// My `str` type.
/// #[repr(transparent)]
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// pub struct MyStr([u8]);
///
/// /// Spec for `MyStr` type.
/// enum MyStrSpec {}
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
/// #     fn validate(s: &Self::Inner) -> Result<(), Self::Error> {
/// #         Ok(())
/// #     }
/// #     validated_slice::impl_slice_spec_methods! {
/// #         field=0;
/// #         methods=[
/// #             as_inner,
/// #             as_inner_mut,
/// #             from_inner_unchecked,
/// #             from_inner_unchecked_mut,
/// #         ];
/// #     }
/// }
/// # struct MyUtf8Error;
/// ```
///
/// Then you can implement std traits as below:
///
/// ```
/// # use std as alloc;
/// # /// My `str` type.
/// # #[repr(transparent)]
/// # #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// # pub struct MyStr([u8]);
/// #
/// # /// Spec for `MyStr` type.
/// # enum MyStrSpec {}
/// #
/// # impl validated_slice::SliceSpec for MyStrSpec {
/// #     // My `str` type.
/// #     type Custom = MyStr;
/// #     // Backend type of `MyStr`.
/// #     type Inner = [u8];
/// #     // My `std::str::Utf8Error`.
/// #     type Error = MyUtf8Error;
/// #
/// #     /* ... and methods. */
/// #     fn validate(s: &Self::Inner) -> Result<(), Self::Error> {
/// #         Ok(())
/// #     }
/// #     validated_slice::impl_slice_spec_methods! {
/// #         field=0;
/// #         methods=[
/// #             as_inner,
/// #             as_inner_mut,
/// #             from_inner_unchecked,
/// #             from_inner_unchecked_mut,
/// #         ];
/// #     }
/// }
/// # struct MyUtf8Error;
/// validated_slice::impl_std_traits_for_slice! {
///     // `Std` is omissible.
///     Std {
///         // Module identifier of `core` crate.
///         // Default is `std`.
///         core: core,
///         // Module identifier of `alloc` crate.
///         // Default is `std`.
///         alloc: alloc,
///     };
///     Spec {
///         spec: MyStrSpec,
///         custom: MyStr,
///         inner: [u8],
///         error: MyUtf8Error,
///     };
///     { AsRef<[u8]> };
///     { AsRef<{Custom}> };
///     { From<&{Custom}> for Arc<{Custom}> };
///     /* ... and more traits you want! */
/// }
/// ```
///
/// ## Core and alloc
///
/// For `no_std` use, the macro uses custom `core` and `alloc` crate if given.
/// You can support both nostd and non-nostd environment as below:
///
/// ```ignore
/// // Use `std` when available.
/// #[cfg(feature = "std")]
/// use alloc as std;
/// // Use external `alloc` crate when nostd.
/// #[cfg(not(feature = "std"))]
/// use alloc;
///
/// validated_slice::impl_std_traits_for_slice! {
///     Std {
///         core: core,
///         alloc: alloc,
///     };
///     Spec { /* ... */ };
///     /* ... */
/// }
/// ```
///
/// When you don't need `alloc` crate on nostd build, value of `alloc` field is not used.
/// Simply specify `alloc: alloc,` or something.
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
///     + `{ From<&{Custom}> for &{Inner} };
///     + `{ From<&mut {Custom}> for &mut {Inner} };
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
                @impl; ({std, std}, $spec, $custom, $inner, $error);
                rest=[$($rest)*];
            }
        )*
    };

    (
        Std {
            core: $core:ident,
            alloc: $alloc:ident,
        };
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
                @impl; ({$core, $alloc}, $spec, $custom, $inner, $error);
                rest=[$($rest)*];
            }
        )*
    };

    // std::convert::AsMut
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsMut<{Custom}> ];
    ) => {
        impl $core::convert::AsMut<$custom> for $custom {
            #[inline]
            fn as_mut(&mut self) -> &mut $custom {
                self
            }
        }
    };
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsMut<$param:ty> ];
    ) => {
        impl $core::convert::AsMut<$param> for $custom
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
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsRef<{Custom}> ];
    ) => {
        impl $core::convert::AsRef<$custom> for $custom {
            #[inline]
            fn as_ref(&self) -> &$custom {
                self
            }
        }
    };
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsRef<{Custom}> for Cow<{Custom}> ];
    ) => {
        impl<'a> $core::convert::AsRef<$custom> for $alloc::borrow::Cow<'a, $custom> {
            #[inline]
            fn as_ref(&self) -> &$custom {
                &**self
            }
        }
    };
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsRef<$param:ty> ];
    ) => {
        impl $core::convert::AsRef<$param> for $custom
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
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ AsRef<$param:ty> for Cow<{Custom}> ];
    ) => {
        impl<'a> $core::convert::AsRef<$param> for $alloc::borrow::Cow<'a, $custom>
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
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&{Inner}> for &{Custom} ];
    ) => {
        impl<'a> $core::convert::From<&'a $inner> for &'a $custom {
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
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&mut {Inner}> for &mut {Custom} ];
    ) => {
        impl<'a> $core::convert::From<&'a mut $inner> for &'a mut $custom {
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
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&{Custom}> for &{Inner} ];
    ) => {
        impl<'a> $core::convert::From<&'a $custom> for &'a $inner {
            #[inline]
            fn from(s: &'a $custom) -> Self {
                <$spec as $crate::SliceSpec>::as_inner(s)
            }
        }
    };
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&mut {Custom}> for &mut {Inner} ];
    ) => {
        impl<'a> $core::convert::From<&'a mut $custom> for &'a mut $inner {
            #[inline]
            fn from(s: &'a mut $custom) -> Self {
                <$spec as $crate::SliceSpec>::as_inner_mut(s)
            }
        }
    };

    // std::convert::From for smart pointers
    (
        @impl [smartptr]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty, $mut:ident);
        rest=[ From<&{Custom}> for $($smartptr:ident)::* <{Custom}> ];
    ) => {
        impl<'a> $core::convert::From<&'a $custom> for $($smartptr)::* <$custom>
        where
            $($smartptr)::* <$inner>: $core::convert::From<&'a $inner>,
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
                        $($smartptr)::* ::<$inner>::into_raw(buf) as *$mut $custom
                    )
                }
            }
        }
    };
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&{Custom}> for Arc<{Custom}> ];
    ) => {
        $crate::impl_std_traits_for_slice! {
            @impl [smartptr]; ({$core, $alloc}, $spec, $custom, $inner, $error, const);
            rest=[ From<&{Custom}> for $alloc::sync::Arc <{Custom}> ];
        }
    };
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&{Custom}> for Box<{Custom}> ];
    ) => {
        $crate::impl_std_traits_for_slice! {
            @impl [smartptr]; ({$core, $alloc}, $spec, $custom, $inner, $error, mut);
            rest=[ From<&{Custom}> for $alloc::boxed::Box <{Custom}> ];
        }
    };
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ From<&{Custom}> for Rc<{Custom}> ];
    ) => {
        $crate::impl_std_traits_for_slice! {
            @impl [smartptr]; ({$core, $alloc}, $spec, $custom, $inner, $error, const);
            rest=[ From<&{Custom}> for $alloc::rc::Rc <{Custom}> ];
        }
    };

    // std::convert::TryFrom
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ TryFrom<&{Inner}> for &{Custom} ];
    ) => {
        impl<'a> $core::convert::TryFrom<&'a $inner> for &'a $custom {
            type Error = $error;

            fn try_from(s: &'a $inner) -> $core::result::Result<Self, Self::Error> {
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
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ TryFrom<&mut {Inner}> for &mut {Custom} ];
    ) => {
        impl<'a> $core::convert::TryFrom<&'a mut $inner> for &'a mut $custom {
            type Error = $error;

            fn try_from(s: &'a mut $inner) -> $core::result::Result<Self, Self::Error> {
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
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ Default for &{Custom} ];
    ) => {
        impl<'a> $core::default::Default for &'a $custom
        where
            &'a $inner: $core::default::Default,
        {
            fn default() -> Self {
                let inner = <&'a $inner as $core::default::Default>::default();
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
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ Default for &mut {Custom} ];
    ) => {
        impl<'a> $core::default::Default for &'a mut $custom
        where
            &'a mut $inner: $core::default::Default,
        {
            fn default() -> Self {
                let inner = <&'a mut $inner as $core::default::Default>::default();
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
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ Debug ];
    ) => {
        impl $core::fmt::Debug for $custom
        where
            $inner: $core::fmt::Debug,
        {
            #[inline]
            fn fmt(&self, f: &mut $core::fmt::Formatter<'_>) -> $core::fmt::Result {
                let inner = <$spec as $crate::SliceSpec>::as_inner(self);
                <$inner as $core::fmt::Debug>::fmt(inner, f)
            }
        }
    };

    // std::fmt::Display
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ Display ];
    ) => {
        impl $core::fmt::Display for $custom
        where
            $inner: $core::fmt::Display,
        {
            #[inline]
            fn fmt(&self, f: &mut $core::fmt::Formatter<'_>) -> $core::fmt::Result {
                let inner = <$spec as $crate::SliceSpec>::as_inner(self);
                <$inner as $core::fmt::Display>::fmt(inner, f)
            }
        }
    };

    // std::ops::Deref
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ Deref<Target = {Inner}> ];
    ) => {
        impl $core::ops::Deref for $custom {
            type Target = $inner;

            #[inline]
            fn deref(&self) -> &Self::Target {
                <$spec as $crate::SliceSpec>::as_inner(self)
            }
        }
    };

    // std::ops::DerefMut
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
        rest=[ DerefMut<Target = {Inner}> ];
    ) => {
        impl $core::ops::DerefMut for $custom {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                <$spec as $crate::SliceSpec>::as_inner_mut(self)
            }
        }
    };

    // Fallback.
    (
        @impl; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $error:ty);
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
///     // `Std` is omissible.
///     Std {
///         // Module identifier of `core` crate.
///         // Default is `std`.
///         core: core,
///         // Module identifier of `alloc` crate.
///         // Default is `std`.
///         alloc: alloc,
///     };
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
/// ## Core and alloc
///
/// For `no_std` use, the macro uses custom `core` and `alloc` crate if given.
/// You can support both nostd and non-nostd environment as below:
///
/// ```ignore
/// // Use `std` when available.
/// #[cfg(feature = "std")]
/// use alloc as std;
/// // Use external `alloc` crate when nostd.
/// #[cfg(not(feature = "std"))]
/// use alloc;
///
/// validated_slice::impl_cmp_for_slice! {
///     Std {
///         core: core,
///         alloc: alloc,
///     }
///     Spec { /* ... */ };
///     Cmp { /* ... */ };
///     /* ... */
/// }
/// ```
///
/// When you don't need `alloc` crate on nostd build, value of `alloc` field is not used.
/// Simply specify `alloc: alloc,` or something.
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
        Cmp { $($cmp_targets:ident),* };
        $($rest:tt)*
    ) => {
        $crate::impl_cmp_for_slice! {
            @full;
            Std {
                core: std,
                alloc: std,
            };
            Spec {
                spec: $spec,
                custom: $custom,
                inner: $inner,
                base: $base,
            };
            Cmp { $($cmp_targets),* };
            $($rest)*
        }
    };
    (
        Std {
            core: $core:ident,
            alloc: $alloc:ident,
        };
        Spec {
            spec: $spec:ty,
            custom: $custom:ty,
            inner: $inner:ty,
            base: $base:ident,
        };
        Cmp { $($cmp_targets:ident),* };
        $($rest:tt)*
    ) => {
        $crate::impl_cmp_for_slice! {
            @full;
            Std {
                core: $core,
                alloc: $alloc,
            };
            Spec {
                spec: $spec,
                custom: $custom,
                inner: $inner,
                base: $base,
            };
            Cmp { $($cmp_targets),* };
            $($rest)*
        }
    };

    (
        @full;
        Std {
            core: $core:ident,
            alloc: $alloc:ident,
        };
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
                @impl[PartialEq]; ({$core, $alloc}, $spec, $custom, $inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
            $crate::impl_cmp_for_slice! {
                @impl[PartialOrd]; ({$core, $alloc}, $spec, $custom, $inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
        )*
    };
    (
        @full;
        Std {
            core: $core:ident,
            alloc: $alloc:ident,
        };
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
                @impl[PartialEq]; ({$core, $alloc}, $spec, $custom, $inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
        )*
    };
    (
        @full;
        Std {
            core: $core:ident,
            alloc: $alloc:ident,
        };
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
                @impl[PartialOrd]; ({$core, $alloc}, $spec, $custom, $inner, $base);
                { ($($lhs)*), ($($rhs)*) $(, $($opt),*)? };
            }
        )*
    };

    (
        @impl[PartialEq]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*) };
    ) => {
        impl $core::cmp::PartialEq<
            $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($rhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($lhs)* })
        {
            #[inline]
            fn eq(&self, other: &$crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($rhs)* })) -> bool {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialEq]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($rhs)* }; other),
                )
            }
        }
    };
    (
        @impl[PartialEq]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*), rev };
    ) => {
        impl $core::cmp::PartialEq<
            $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($rhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($lhs)* })
        {
            #[inline]
            fn eq(&self, other: &$crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($rhs)* })) -> bool {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialEq]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($rhs)* }; other),
                )
            }
        }
        impl $core::cmp::PartialEq<
            $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($lhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($rhs)* })
        {
            #[inline]
            fn eq(&self, other: &$crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($lhs)* })) -> bool {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialEq]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($rhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($lhs)* }; other),
                )
            }
        }
    };
    (
        @impl[PartialOrd]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*) };
    ) => {
        impl $core::cmp::PartialOrd<
            $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($rhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($lhs)* })
        {
            #[inline]
            fn partial_cmp(&self, other: &$crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($rhs)* }))
                -> $core::option::Option<$core::cmp::Ordering>
            {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialOrd]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($rhs)* }; other),
                )
            }
        }
    };
    (
        @impl[PartialOrd]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty, $base:ident);
        { ($($lhs:tt)*), ($($rhs:tt)*), rev };
    ) => {
        impl $core::cmp::PartialOrd<
            $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($rhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($lhs)* })
        {
            #[inline]
            fn partial_cmp(&self, other: &$crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($rhs)* }))
                -> $core::option::Option<$core::cmp::Ordering>
            {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialOrd]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($lhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($rhs)* }; other),
                )
            }
        }
        impl $core::cmp::PartialOrd<
            $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($lhs)* })
        > for $crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($rhs)* })
        {
            #[inline]
            fn partial_cmp(&self, other: &$crate::impl_cmp_for_slice!(@type; ({$core, $alloc}, $custom, $inner); { $($lhs)* }))
                -> $core::option::Option<$core::cmp::Ordering>
            {
                $crate::impl_cmp_for_slice!(@cmp_fn[PartialOrd]; ($custom, $inner, $base))(
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($rhs)* }; self),
                    $crate::impl_cmp_for_slice!(@expr[$base]; ({$core, $alloc}, $spec, $custom, $inner); { $($lhs)* }; other),
                )
            }
        }
    };

    (@type; ({$core:ident, $alloc:ident}, $custom:ty, $inner:ty); { {Custom} }) => { $custom };
    (@type; ({$core:ident, $alloc:ident}, $custom:ty, $inner:ty); { &{Custom} }) => { &$custom };
    (@type; ({$core:ident, $alloc:ident}, $custom:ty, $inner:ty); { Cow<{Custom}> }) => { $alloc::borrow::Cow<'_, $custom> };
    (@type; ({$core:ident, $alloc:ident}, $custom:ty, $inner:ty); { {Inner} }) => { $inner };
    (@type; ({$core:ident, $alloc:ident}, $custom:ty, $inner:ty); { &{Inner} }) => { &$inner };
    (@type; ({$core:ident, $alloc:ident}, $custom:ty, $inner:ty); { Cow<{Inner}> }) => { $alloc::borrow::Cow<'_, $inner> };
    (@type; ({$core:ident, $alloc:ident}, $custom:ty, $inner:ty); { $ty:ty }) => { $ty };

    (@cmp_fn[PartialEq]; ($custom:ty, $inner:ty, Inner)) => { <$inner as core::cmp::PartialEq<$inner>>::eq };
    (@cmp_fn[PartialEq]; ($custom:ty, $inner:ty, Custom)) => { <$custom as core::cmp::PartialEq<$custom>>::eq };
    (@cmp_fn[PartialOrd]; ($custom:ty, $inner:ty, Inner)) => { <$inner as core::cmp::PartialOrd<$inner>>::partial_cmp };
    (@cmp_fn[PartialOrd]; ($custom:ty, $inner:ty, Custom)) => { <$custom as core::cmp::PartialOrd<$custom>>::partial_cmp };

    (@expr[Inner]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty); { {Custom} }; $expr:expr) => {
        <$spec as $crate::SliceSpec>::as_inner($expr)
    };
    (@expr[Inner]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty); { &{Custom} }; $expr:expr) => {
        <$spec as $crate::SliceSpec>::as_inner(*$expr)
    };
    (@expr[Inner]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty); { Cow<{Custom}> }; $expr:expr) => {
        <$spec as $crate::SliceSpec>::as_inner(&**$expr)
    };
    (@expr[Inner]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty); { {Inner} }; $expr:expr) => {
        $expr
    };
    (@expr[Inner]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty); { &{Inner} }; $expr:expr) => {
        *$expr
    };
    (@expr[Inner]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty); { Cow<{Inner}> }; $expr:expr) => {
        &**$expr
    };
    (@expr[Inner]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty); { $ty:ty }; $expr:expr) => {
        $core::convert::AsRef::<$inner>::as_ref($expr)
    };
    (@expr[Custom]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty); { {Custom} }; $expr:expr) => {
        $expr
    };
    (@expr[Custom]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty); { &{Custom} }; $expr:expr) => {
        *$expr
    };
    (@expr[Custom]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty); { Cow<{Custom}> }; $expr:expr) => {
        &**$expr
    };
    (@expr[Custom]; ({$core:ident, $alloc:ident}, $spec:ty, $custom:ty, $inner:ty); { $ty:ty }; $expr:expr) => {
        $core::convert::AsRef::<$custom>::as_ref($expr)
    };

    ($($rest:tt)*) => {
        compile_error!(stringify!($($rest)*));
    };
}

//! A library to easily define validated custom slice and vector types.
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

#[macro_use]
mod macros;

/// A trait to provide types and features for a custom slice type.
///
/// # Safety
///
/// To avoid undefined behavior, users are responsible to let implementations satisfy all
/// conditions below:
///
/// * `Self::validate()` always returns the same result for the same input.
/// * `Self::Inner` is the only non-zero type field of `Self::Custom`.
/// * `Self::Custom` has attribute `#[repr(transparent)]` or `#[repr(C)]`.
///
/// If any of the condition is not met, use of methods may cause undefined behavior.
///
/// # Examples
///
/// ```
/// /// ASCII string slice.
/// // `#[repr(transparent)]` or `#[repr(C)]` is required.
/// // Without it, generated codes would be unsound.
/// #[repr(transparent)]
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// pub struct AsciiStr(str);
///
/// /// ASCII string validation error.
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// pub struct AsciiError {
///     /// Byte position of the first invalid byte.
///     valid_up_to: usize,
/// }
///
/// enum AsciiStrSpec {}
///
/// impl validated_slice::SliceSpec for AsciiStrSpec {
///     type Custom = AsciiStr;
///     type Inner = str;
///     type Error = AsciiError;
///
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
pub trait SliceSpec {
    /// Custom borrowed slice type.
    type Custom: ?Sized;
    /// Borrowed inner slice type of `Self::Custom`.
    type Inner: ?Sized;
    /// Validation error type.
    type Error;

    /// Validates the inner slice to check if the value is valid as the custom slice type value.
    ///
    /// Returns `Ok(())` if the value is valid (and safely convertible to `Self::Custom`.
    /// Returns `Err(_)` if the validation failed.
    fn validate(s: &Self::Inner) -> Result<(), Self::Error>;
    /// Converts a reference to the custom slice into a reference to the inner slice type.
    fn as_inner(s: &Self::Custom) -> &Self::Inner;
    /// Converts a mutable reference to the custom slice into a mutable reference to the inner slice
    /// type.
    fn as_inner_mut(s: &mut Self::Custom) -> &mut Self::Inner;
    /// Creates a reference to the custom slice type without any validation.
    ///
    /// # Safety
    ///
    /// This is safe only when all of the conditions below are met:
    ///
    /// * `Self::validate(s)` returns `Ok(())`.
    /// * `Self::Inner` is the only non-zero type field of `Self::Custom`.
    /// * `Self::Custom` has attribute `#[repr(transparent)]` or `#[repr(C)]`.
    ///
    /// If any of the condition is not met, this function may cause undefined behavior.
    unsafe fn from_inner_unchecked(s: &Self::Inner) -> &Self::Custom;
    /// Creates a mutable reference to the custom slice type without any validation.
    ///
    /// # Safety
    ///
    /// Safety condition is same as [`from_inner_unchecked`].
    ///
    /// [`from_inner_unchecked`]: #tymethod.from_inner_unchecked
    unsafe fn from_inner_unchecked_mut(s: &mut Self::Inner) -> &mut Self::Custom;
}

/// A trait to provide types and features for an owned custom slice type.
///
/// # Safety
///
/// To avoid undefined behavior, users are responsible to let implementations satisfy all
/// conditions below:
///
/// * Safety conditions for `Self::SliceSpec` is satisfied.
/// * `Self::SliceCustom` is set to `<Self::SliceSpec as SliceSpec>::Custom`.
/// * `Self::SliceInner` is set to `<Self::SliceSpec as SliceSpec>::Inner`.
/// * `Self::SliceError` is set to `<Self::SliceSpec as SliceSpec>::Error`.
///
/// If any of the conditions is not met, use of methods may cause undefined behavior.
///
/// # Examples
///
/// ```
/// # /// ASCII string slice.
/// # // `#[repr(transparent)]` or `#[repr(C)]` is required.
/// # // Without it, generated codes would be unsound.
/// # #[repr(transparent)]
/// # #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// # pub struct AsciiStr(str);
/// #
/// # /// ASCII string validation error.
/// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// # pub struct AsciiError {
/// #     /// Byte position of the first invalid byte.
/// #     valid_up_to: usize,
/// # }
/// #
/// # enum AsciiStrSpec {}
/// #
/// # impl validated_slice::SliceSpec for AsciiStrSpec {
/// #     type Custom = AsciiStr;
/// #     type Inner = str;
/// #     type Error = AsciiError;
/// #
/// #     fn validate(s: &Self::Inner) -> Result<(), Self::Error> {
/// #         match s.as_bytes().iter().position(|b| !b.is_ascii()) {
/// #             Some(pos) => Err(AsciiError { valid_up_to: pos }),
/// #             None => Ok(()),
/// #         }
/// #     }
/// #
/// #     validated_slice::impl_slice_spec_methods! {
/// #         field=0;
/// #         methods=[
/// #             as_inner,
/// #             as_inner_mut,
/// #             from_inner_unchecked,
/// #             from_inner_unchecked_mut,
/// #         ];
/// #     }
/// # }
/// /// ASCII string boxed slice.
/// #[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// pub struct AsciiString(String);
///
/// enum AsciiStringSpec {}
///
/// impl validated_slice::OwnedSliceSpec for AsciiStringSpec {
///     type Custom = AsciiString;
///     type Inner = String;
///     // You can use dedicated error type for owned slice here,
///     // as `std::str::Utf8Error` is used for borrowed slice validation and
///     // `std::string::FromUtf8Error` is used for owned slice validation.
///     type Error = AsciiError;
///     type SliceSpec = AsciiStrSpec;
///     type SliceCustom = AsciiStr;
///     type SliceInner = str;
///     type SliceError = AsciiError;
///
///     #[inline]
///     fn convert_validation_error(e: Self::SliceError, _: Self::Inner) -> Self::Error {
///         e
///     }
///
///     #[inline]
///     fn as_slice_inner(s: &Self::Custom) -> &Self::SliceInner {
///         &s.0
///     }
///
///     #[inline]
///     fn as_slice_inner_mut(s: &mut Self::Custom) -> &mut Self::SliceInner {
///         &mut s.0
///     }
///
///     #[inline]
///     fn inner_as_slice_inner(s: &Self::Inner) -> &Self::SliceInner {
///         s
///     }
///
///     #[inline]
///     unsafe fn from_inner_unchecked(s: Self::Inner) -> Self::Custom {
///         AsciiString(s)
///     }
///
///     #[inline]
///     fn into_inner(s: Self::Custom) -> Self::Inner {
///         s.0
///     }
/// }
/// ```
pub trait OwnedSliceSpec {
    /// Custom owned slice type.
    type Custom;
    /// Owned inner slice type of `Self::Custom`.
    type Inner;
    /// Validation error type for owned inner type.
    type Error;
    /// Spec of the borrowed slice type.
    type SliceSpec: SliceSpec;
    /// Same type as `<Self::SliceSpec as SliceSpec>::Custom`.
    type SliceCustom: ?Sized;
    /// Same type as `<Self::SliceSpec as SliceSpec>::Inner`.
    type SliceInner: ?Sized;
    /// Same type as `<Self::SliceSpec as SliceSpec>::Error`.
    type SliceError;

    /// Converts a borrowed slice validation error into an owned slice validation error.
    fn convert_validation_error(e: Self::SliceError, v: Self::Inner) -> Self::Error;
    /// Returns the borrowed inner slice for the given reference to a custom owned slice.
    fn as_slice_inner(s: &Self::Custom) -> &Self::SliceInner;
    /// Returns the borrowed inner slice for the given mutable reference to a custom owned slice.
    fn as_slice_inner_mut(s: &mut Self::Custom) -> &mut Self::SliceInner;
    /// Returns the borrowed inner slice for the given reference to owned inner slice.
    fn inner_as_slice_inner(s: &Self::Inner) -> &Self::SliceInner;
    /// Creates a reference to the custom slice type without any validation.
    ///
    /// # Safety
    ///
    /// This is safe only when all of the conditions below are met:
    ///
    /// * `Self::validate(s)` returns `Ok(())`.
    /// * Safety condition for `Self::SliceSpec` is satisfied.
    ///
    /// If any of the condition is not met, this function may cause undefined behavior.
    unsafe fn from_inner_unchecked(s: Self::Inner) -> Self::Custom;
    /// Returns the inner value with its ownership.
    fn into_inner(s: Self::Custom) -> Self::Inner;
}

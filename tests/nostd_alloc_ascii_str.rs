//! ASCII string in (pseudo) nostd and alloc environment.
//!
//! Types for strings which consists of only ASCII characters.

use std as alloc;

enum AsciiStrSpec {}

impl validated_slice::SliceSpec for AsciiStrSpec {
    type Custom = AsciiStr;
    type Inner = str;
    type Error = AsciiError;

    fn validate(s: &Self::Inner) -> Result<(), Self::Error> {
        match s.as_bytes().iter().position(|b| !b.is_ascii()) {
            Some(pos) => Err(AsciiError { valid_up_to: pos }),
            None => Ok(()),
        }
    }

    validated_slice::impl_slice_spec_methods! {
        field=0;
        methods=[
            as_inner,
            as_inner_mut,
            from_inner_unchecked,
            from_inner_unchecked_mut,
        ];
    }
}

/// ASCII string validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AsciiError {
    /// Byte position of the first invalid byte.
    valid_up_to: usize,
}

/// ASCII string slice.
// `#[repr(transparent)]` or `#[repr(C)]` is required.
// Without it, generated codes would be unsound.
//
// You can use `#[derive(Debug, PartialEq, PartialOrd)]` here, but in this example they are
// implemented by macros in `validated_slice`, or implemented manually.
#[repr(transparent)]
#[derive(Eq, Ord, Hash)]
pub struct AsciiStr(str);

impl core::fmt::Debug for AsciiStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Ascii({:?})", &self.0)
    }
}

validated_slice::impl_std_traits_for_slice! {
    Std {
        core: core,
        alloc: alloc,
    };
    Spec {
        spec: AsciiStrSpec,
        custom: AsciiStr,
        inner: str,
        error: AsciiError,
    };
    // AsRef<[u8]> for AsciiStr
    { AsRef<[u8]> };
    // AsRef<str> for AsciiStr
    { AsRef<str> };
    // AsRef<AsciiStr> for AsciiStr
    { AsRef<{Custom}> };
    // From<&'_ AsciiStr> for &'_ str
    { From<&{Custom}> for &{Inner} };
    // From<&'_ AsciiStr> for Arc<AsciiStr>
    { From<&{Custom}> for Arc<{Custom}> };
    // From<&'_ AsciiStr> for Box<AsciiStr>
    { From<&{Custom}> for Box<{Custom}> };
    // From<&'_ AsciiStr> for Rc<AsciiStr>
    { From<&{Custom}> for Rc<{Custom}> };
    // TryFrom<&'_ str> for &'_ AsciiStr
    { TryFrom<&{Inner}> for &{Custom} };
    // TryFrom<&'_ mut str> for &'_ mut AsciiStr
    { TryFrom<&mut {Inner}> for &mut {Custom} };
    // Default for &'_ AsciiStr
    { Default for &{Custom} };
    // Default for &'_ mut AsciiStr
    { Default for &mut {Custom} };
    // Display for AsciiStr
    { Display };
    // Deref<Target = str> for Custom
    { Deref<Target = {Inner}> };
}

validated_slice::impl_cmp_for_slice! {
    Std {
        core: core,
        alloc: alloc,
    };
    Spec {
        spec: AsciiStrSpec,
        custom: AsciiStr,
        inner: str,
        base: Inner,
    };
    Cmp { PartialEq, PartialOrd };
    // This is same as `#[derive(PartialEq, PartialOrd)]`.
    { ({Custom}), ({Custom}) };
    { ({Custom}), (&{Custom}), rev };
    // NOTE: This requires `alloc::borrow::ToOwned for AsciiStr`.
    { ({Custom}), (Cow<{Custom}>), rev };

    { ({Custom}), ({Inner}), rev };
    { ({Custom}), (&{Inner}), rev };
    { (&{Custom}), ({Inner}), rev };
    { ({Custom}), (Cow<{Inner}>), rev };
    { (&{Custom}), (Cow<{Inner}>), rev };
    // NOTE: `{Inner}` should be local type to implement this.
    //{ ({Inner}), (Cow<{Custom}>), rev };
    // NOTE: `{Inner}` should be local type to implement this.
    //{ (&{Inner}), (Cow<{Custom}>), rev };
}

enum AsciiBoxStrSpec {}

impl validated_slice::OwnedSliceSpec for AsciiBoxStrSpec {
    type Custom = AsciiBoxStr;
    type Inner = Box<str>;
    type Error = AsciiError;
    type SliceSpec = AsciiStrSpec;
    type SliceCustom = AsciiStr;
    type SliceInner = str;
    type SliceError = AsciiError;

    #[inline]
    fn convert_validation_error(e: Self::SliceError, _: Self::Inner) -> Self::Error {
        e
    }

    #[inline]
    fn as_slice_inner(s: &Self::Custom) -> &Self::SliceInner {
        &s.0
    }

    #[inline]
    fn as_slice_inner_mut(s: &mut Self::Custom) -> &mut Self::SliceInner {
        &mut s.0
    }

    #[inline]
    fn inner_as_slice_inner(s: &Self::Inner) -> &Self::SliceInner {
        s
    }

    #[inline]
    unsafe fn from_inner_unchecked(s: Self::Inner) -> Self::Custom {
        AsciiBoxStr(s)
    }
}

/// ASCII string boxed slice.
#[derive(Default, Clone, Eq, Ord, Hash)]
pub struct AsciiBoxStr(Box<str>);

impl From<AsciiString> for AsciiBoxStr {
    fn from(s: AsciiString) -> Self {
        Self(s.0.into_boxed_str())
    }
}

validated_slice::impl_std_traits_for_owned_slice! {
    Std {
        core: core,
        alloc: alloc,
    };
    Spec {
        spec: AsciiBoxStrSpec,
        custom: AsciiBoxStr,
        inner: Box<str>,
        error: AsciiError,
        slice_custom: AsciiStr,
        slice_inner: str,
        slice_error: AsciiError,
    };
    // AsMut<str> for AsciiBoxStr
    // NOTE: `AsMut<[u8]> for str` is not implemented.
    //{ AsMut<str> };
    // AsMut<AsciiStr> for AsciiBoxStr
    { AsMut<{SliceCustom}> };
    // AsRef<[u8]> for AsciiBoxStr
    { AsRef<[u8]> };
    // AsRef<str> for AsciiBoxStr
    { AsRef<str> };
    // AsRef<AsciiStr> for AsciiBoxStr
    { AsRef<{SliceCustom}> };
    // Borrow<[u8]> for AsciiBoxStr
    // NOTE: `Borrow<[u8]> for str` is not implemented.
    //{ Borrow<[u8]> };
    // Borrow<str> for AsciiBoxStr
    { Borrow<str> };
    // Borrow<AsciiStr> for AsciiBoxStr
    { Borrow<{SliceCustom}> };
    // BorrowMut<AsciiStr> for AsciiBoxStr
    { BorrowMut<{SliceCustom}> };
    // ToOwned<Owned = AsciiBoxStr> for AsciiStr
    //{ ToOwned<Owned = {Custom}> for {SliceCustom} };
    // From<&'_ AsciiStr> for AsciiBoxStr
    { From<&{SliceCustom}> };
    // TryFrom<&'_ str> for AsciiBoxStr
    { TryFrom<&{SliceInner}> };
    // TryFrom<Box<str>> for AsciiBoxStr
    { TryFrom<{Inner}> };
    // Default for AsciiBoxStr
    // NOTE: Same as `#[derive(Default)]` in this case.
    //{ Default };
    // Debug for AsciiBoxStr
    { Debug };
    // Display for AsciiBoxStr
    { Display };
    // Deref<Target = AsciiStr> for AsciiBoxStr
    { Deref<Target = {SliceCustom}> };
    // DerefMut<Target = AsciiStr> for AsciiBoxStr
    { DerefMut<Target = {SliceCustom}> };
    // FromStr<Err = AsciiError> for AsciiBoxStr
    { FromStr };
}

validated_slice::impl_cmp_for_owned_slice! {
    Std {
        core: core,
        alloc: alloc,
    };
    Spec {
        spec: AsciiBoxStrSpec,
        custom: AsciiBoxStr,
        inner: Box<str>,
        slice_custom: AsciiStr,
        slice_inner: str,
        base: Inner,
    };
    Cmp { PartialEq, PartialOrd };
    // { lhs, rhs }.
    { ({Custom}), ({Custom}) };
    { ({Custom}), ({SliceCustom}), rev };
    { ({Custom}), (&{SliceCustom}), rev };
    //// NOTE: This requires `core::borrow::Borrow for AsciiBoxStr`.
    { ({Custom}), (Cow<{SliceCustom}>), rev };
    { ({Custom}), ({Inner}), rev };
    { ({Custom}), ({SliceInner}), rev };
    { ({Custom}), (&{SliceInner}), rev };
    { ({Custom}), (Cow<{SliceInner}>), rev };
    { ({Inner}), ({SliceCustom}), rev };
    { ({Inner}), (&{SliceCustom}), rev };
}

enum AsciiStringSpec {}

impl validated_slice::OwnedSliceSpec for AsciiStringSpec {
    type Custom = AsciiString;
    type Inner = String;
    type Error = AsciiError;
    type SliceSpec = AsciiStrSpec;
    type SliceCustom = AsciiStr;
    type SliceInner = str;
    type SliceError = AsciiError;

    #[inline]
    fn convert_validation_error(e: Self::SliceError, _: Self::Inner) -> Self::Error {
        e
    }

    #[inline]
    fn as_slice_inner(s: &Self::Custom) -> &Self::SliceInner {
        &s.0
    }

    #[inline]
    fn as_slice_inner_mut(s: &mut Self::Custom) -> &mut Self::SliceInner {
        &mut s.0
    }

    #[inline]
    fn inner_as_slice_inner(s: &Self::Inner) -> &Self::SliceInner {
        s
    }

    #[inline]
    unsafe fn from_inner_unchecked(s: Self::Inner) -> Self::Custom {
        AsciiString(s)
    }
}

/// ASCII string boxed slice.
#[derive(Default, Clone, Eq, Ord, Hash)]
pub struct AsciiString(String);

impl From<AsciiBoxStr> for AsciiString {
    fn from(s: AsciiBoxStr) -> Self {
        Self(s.0.into())
    }
}

validated_slice::impl_std_traits_for_owned_slice! {
    Std {
        core: core,
        alloc: alloc,
    };
    Spec {
        spec: AsciiStringSpec,
        custom: AsciiString,
        inner: String,
        error: AsciiError,
        slice_custom: AsciiStr,
        slice_inner: str,
        slice_error: AsciiError,
    };
    // AsMut<str> for AsciiString
    // NOTE: `AsMut<[u8]> for str` is not implemented.
    //{ AsMut<str> };
    // AsMut<AsciiStr> for AsciiString
    { AsMut<{SliceCustom}> };
    // AsRef<[u8]> for AsciiString
    { AsRef<[u8]> };
    // AsRef<str> for AsciiString
    { AsRef<str> };
    // AsRef<AsciiStr> for AsciiString
    { AsRef<{SliceCustom}> };
    // Borrow<[u8]> for AsciiString
    // NOTE: `Borrow<[u8]> for str` is not implemented.
    //{ Borrow<[u8]> };
    // Borrow<str> for AsciiString
    { Borrow<str> };
    // Borrow<AsciiStr> for AsciiString
    { Borrow<{SliceCustom}> };
    // BorrowMut<AsciiStr> for AsciiString
    { BorrowMut<{SliceCustom}> };
    // ToOwned<Owned = AsciiString> for AsciiStr
    { ToOwned<Owned = {Custom}> for {SliceCustom} };
    // From<&'_ AsciiStr> for AsciiString
    { From<&{SliceCustom}> };
    // TryFrom<&'_ str> for AsciiString
    { TryFrom<&{SliceInner}> };
    // TryFrom<String> for AsciiString
    { TryFrom<{Inner}> };
    // Default for AsciiString
    // NOTE: Same as `#[derive(Default)]` in this case.
    //{ Default };
    // Debug for AsciiString
    { Debug };
    // Display for AsciiString
    { Display };
    // Deref<Target = AsciiStr> for AsciiString
    { Deref<Target = {SliceCustom}> };
    // DerefMut<Target = AsciiStr> for AsciiString
    { DerefMut<Target = {SliceCustom}> };
    // FromStr<Err = AsciiError> for AsciiString
    { FromStr };
}

validated_slice::impl_cmp_for_owned_slice! {
    Std {
        core: core,
        alloc: alloc,
    };
    Spec {
        spec: AsciiStringSpec,
        custom: AsciiString,
        inner: String,
        slice_custom: AsciiStr,
        slice_inner: str,
        base: Inner,
    };
    Cmp { PartialEq, PartialOrd };
    // { lhs, rhs }.
    { ({Custom}), ({Custom}) };
    { ({Custom}), ({SliceCustom}), rev };
    { ({Custom}), (&{SliceCustom}), rev };
    //// NOTE: This requires `core::borrow::Borrow for AsciiString`.
    { ({Custom}), (Cow<{SliceCustom}>), rev };
    { ({Custom}), ({Inner}), rev };
    { ({Custom}), ({SliceInner}), rev };
    { ({Custom}), (&{SliceInner}), rev };
    { ({Custom}), (Cow<{SliceInner}>), rev };
    { ({Inner}), ({SliceCustom}), rev };
    { ({Inner}), (&{SliceCustom}), rev };
}

#[cfg(test)]
mod ascii_str {
    use super::*;

    #[test]
    fn as_ref()
    where
        AsciiStr: AsRef<[u8]>,
        AsciiStr: AsRef<str>,
        AsciiStr: AsRef<AsciiStr>,
    {
    }

    #[test]
    fn partial_eq_custom()
    where
        AsciiStr: PartialEq<AsciiStr>,
        for<'a> AsciiStr: PartialEq<&'a AsciiStr>,
        for<'a> &'a AsciiStr: PartialEq<AsciiStr>,
        for<'a> AsciiStr: PartialEq<alloc::borrow::Cow<'a, AsciiStr>>,
        for<'a> alloc::borrow::Cow<'a, AsciiStr>: PartialEq<alloc::borrow::Cow<'a, AsciiStr>>,
    {
    }

    #[test]
    fn partial_eq_inner()
    where
        AsciiStr: PartialEq<str>,
        str: PartialEq<AsciiStr>,
        for<'a> AsciiStr: PartialEq<&'a str>,
        for<'a> &'a str: PartialEq<AsciiStr>,
        for<'a> &'a AsciiStr: PartialEq<str>,
        for<'a> str: PartialEq<&'a AsciiStr>,
        for<'a> AsciiStr: PartialEq<alloc::borrow::Cow<'a, str>>,
        for<'a> alloc::borrow::Cow<'a, str>: PartialEq<AsciiStr>,
        for<'a, 'b> &'b AsciiStr: PartialEq<alloc::borrow::Cow<'a, str>>,
        for<'a, 'b> alloc::borrow::Cow<'a, str>: PartialEq<&'b AsciiStr>,
    {
    }

    #[test]
    fn from()
    where
        for<'a> &'a str: From<&'a AsciiStr>,
    {
    }

    #[test]
    fn from_smart_ptr()
    where
        for<'a> alloc::sync::Arc<AsciiStr>: From<&'a AsciiStr>,
        for<'a> alloc::boxed::Box<AsciiStr>: From<&'a AsciiStr>,
        for<'a> alloc::rc::Rc<AsciiStr>: From<&'a AsciiStr>,
    {
    }

    #[test]
    fn try_from()
    where
        for<'a> &'a AsciiStr: core::convert::TryFrom<&'a str>,
        for<'a> &'a mut AsciiStr: core::convert::TryFrom<&'a mut str>,
    {
    }

    #[test]
    fn default()
    where
        for<'a> &'a AsciiStr: Default,
        for<'a> &'a mut AsciiStr: Default,
    {
    }

    #[test]
    fn fmt()
    where
        AsciiStr: core::fmt::Debug,
        AsciiStr: core::fmt::Display,
    {
        use core::convert::TryFrom;

        let sample_raw = "text";
        let sample_ascii = <&AsciiStr>::try_from(sample_raw).expect("Should never fail");
        assert_eq!(format!("{:?}", sample_ascii), "Ascii(\"text\")");
        assert_eq!(format!("{}", sample_ascii), sample_raw);
    }

    #[test]
    fn deref()
    where
        AsciiStr: core::ops::Deref<Target = str>,
    {
    }
}

#[cfg(test)]
mod ascii_box_str {
    use super::*;

    #[test]
    fn as_ref()
    where
        AsciiBoxStr: AsRef<[u8]>,
        AsciiBoxStr: AsRef<str>,
        AsciiBoxStr: AsRef<AsciiStr>,
        AsciiBoxStr: AsMut<AsciiStr>,
    {
    }

    #[test]
    fn borrow()
    where
        AsciiBoxStr: core::borrow::Borrow<str>,
        AsciiBoxStr: core::borrow::Borrow<AsciiStr>,
    {
    }

    #[test]
    fn borrow_mut()
    where
        AsciiBoxStr: core::borrow::BorrowMut<AsciiStr>,
    {
    }

    #[test]
    fn partial_eq_custom()
    where
        AsciiBoxStr: PartialEq<AsciiBoxStr>,
        AsciiBoxStr: PartialEq<AsciiStr>,
        AsciiStr: PartialEq<AsciiBoxStr>,
        for<'a> AsciiBoxStr: PartialEq<&'a AsciiStr>,
        for<'a> &'a AsciiStr: PartialEq<AsciiBoxStr>,
        for<'a> AsciiBoxStr: PartialEq<alloc::borrow::Cow<'a, AsciiStr>>,
        for<'a> alloc::borrow::Cow<'a, AsciiStr>: PartialEq<AsciiBoxStr>,
    {
    }

    #[test]
    fn partial_eq_inner()
    where
        AsciiBoxStr: PartialEq<str>,
        str: PartialEq<AsciiBoxStr>,
        for<'a> AsciiBoxStr: PartialEq<&'a str>,
        for<'a> &'a str: PartialEq<AsciiBoxStr>,
        for<'a> AsciiBoxStr: PartialEq<alloc::borrow::Cow<'a, str>>,
        for<'a> alloc::borrow::Cow<'a, str>: PartialEq<AsciiBoxStr>,
        Box<str>: PartialEq<AsciiStr>,
        AsciiStr: PartialEq<Box<str>>,
        for<'a> Box<str>: PartialEq<&'a AsciiStr>,
        for<'a> &'a AsciiStr: PartialEq<Box<str>>,
    {
    }

    #[test]
    fn partial_ord_custom()
    where
        AsciiBoxStr: PartialOrd<AsciiBoxStr>,
        AsciiBoxStr: PartialOrd<AsciiStr>,
        AsciiStr: PartialOrd<AsciiBoxStr>,
        for<'a> AsciiBoxStr: PartialOrd<&'a AsciiStr>,
        for<'a> &'a AsciiStr: PartialOrd<AsciiBoxStr>,
        for<'a> AsciiBoxStr: PartialOrd<alloc::borrow::Cow<'a, AsciiStr>>,
        for<'a> alloc::borrow::Cow<'a, AsciiStr>: PartialOrd<AsciiBoxStr>,
    {
    }

    #[test]
    fn partial_ord_inner()
    where
        AsciiBoxStr: PartialOrd<str>,
        str: PartialOrd<AsciiBoxStr>,
        for<'a> AsciiBoxStr: PartialOrd<&'a str>,
        for<'a> &'a str: PartialOrd<AsciiBoxStr>,
        for<'a> AsciiBoxStr: PartialOrd<alloc::borrow::Cow<'a, str>>,
        for<'a> alloc::borrow::Cow<'a, str>: PartialOrd<AsciiBoxStr>,
        Box<str>: PartialOrd<AsciiStr>,
        AsciiStr: PartialOrd<Box<str>>,
        for<'a> Box<str>: PartialOrd<&'a AsciiStr>,
        for<'a> &'a AsciiStr: PartialOrd<Box<str>>,
    {
    }

    #[test]
    fn from()
    where
        for<'a> AsciiBoxStr: From<&'a AsciiStr>,
    {
    }

    #[test]
    fn try_from()
    where
        for<'a> AsciiBoxStr: core::convert::TryFrom<&'a str>,
        AsciiBoxStr: core::convert::TryFrom<Box<str>>,
    {
    }

    #[test]
    fn fmt()
    where
        AsciiBoxStr: core::fmt::Debug,
        AsciiBoxStr: core::fmt::Display,
    {
        use core::convert::TryFrom;

        let sample_raw = "text";
        let sample_ascii = AsciiBoxStr::try_from(sample_raw).expect("Should never fail");
        assert_eq!(format!("{:?}", sample_ascii), "Ascii(\"text\")");
        assert_eq!(format!("{}", sample_ascii), sample_raw);
        let sample_ascii_borrowed = <&AsciiStr>::try_from(sample_raw).expect("Should never fail");
        assert_eq!(
            format!("{:?}", sample_ascii),
            format!("{:?}", sample_ascii_borrowed)
        );
        assert_eq!(
            format!("{}", sample_ascii),
            format!("{}", sample_ascii_borrowed)
        );
    }

    #[test]
    fn deref()
    where
        AsciiBoxStr: core::ops::Deref<Target = AsciiStr>,
    {
    }

    #[test]
    fn deref_mut()
    where
        AsciiBoxStr: core::ops::DerefMut<Target = AsciiStr>,
    {
    }

    #[test]
    fn from_str()
    where
        AsciiBoxStr: core::str::FromStr<Err = AsciiError>,
    {
    }
}

#[cfg(test)]
mod ascii_string {
    use super::*;

    #[test]
    fn as_ref()
    where
        AsciiString: AsRef<[u8]>,
        AsciiString: AsRef<str>,
        AsciiString: AsRef<AsciiStr>,
        AsciiString: AsMut<AsciiStr>,
    {
    }

    #[test]
    fn borrow()
    where
        AsciiString: core::borrow::Borrow<str>,
        AsciiString: core::borrow::Borrow<AsciiStr>,
    {
    }

    #[test]
    fn borrow_mut()
    where
        AsciiString: core::borrow::BorrowMut<AsciiStr>,
    {
    }

    #[test]
    fn to_owned()
    where
        AsciiStr: alloc::borrow::ToOwned<Owned = AsciiString>,
    {
    }

    #[test]
    fn partial_eq_custom()
    where
        AsciiString: PartialEq<AsciiString>,
        AsciiString: PartialEq<AsciiStr>,
        AsciiStr: PartialEq<AsciiString>,
        for<'a> AsciiString: PartialEq<&'a AsciiStr>,
        for<'a> &'a AsciiStr: PartialEq<AsciiString>,
        for<'a> AsciiString: PartialEq<alloc::borrow::Cow<'a, AsciiStr>>,
        for<'a> alloc::borrow::Cow<'a, AsciiStr>: PartialEq<AsciiString>,
    {
    }

    #[test]
    fn partial_eq_inner()
    where
        AsciiString: PartialEq<str>,
        str: PartialEq<AsciiString>,
        for<'a> AsciiString: PartialEq<&'a str>,
        for<'a> &'a str: PartialEq<AsciiString>,
        for<'a> AsciiString: PartialEq<alloc::borrow::Cow<'a, str>>,
        for<'a> alloc::borrow::Cow<'a, str>: PartialEq<AsciiString>,
        String: PartialEq<AsciiStr>,
        AsciiStr: PartialEq<String>,
        for<'a> String: PartialEq<&'a AsciiStr>,
        for<'a> &'a AsciiStr: PartialEq<String>,
    {
    }

    #[test]
    fn partial_ord_custom()
    where
        AsciiString: PartialOrd<AsciiString>,
        AsciiString: PartialOrd<AsciiStr>,
        AsciiStr: PartialOrd<AsciiString>,
        for<'a> AsciiString: PartialOrd<&'a AsciiStr>,
        for<'a> &'a AsciiStr: PartialOrd<AsciiString>,
        for<'a> AsciiString: PartialOrd<alloc::borrow::Cow<'a, AsciiStr>>,
        for<'a> alloc::borrow::Cow<'a, AsciiStr>: PartialOrd<AsciiString>,
    {
    }

    #[test]
    fn partial_ord_inner()
    where
        AsciiString: PartialOrd<str>,
        str: PartialOrd<AsciiString>,
        for<'a> AsciiString: PartialOrd<&'a str>,
        for<'a> &'a str: PartialOrd<AsciiString>,
        for<'a> AsciiString: PartialOrd<alloc::borrow::Cow<'a, str>>,
        for<'a> alloc::borrow::Cow<'a, str>: PartialOrd<AsciiString>,
        String: PartialOrd<AsciiStr>,
        AsciiStr: PartialOrd<String>,
        for<'a> String: PartialOrd<&'a AsciiStr>,
        for<'a> &'a AsciiStr: PartialOrd<String>,
    {
    }

    #[test]
    fn from()
    where
        for<'a> AsciiString: From<&'a AsciiStr>,
    {
    }

    #[test]
    fn try_from()
    where
        for<'a> AsciiString: core::convert::TryFrom<&'a str>,
        AsciiString: core::convert::TryFrom<String>,
    {
    }

    #[test]
    fn fmt()
    where
        AsciiString: core::fmt::Debug,
        AsciiString: core::fmt::Display,
    {
        use core::convert::TryFrom;

        let sample_raw = "text";
        let sample_ascii = AsciiString::try_from(sample_raw).expect("Should never fail");
        assert_eq!(format!("{:?}", sample_ascii), "Ascii(\"text\")");
        assert_eq!(format!("{}", sample_ascii), sample_raw);
        let sample_ascii_borrowed = <&AsciiStr>::try_from(sample_raw).expect("Should never fail");
        assert_eq!(
            format!("{:?}", sample_ascii),
            format!("{:?}", sample_ascii_borrowed)
        );
        assert_eq!(
            format!("{}", sample_ascii),
            format!("{}", sample_ascii_borrowed)
        );
    }

    #[test]
    fn deref()
    where
        AsciiString: core::ops::Deref<Target = AsciiStr>,
    {
    }

    #[test]
    fn deref_mut()
    where
        AsciiString: core::ops::DerefMut<Target = AsciiStr>,
    {
    }

    #[test]
    fn from_str()
    where
        AsciiString: core::str::FromStr<Err = AsciiError>,
    {
    }
}

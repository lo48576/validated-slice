//! Plain string.
//!
//! String types defined here are almost same as std string types.

struct PlainStrSpec;

impl validated_slice::SliceSpec for PlainStrSpec {
    type Custom = PlainStr;
    type Inner = str;
    type Error = std::convert::Infallible;

    #[inline]
    fn validate(_: &Self::Inner) -> Result<(), Self::Error> {
        Ok(())
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

/// Plain string slice.
// `#[repr(transparent)]` or `#[repr(C)]` is required.
// Without it, generated codes would be unsound.
//
// You can use `#[derive(Debug, PartialEq, PartialOrd)]` here, but in this example they are
// implemented by macros in `validated_slice`.
#[repr(transparent)]
#[derive(Eq, Ord, Hash)]
pub struct PlainStr(str);

validated_slice::impl_std_traits_for_slice! {
    Spec {
        spec: PlainStrSpec,
        custom: PlainStr,
        inner: str,
        error: std::convert::Infallible,
    };
    // AsMut<str> for PlainStr
    // NOTE: `AsMut<[u8]> for str` is not implemented.
    //{ AsMut<str> };
    // AsMut<PlainStr> for PlainStr
    { AsMut<{Custom}> };
    // AsRef<[u8]> for PlainStr
    { AsRef<[u8]> };
    // AsRef<str> for PlainStr
    { AsRef<str> };
    // AsRef<PlainStr> for PlainStr
    { AsRef<{Custom}> };
    // From<&'_ str> for &'_ PlainStr
    { From<&{Inner}> for &{Custom} };
    // From<&'_ mut str> for &'_ mut PlainStr
    { From<&mut {Inner}> for &mut {Custom} };
    // From<&'_ PlainStr> for Arc<PlainStr>
    { From<&{Custom}> for Arc<{Custom}> };
    // From<&'_ PlainStr> for Box<PlainStr>
    { From<&{Custom}> for Box<{Custom}> };
    // From<&'_ PlainStr> for Rc<PlainStr>
    { From<&{Custom}> for Rc<{Custom}> };
    // Default for &'_ PlainStr
    { Default for &{Custom} };
    // Default for &'_ mut PlainStr
    { Default for &mut {Custom} };
    // Debug for PlainStr
    { Debug };
    // Display for PlainStr
    { Display };
    // Deref<Target = str> for PlainStr
    { Deref<Target = {Inner}> };
    // DerefMut<Target = str> for PlainStr
    { DerefMut<Target = {Inner}> };
}

validated_slice::impl_cmp_for_slice! {
    Spec {
        spec: PlainStrSpec,
        custom: PlainStr,
        inner: str,
        base: Inner,
    };
    Cmp { PartialEq, PartialOrd };
    // { lhs, rhs }.
    { ({Custom}), ({Custom}) };
    { ({Custom}), (&{Custom}), rev };
    // NOTE: This requires `std::borrow::ToOwned for PlainStr`.
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

struct PlainBoxStrSpec;

impl validated_slice::OwnedSliceSpec for PlainBoxStrSpec {
    type Custom = PlainBoxStr;
    type Inner = Box<str>;
    type Error = std::convert::Infallible;
    type SliceSpec = PlainStrSpec;
    type SliceCustom = PlainStr;
    type SliceInner = str;
    type SliceError = std::convert::Infallible;

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
        PlainBoxStr(s)
    }
}

/// ASCII string boxed slice.
#[derive(Default, Clone, Eq, Ord, Hash)]
pub struct PlainBoxStr(Box<str>);

impl From<PlainString> for PlainBoxStr {
    fn from(s: PlainString) -> Self {
        Self(s.0.into_boxed_str())
    }
}

validated_slice::impl_std_traits_for_owned_slice! {
    Spec {
        spec: PlainBoxStrSpec,
        custom: PlainBoxStr,
        inner: Box<str>,
        error: std::convert::Infallible,
        slice_custom: PlainStr,
        slice_inner: str,
        slice_error: std::convert::Infallible,
    };
    // AsMut<str> for PlainBoxStr
    // NOTE: `AsMut<[u8]> for str` is not implemented.
    //{ AsMut<str> };
    // AsMut<PlainStr> for PlainBoxStr
    { AsMut<{SliceCustom}> };
    // AsRef<[u8]> for PlainBoxStr
    { AsRef<[u8]> };
    // AsRef<str> for PlainBoxStr
    { AsRef<str> };
    // AsRef<PlainStr> for PlainBoxStr
    { AsRef<{SliceCustom}> };
    // Borrow<[u8]> for PlainBoxStr
    // NOTE: `Borrow<[u8]> for str` is not implemented.
    //{ Borrow<[u8]> };
    // Borrow<str> for PlainBoxStr
    { Borrow<str> };
    // Borrow<PlainStr> for PlainBoxStr
    { Borrow<{SliceCustom}> };
    // BorrowMut<str> for PlainBoxStr
    { BorrowMut<str> };
    // BorrowMut<PlainStr> for PlainBoxStr
    { BorrowMut<{SliceCustom}> };
    // ToOwned<Owned = PlainBoxStr> for PlainStr
    //{ ToOwned<Owned = {Custom}> for {SliceCustom} };
    // From<&'_ str> for PlainBoxStr
    { From<&{SliceInner}> };
    // From<&'_ PlainStr> for PlainBoxStr
    { From<&{SliceCustom}> };
    // From<Box<str>> for PlainBoxStr
    { From<{Inner}> };
    // Default for PlainBoxStr
    // NOTE: Same as `#[derive(Default)]` in this case.
    //{ Default };
    // Debug for PlainBoxStr
    { Debug };
    // Display for PlainBoxStr
    { Display };
    // Deref<Target = PlainStr> for PlainBoxStr
    { Deref<Target = {SliceCustom}> };
    // DerefMut<Target = PlainStr> for PlainBoxStr
    { DerefMut<Target = {SliceCustom}> };
}

validated_slice::impl_cmp_for_owned_slice! {
    Spec {
        spec: PlainBoxStrSpec,
        custom: PlainBoxStr,
        inner: Box<str>,
        slice_custom: PlainStr,
        slice_inner: str,
        base: Inner,
    };
    Cmp { PartialEq, PartialOrd };
    // { lhs, rhs }.
    { ({Custom}), ({Custom}) };
    { ({Custom}), ({SliceCustom}), rev };
    { ({Custom}), (&{SliceCustom}), rev };
    //// NOTE: This requires `std::borrow::Borrow for PlainBoxStr`.
    { ({Custom}), (Cow<{SliceCustom}>), rev };
    { ({Custom}), ({Inner}), rev };
    { ({Custom}), ({SliceInner}), rev };
    { ({Custom}), (&{SliceInner}), rev };
    { ({Custom}), (Cow<{SliceInner}>), rev };
    { ({Inner}), ({SliceCustom}), rev };
    { ({Inner}), (&{SliceCustom}), rev };
}

struct PlainStringSpec;

impl validated_slice::OwnedSliceSpec for PlainStringSpec {
    type Custom = PlainString;
    type Inner = String;
    type Error = std::convert::Infallible;
    type SliceSpec = PlainStrSpec;
    type SliceCustom = PlainStr;
    type SliceInner = str;
    type SliceError = std::convert::Infallible;

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
        PlainString(s)
    }
}

/// ASCII string boxed slice.
#[derive(Default, Clone, Eq, Ord, Hash)]
pub struct PlainString(String);

impl From<PlainBoxStr> for PlainString {
    fn from(s: PlainBoxStr) -> Self {
        Self(s.0.into())
    }
}

validated_slice::impl_std_traits_for_owned_slice! {
    Spec {
        spec: PlainStringSpec,
        custom: PlainString,
        inner: String,
        error: std::convert::Infallible,
        slice_custom: PlainStr,
        slice_inner: str,
        slice_error: std::convert::Infallible,
    };
    // AsMut<str> for PlainString
    // NOTE: `AsMut<[u8]> for str` is not implemented.
    //{ AsMut<str> };
    // AsMut<PlainStr> for PlainString
    { AsMut<{SliceCustom}> };
    // AsRef<[u8]> for PlainString
    { AsRef<[u8]> };
    // AsRef<str> for PlainString
    { AsRef<str> };
    // AsRef<PlainStr> for PlainString
    { AsRef<{SliceCustom}> };
    // Borrow<[u8]> for PlainString
    // NOTE: `Borrow<[u8]> for str` is not implemented.
    //{ Borrow<[u8]> };
    // Borrow<str> for PlainString
    { Borrow<str> };
    // Borrow<PlainStr> for PlainString
    { Borrow<{SliceCustom}> };
    // BorrowMut<PlainStr> for PlainString
    { BorrowMut<{SliceCustom}> };
    // ToOwned<Owned = PlainString> for PlainStr
    { ToOwned<Owned = {Custom}> for {SliceCustom} };
    // From<String> for PlainString
    { From<{Inner}> };
    // From<&'_ str> for PlainString
    { From<&{SliceInner}> };
    // From<&'_ PlainStr> for PlainString
    { From<&{SliceCustom}> };
    // Default for PlainString
    // NOTE: Same as `#[derive(Default)]` in this case.
    //{ Default };
    // Debug for PlainString
    { Debug };
    // Display for PlainString
    { Display };
    // Deref<Target = PlainStr> for PlainString
    { Deref<Target = {SliceCustom}> };
    // DerefMut<Target = PlainStr> for PlainString
    { DerefMut<Target = {SliceCustom}> };
}

validated_slice::impl_cmp_for_owned_slice! {
    Spec {
        spec: PlainStringSpec,
        custom: PlainString,
        inner: String,
        slice_custom: PlainStr,
        slice_inner: str,
        base: Inner,
    };
    Cmp { PartialEq, PartialOrd };
    // This is same as `#[derive(PartialEq, PartialOrd)]`.
    { ({Custom}), ({Custom}) };
    { ({Custom}), ({SliceCustom}), rev };
    { ({Custom}), (&{SliceCustom}), rev };
    //// NOTE: This requires `std::borrow::Borrow for PlainString`.
    { ({Custom}), (Cow<{SliceCustom}>), rev };
    { ({Custom}), ({Inner}), rev };
    { ({Custom}), ({SliceInner}), rev };
    { ({Custom}), (&{SliceInner}), rev };
    { ({Custom}), (Cow<{SliceInner}>), rev };
    { ({Inner}), ({SliceCustom}), rev };
    { ({Inner}), (&{SliceCustom}), rev };
}

#[cfg(test)]
mod plain_str {
    use super::*;

    #[test]
    fn as_ref()
    where
        PlainStr: AsRef<[u8]>,
        PlainStr: AsRef<str>,
        PlainStr: AsRef<PlainStr>,
        PlainStr: AsMut<PlainStr>,
    {
    }

    #[test]
    fn partial_eq_custom()
    where
        PlainStr: PartialEq<PlainStr>,
        for<'a> PlainStr: PartialEq<&'a PlainStr>,
        for<'a> &'a PlainStr: PartialEq<PlainStr>,
        for<'a> PlainStr: PartialEq<std::borrow::Cow<'a, PlainStr>>,
        for<'a> std::borrow::Cow<'a, PlainStr>: PartialEq<std::borrow::Cow<'a, PlainStr>>,
    {
    }

    #[test]
    fn partial_eq_inner()
    where
        PlainStr: PartialEq<str>,
        str: PartialEq<PlainStr>,
        for<'a> PlainStr: PartialEq<&'a str>,
        for<'a> &'a str: PartialEq<PlainStr>,
        for<'a> &'a PlainStr: PartialEq<str>,
        for<'a> str: PartialEq<&'a PlainStr>,
        for<'a> PlainStr: PartialEq<std::borrow::Cow<'a, str>>,
        for<'a> std::borrow::Cow<'a, str>: PartialEq<PlainStr>,
        for<'a, 'b> &'b PlainStr: PartialEq<std::borrow::Cow<'a, str>>,
        for<'a, 'b> std::borrow::Cow<'a, str>: PartialEq<&'b PlainStr>,
    {
    }

    #[test]
    fn from()
    where
        for<'a> &'a PlainStr: From<&'a str>,
        for<'a> &'a mut PlainStr: From<&'a mut str>,
    {
    }

    #[test]
    fn from_smart_ptr()
    where
        for<'a> std::sync::Arc<PlainStr>: From<&'a PlainStr>,
        for<'a> Box<PlainStr>: From<&'a PlainStr>,
        for<'a> std::rc::Rc<PlainStr>: From<&'a PlainStr>,
    {
    }

    #[test]
    fn default()
    where
        for<'a> &'a PlainStr: Default,
        for<'a> &'a mut PlainStr: Default,
    {
    }

    #[test]
    fn fmt()
    where
        PlainStr: std::fmt::Debug,
        PlainStr: std::fmt::Display,
    {
        let sample_raw = "text";
        let sample_plain = <&PlainStr>::from(sample_raw);
        assert_eq!(format!("{:?}", sample_plain), "\"text\"");
        assert_eq!(format!("{}", sample_plain), sample_raw);
    }

    #[test]
    fn deref()
    where
        PlainStr: std::ops::Deref<Target = str>,
        PlainStr: std::ops::DerefMut<Target = str>,
    {
    }
}

#[cfg(test)]
mod plain_box_str {
    use super::*;

    #[test]
    fn as_ref()
    where
        PlainBoxStr: AsRef<[u8]>,
        PlainBoxStr: AsRef<str>,
        PlainBoxStr: AsRef<PlainStr>,
        PlainBoxStr: AsMut<PlainStr>,
    {
    }

    #[test]
    fn borrow()
    where
        PlainBoxStr: std::borrow::Borrow<str>,
        PlainBoxStr: std::borrow::Borrow<PlainStr>,
    {
    }

    #[test]
    fn borrow_mut()
    where
        PlainBoxStr: std::borrow::BorrowMut<str>,
        PlainBoxStr: std::borrow::BorrowMut<PlainStr>,
    {
    }

    #[test]
    fn partial_eq_custom()
    where
        PlainBoxStr: PartialEq<PlainBoxStr>,
        PlainBoxStr: PartialEq<PlainStr>,
        PlainStr: PartialEq<PlainBoxStr>,
        for<'a> PlainBoxStr: PartialEq<&'a PlainStr>,
        for<'a> &'a PlainStr: PartialEq<PlainBoxStr>,
        for<'a> PlainBoxStr: PartialEq<std::borrow::Cow<'a, PlainStr>>,
        for<'a> std::borrow::Cow<'a, PlainStr>: PartialEq<PlainBoxStr>,
    {
    }

    #[test]
    fn partial_eq_inner()
    where
        PlainBoxStr: PartialEq<str>,
        str: PartialEq<PlainBoxStr>,
        for<'a> PlainBoxStr: PartialEq<&'a str>,
        for<'a> &'a str: PartialEq<PlainBoxStr>,
        for<'a> PlainBoxStr: PartialEq<std::borrow::Cow<'a, str>>,
        for<'a> std::borrow::Cow<'a, str>: PartialEq<PlainBoxStr>,
        Box<str>: PartialEq<PlainStr>,
        PlainStr: PartialEq<Box<str>>,
        for<'a> Box<str>: PartialEq<&'a PlainStr>,
        for<'a> &'a PlainStr: PartialEq<Box<str>>,
    {
    }

    #[test]
    fn partial_ord_custom()
    where
        PlainBoxStr: PartialOrd<PlainBoxStr>,
        PlainBoxStr: PartialOrd<PlainStr>,
        PlainStr: PartialOrd<PlainBoxStr>,
        for<'a> PlainBoxStr: PartialOrd<&'a PlainStr>,
        for<'a> &'a PlainStr: PartialOrd<PlainBoxStr>,
        for<'a> PlainBoxStr: PartialOrd<std::borrow::Cow<'a, PlainStr>>,
        for<'a> std::borrow::Cow<'a, PlainStr>: PartialOrd<PlainBoxStr>,
    {
    }

    #[test]
    fn partial_ord_inner()
    where
        PlainBoxStr: PartialOrd<str>,
        str: PartialOrd<PlainBoxStr>,
        for<'a> PlainBoxStr: PartialOrd<&'a str>,
        for<'a> &'a str: PartialOrd<PlainBoxStr>,
        for<'a> PlainBoxStr: PartialOrd<std::borrow::Cow<'a, str>>,
        for<'a> std::borrow::Cow<'a, str>: PartialOrd<PlainBoxStr>,
        Box<str>: PartialOrd<PlainStr>,
        PlainStr: PartialOrd<Box<str>>,
        for<'a> Box<str>: PartialOrd<&'a PlainStr>,
        for<'a> &'a PlainStr: PartialOrd<Box<str>>,
    {
    }

    #[test]
    fn from()
    where
        for<'a> PlainBoxStr: From<&'a str>,
        for<'a> PlainBoxStr: From<&'a PlainStr>,
        PlainBoxStr: From<Box<str>>,
    {
    }

    #[test]
    fn fmt()
    where
        PlainBoxStr: std::fmt::Debug,
        PlainBoxStr: std::fmt::Display,
    {
        let sample_raw = "text";
        let sample_plain = PlainBoxStr::from(sample_raw);
        assert_eq!(format!("{:?}", sample_plain), "\"text\"");
        assert_eq!(format!("{}", sample_plain), sample_raw);
        let sample_plain_borrowed = <&PlainStr>::from(sample_raw);
        assert_eq!(
            format!("{:?}", sample_plain),
            format!("{:?}", sample_plain_borrowed)
        );
        assert_eq!(
            format!("{}", sample_plain),
            format!("{}", sample_plain_borrowed)
        );
    }

    #[test]
    fn deref()
    where
        PlainBoxStr: std::ops::Deref<Target = PlainStr>,
    {
    }

    #[test]
    fn deref_mut()
    where
        PlainBoxStr: std::ops::DerefMut<Target = PlainStr>,
    {
    }
}

#[cfg(test)]
mod plain_string {
    use super::*;

    #[test]
    fn as_ref()
    where
        PlainString: AsRef<[u8]>,
        PlainString: AsRef<str>,
        PlainString: AsRef<PlainStr>,
        PlainString: AsMut<PlainStr>,
    {
    }

    #[test]
    fn borrow()
    where
        PlainString: std::borrow::Borrow<str>,
        PlainString: std::borrow::Borrow<PlainStr>,
    {
    }

    #[test]
    fn borrow_mut()
    where
        PlainString: std::borrow::BorrowMut<PlainStr>,
    {
    }

    #[test]
    fn to_owned()
    where
        PlainStr: std::borrow::ToOwned<Owned = PlainString>,
    {
    }

    #[test]
    fn partial_eq_custom()
    where
        PlainString: PartialEq<PlainString>,
        PlainString: PartialEq<PlainStr>,
        PlainStr: PartialEq<PlainString>,
        for<'a> PlainString: PartialEq<&'a PlainStr>,
        for<'a> &'a PlainStr: PartialEq<PlainString>,
        for<'a> PlainString: PartialEq<std::borrow::Cow<'a, PlainStr>>,
        for<'a> std::borrow::Cow<'a, PlainStr>: PartialEq<PlainString>,
    {
    }

    #[test]
    fn partial_eq_inner()
    where
        PlainString: PartialEq<str>,
        str: PartialEq<PlainString>,
        for<'a> PlainString: PartialEq<&'a str>,
        for<'a> &'a str: PartialEq<PlainString>,
        for<'a> PlainString: PartialEq<std::borrow::Cow<'a, str>>,
        for<'a> std::borrow::Cow<'a, str>: PartialEq<PlainString>,
        String: PartialEq<PlainStr>,
        PlainStr: PartialEq<String>,
        for<'a> String: PartialEq<&'a PlainStr>,
        for<'a> &'a PlainStr: PartialEq<String>,
    {
    }

    #[test]
    fn partial_ord_custom()
    where
        PlainString: PartialOrd<PlainString>,
        PlainString: PartialOrd<PlainStr>,
        PlainStr: PartialOrd<PlainString>,
        for<'a> PlainString: PartialOrd<&'a PlainStr>,
        for<'a> &'a PlainStr: PartialOrd<PlainString>,
        for<'a> PlainString: PartialOrd<std::borrow::Cow<'a, PlainStr>>,
        for<'a> std::borrow::Cow<'a, PlainStr>: PartialOrd<PlainString>,
    {
    }

    #[test]
    fn partial_ord_inner()
    where
        PlainString: PartialOrd<str>,
        str: PartialOrd<PlainString>,
        for<'a> PlainString: PartialOrd<&'a str>,
        for<'a> &'a str: PartialOrd<PlainString>,
        for<'a> PlainString: PartialOrd<std::borrow::Cow<'a, str>>,
        for<'a> std::borrow::Cow<'a, str>: PartialOrd<PlainString>,
        String: PartialOrd<PlainStr>,
        PlainStr: PartialOrd<String>,
        for<'a> String: PartialOrd<&'a PlainStr>,
        for<'a> &'a PlainStr: PartialOrd<String>,
    {
    }

    #[test]
    fn from()
    where
        for<'a> PlainString: From<&'a str>,
        for<'a> PlainString: From<&'a PlainStr>,
        PlainString: From<String>,
    {
    }

    #[test]
    fn fmt()
    where
        PlainString: std::fmt::Debug,
        PlainString: std::fmt::Display,
    {
        let sample_raw = "text";
        let sample_plain = PlainString::from(sample_raw);
        assert_eq!(format!("{:?}", sample_plain), "\"text\"");
        assert_eq!(format!("{}", sample_plain), sample_raw);
        let sample_plain_borrowed = <&PlainStr>::from(sample_raw);
        assert_eq!(
            format!("{:?}", sample_plain),
            format!("{:?}", sample_plain_borrowed)
        );
        assert_eq!(
            format!("{}", sample_plain),
            format!("{}", sample_plain_borrowed)
        );
    }

    #[test]
    fn deref()
    where
        PlainString: std::ops::Deref<Target = PlainStr>,
    {
    }

    #[test]
    fn deref_mut()
    where
        PlainString: std::ops::DerefMut<Target = PlainStr>,
    {
    }
}

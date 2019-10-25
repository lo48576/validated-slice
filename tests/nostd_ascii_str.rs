//! ASCII string in nostd environment.
//!
//! Types for strings which consists of only ASCII characters.

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
        alloc: alloc_should_never_used,
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
        alloc: alloc_should_never_used,
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

    { ({Custom}), ({Inner}), rev };
    { ({Custom}), (&{Inner}), rev };
    { (&{Custom}), ({Inner}), rev };
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
    {
    }

    #[test]
    fn from()
    where
        for<'a> &'a str: From<&'a AsciiStr>,
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

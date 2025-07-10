//! String related types

use std::{
    borrow::{Borrow, Cow},
    fmt,
    ops::{Deref, DerefMut},
    sync::LazyLock,
};

use malloc_size_of_derive::MallocSizeOf;
use regex::Regex;

/// A DOMString.
///
/// This type corresponds to the [`DOMString`] type in WebIDL.
///
/// [`DOMString`]: https://webidl.spec.whatwg.org/#idl-DOMString
#[derive(Clone, Debug, Default, Eq, Hash, MallocSizeOf, Ord, PartialEq, PartialOrd)]
pub struct DOMString(String);

impl DOMString {
    /// Creates a new `DOMString`.
    pub fn new() -> DOMString {
        DOMString(String::new())
    }

    /// Creates a new `DOMString` from a `String`.
    pub fn from_string(s: String) -> DOMString {
        DOMString(s)
    }

    /// Get the internal `&str` value of this [`DOMString`].
    pub fn str(&self) -> &str {
        &self.0
    }

    /// Appends a given string slice onto the end of this String.
    pub fn push_str(&mut self, string: &str) {
        self.0.push_str(string)
    }

    /// Clears this `DOMString`, removing all contents.
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Shortens this String to the specified length.
    pub fn truncate(&mut self, new_len: usize) {
        self.0.truncate(new_len);
    }

    /// Removes newline characters according to <https://infra.spec.whatwg.org/#strip-newlines>.
    pub fn strip_newlines(&mut self) {
        self.0.retain(|c| c != '\r' && c != '\n');
    }

    /// Removes leading and trailing ASCII whitespaces according to
    /// <https://infra.spec.whatwg.org/#strip-leading-and-trailing-ascii-whitespace>.
    pub fn strip_leading_and_trailing_ascii_whitespace(&mut self) {
        if self.0.is_empty() {
            return;
        }

        let trailing_whitespace_len = self
            .0
            .trim_end_matches(|ref c| char::is_ascii_whitespace(c))
            .len();
        self.0.truncate(trailing_whitespace_len);
        if self.0.is_empty() {
            return;
        }

        let first_non_whitespace = self.0.find(|ref c| !char::is_ascii_whitespace(c)).unwrap();
        self.0.replace_range(0..first_non_whitespace, "");
    }

    /// <https://html.spec.whatwg.org/multipage/#valid-floating-point-number>
    pub fn is_valid_floating_point_number_string(&self) -> bool {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^-?(?:\d+\.\d+|\d+|\.\d+)(?:(e|E)(\+|\-)?\d+)?$").unwrap()
        });

        RE.is_match(&self.0) && self.parse_floating_point_number().is_some()
    }

    /// <https://html.spec.whatwg.org/multipage/#rules-for-parsing-floating-point-number-values>
    pub fn parse_floating_point_number(&self) -> Option<f64> {
        // Steps 15-16 are telling us things about IEEE rounding modes
        // for floating-point significands; this code assumes the Rust
        // compiler already matches them in any cases where
        // that actually matters. They are not
        // related to f64::round(), which is for rounding to integers.
        let input = &self.0;
        if let Ok(val) = input.trim().parse::<f64>() {
            if !(
                // A valid number is the same as what rust considers to be valid,
                // except for +1., NaN, and Infinity.
                val.is_infinite() || val.is_nan() || input.ends_with('.') || input.starts_with('+')
            ) {
                return Some(val);
            }
        }
        None
    }

    /// Applies the same processing as `parse_floating_point_number` with some additional handling
    /// according to ECMA's string conversion steps.
    ///
    /// Used for specific elements when handling floating point values, namely the `number` and
    /// `range` inputs, as well as `meter` and `progress` elements.
    ///
    /// <https://html.spec.whatwg.org/multipage/#best-representation-of-the-number-as-a-floating-point-number>
    /// <https://tc39.es/ecma262/#sec-numeric-types-number-tostring>
    pub fn set_best_representation_of_the_floating_point_number(&mut self) {
        if let Some(val) = self.parse_floating_point_number() {
            // [tc39] Step 2: If x is either +0 or -0, return "0".
            let parsed_value = if val == 0.0 || val == -0.0 {
                0.0_f64
            } else {
                val
            };

            self.0 = parsed_value.to_string()
        }
    }
}

impl Borrow<str> for DOMString {
    #[inline]
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Deref for DOMString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl DerefMut for DOMString {
    #[inline]
    fn deref_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl AsRef<str> for DOMString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DOMString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl PartialEq<str> for DOMString {
    fn eq(&self, other: &str) -> bool {
        &**self == other
    }
}

impl<'a> PartialEq<&'a str> for DOMString {
    fn eq(&self, other: &&'a str) -> bool {
        &**self == *other
    }
}

impl From<String> for DOMString {
    fn from(contents: String) -> DOMString {
        DOMString(contents)
    }
}

impl From<&str> for DOMString {
    fn from(contents: &str) -> DOMString {
        DOMString::from(String::from(contents))
    }
}

impl<'a> From<Cow<'a, str>> for DOMString {
    fn from(contents: Cow<'a, str>) -> DOMString {
        match contents {
            Cow::Owned(s) => DOMString::from(s),
            Cow::Borrowed(s) => DOMString::from(s),
        }
    }
}

// TODO: implement this when html5ever is imported.
// impl From<DOMString> for LocalName {
//     fn from(contents: DOMString) -> LocalName {
//         LocalName::from(contents.0)
//     }
// }
//
// impl From<DOMString> for Namespace {
//     fn from(contents: DOMString) -> Namespace {
//         Namespace::from(contents.0)
//     }
// }
//
// impl From<DOMString> for Atom {
//     fn from(contents: DOMString) -> Atom {
//         Atom::from(contents.0)
//     }
// }
//
// impl<'a> From<DOMString> for CowRcStr<'a> {
//     fn from(contents: DOMString) -> CowRcStr<'a> {
//         contents.0.into()
//     }
// }

impl From<DOMString> for String {
    fn from(contents: DOMString) -> String {
        contents.0
    }
}

impl From<DOMString> for Vec<u8> {
    fn from(contents: DOMString) -> Vec<u8> {
        contents.0.into()
    }
}

impl<'a> From<DOMString> for Cow<'a, str> {
    fn from(contents: DOMString) -> Cow<'a, str> {
        contents.0.into()
    }
}

impl Extend<char> for DOMString {
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = char>,
    {
        self.0.extend(iterable)
    }
}

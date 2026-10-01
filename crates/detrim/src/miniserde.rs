//! String wrappers for miniserde models.
//!
//! [`Trimmed`] supports `String`, `Cow<'_, str>`, `Vec<String>`, and, with the `std`
//! feature, `HashSet<String>`. Strings are trimmed during deserialization.
//! Serialization preserves the stored value. Use `Option<Trimmed<String>>` for
//! nullable strings. Miniserde always produces owned strings, including for `Cow`.
//!
//! ```
//! use detrim::miniserde::Trimmed;
//!
//! #[derive(miniserde::Deserialize)]
//! struct Form {
//!     name: Trimmed<String>,
//! }
//!
//! let form: Form = miniserde::json::from_str(r#"{"name":"  ferris  "}"#).unwrap();
//! assert_eq!(form.name.0, "ferris");
//! ```

use alloc::{
    borrow::{Cow, ToOwned as _},
    boxed::Box,
    string::String,
    vec::Vec,
};
use core::mem;
#[cfg(feature = "std")]
use std::collections::HashSet;

use ::miniserde::{
    de::{Seq, Visitor},
    ser::Fragment,
    Deserialize, Error, Result, Serialize,
};

::miniserde::make_place!(Place);

/// Trims strings, or each string in a collection, during deserialization.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Trimmed<T>(pub T);

impl Deserialize for Trimmed<String> {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
        Place::new(out)
    }
}

impl Visitor for Place<Trimmed<String>> {
    fn string(&mut self, value: &str) -> Result<()> {
        self.out = Some(Trimmed(value.trim().to_owned()));

        Ok(())
    }
}

impl Serialize for Trimmed<String> {
    fn begin(&self) -> Fragment<'_> {
        Serialize::begin(&self.0)
    }
}

impl Deserialize for Trimmed<Cow<'_, str>> {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
        Place::new(out)
    }
}

impl Visitor for Place<Trimmed<Cow<'_, str>>> {
    fn string(&mut self, value: &str) -> Result<()> {
        self.out = Some(Trimmed(Cow::Owned(value.trim().to_owned())));

        Ok(())
    }
}

impl Serialize for Trimmed<Cow<'_, str>> {
    fn begin(&self) -> Fragment<'_> {
        Fragment::Str(Cow::Borrowed(&self.0))
    }
}

/// Trims a string during deserialization and rejects an empty result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyString(pub String);

impl Deserialize for NonEmptyString {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
        Place::new(out)
    }
}

impl Visitor for Place<NonEmptyString> {
    fn string(&mut self, value: &str) -> Result<()> {
        let value = value.trim();

        if value.is_empty() {
            return Err(Error);
        }

        self.out = Some(NonEmptyString(value.to_owned()));

        Ok(())
    }
}

impl Serialize for NonEmptyString {
    fn begin(&self) -> Fragment<'_> {
        Serialize::begin(&self.0)
    }
}

/// Trims a string and maps null or an empty result to `None`.
///
/// The field is required. Use `Option<OptionNonEmptyString>` to allow a missing field.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OptionNonEmptyString(pub Option<String>);

impl Deserialize for OptionNonEmptyString {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
        Place::new(out)
    }
}

impl Visitor for Place<OptionNonEmptyString> {
    fn null(&mut self) -> Result<()> {
        self.out = Some(OptionNonEmptyString(None));

        Ok(())
    }

    fn string(&mut self, value: &str) -> Result<()> {
        let value = value.trim();
        self.out = Some(OptionNonEmptyString(
            (!value.is_empty()).then(|| value.to_owned()),
        ));

        Ok(())
    }
}

impl Serialize for OptionNonEmptyString {
    fn begin(&self) -> Fragment<'_> {
        Serialize::begin(&self.0)
    }
}

macro_rules! collection_impls {
    ($collection:ty) => {
        impl Deserialize for Trimmed<$collection> {
            fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
                Place::new(out)
            }
        }

        impl Visitor for Place<Trimmed<$collection>> {
            fn seq(&mut self) -> Result<Box<dyn Seq + '_>> {
                Ok(Box::new(StringSeq {
                    out: &mut self.out,
                    values: Vec::new(),
                    element: None,
                }))
            }
        }
    };
}

collection_impls!(Vec<String>);
#[cfg(feature = "std")]
collection_impls!(HashSet<String>);

struct StringSeq<'a, C> {
    out: &'a mut Option<Trimmed<C>>,
    values: Vec<String>,
    element: Option<Trimmed<String>>,
}

impl<C: FromIterator<String>> Seq for StringSeq<'_, C> {
    fn element(&mut self) -> Result<&mut dyn Visitor> {
        if let Some(value) = self.element.take() {
            self.values.push(value.0);
        }

        Ok(Deserialize::begin(&mut self.element))
    }

    fn finish(&mut self) -> Result<()> {
        if let Some(value) = self.element.take() {
            self.values.push(value.0);
        }

        *self.out = Some(Trimmed(mem::take(&mut self.values).into_iter().collect()));

        Ok(())
    }
}

impl Serialize for Trimmed<Vec<String>> {
    fn begin(&self) -> Fragment<'_> {
        Serialize::begin(&self.0)
    }
}

#[cfg(feature = "std")]
impl Serialize for Trimmed<HashSet<String>> {
    fn begin(&self) -> Fragment<'_> {
        struct SetSeq<'a>(std::collections::hash_set::Iter<'a, String>);

        impl ::miniserde::ser::Seq for SetSeq<'_> {
            fn next(&mut self) -> Option<&dyn Serialize> {
                self.0.next().map(|value| value as &dyn Serialize)
            }
        }

        Fragment::Seq(Box::new(SetSeq(self.0.iter())))
    }
}

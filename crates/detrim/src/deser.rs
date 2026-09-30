//! String trimming adapters for `deser`.
//!
//! Use `#[deser(deserialize_as = Trim)]` for strings, `Option<Trim>` for
//! optional strings, and `Vec<Trim>` or `HashSet<Trim>` for collections.
//! For a borrowed `Cow<str>`, use `deser::adapters::TrimWhitespace<deser::adapters::Borrowed>`.
//!
//! ```
//! #[derive(Debug, deser::Deserialize)]
//! struct Form {
//!     #[deser(deserialize_as = detrim::deser::NonEmpty)]
//!     name: String,
//! }
//!
//! let form = deser_json::from_str::<Form>(r#"{"name":"  ferris  "}"#).unwrap();
//! assert_eq!(form.name, "ferris");
//! ```

use alloc::{borrow::ToOwned as _, string::String};

/// Trims whitespace from strings, including string slices.
///
/// Compose with `Option`, `Vec`, or `HashSet` to trim their string values.
pub use ::deser::adapters::TrimWhitespace as Trim;
use ::deser::{
    adapters::DeserializeAs,
    de::{Sink, SinkHandle},
    Atom, Error, ErrorKind, State,
};

/// Trims a string and returns an error if it is empty.
pub struct NonEmpty;

/// Trims an optional string and converts empty strings to `None`.
pub struct EmptyAsNone;

::deser::make_slot_wrapper!(TrimSlot);

impl<'de> Sink<'de> for TrimSlot<String> {
    fn atom(&mut self, atom: Atom<'_>, state: &mut State) -> Result<(), Error> {
        match atom {
            Atom::Str(value) if value.trim().is_empty() => Err(Error::new(
                ErrorKind::Unexpected,
                "expected a non-empty string",
            )),

            Atom::Str(value) => {
                **self = Some(value.trim().to_owned());
                Ok(())
            }

            other => self.unexpected_atom(other, state),
        }
    }
}

impl<'de> DeserializeAs<'de, String> for NonEmpty {
    fn deserialize_into_as<'out>(
        out: &'out mut Option<String>,
        _state: &mut State,
    ) -> SinkHandle<'out, 'de> {
        TrimSlot::make_handle(out)
    }
}

impl<'de> Sink<'de> for TrimSlot<Option<String>> {
    fn atom(&mut self, atom: Atom<'_>, state: &mut State) -> Result<(), Error> {
        match atom {
            Atom::Null => {
                **self = Some(None);
                Ok(())
            }

            Atom::Str(value) => {
                **self = Some((!value.trim().is_empty()).then(|| value.trim().to_owned()));
                Ok(())
            }

            other => self.unexpected_atom(other, state),
        }
    }
}

impl<'de> DeserializeAs<'de, Option<String>> for EmptyAsNone {
    fn deserialize_into_as<'out>(
        out: &'out mut Option<Option<String>>,
        _state: &mut State,
    ) -> SinkHandle<'out, 'de> {
        TrimSlot::make_handle(out)
    }

    fn initial_value_as() -> Option<Option<String>> {
        Some(None)
    }
}

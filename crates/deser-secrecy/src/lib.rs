//! Explicit secret exposure for `deser` serialization.
//!
//! ```
//! #[derive(deser::Serialize)]
//! struct Login {
//!     #[deser(serialize_as = deser_secrecy::ExposeSecretString)]
//!     password: secrecy::SecretString,
//! }
//!
//! let login = Login {
//!     password: secrecy::SecretString::from("hunter2"),
//! };
//!
//! assert_eq!(
//!     deser_json::to_string(&login).unwrap(),
//!     r#"{"password":"hunter2"}"#,
//! );
//! ```

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use deser::{adapters::SerializeAs, ser::Chunk, Atom, Error, State};
use secrecy::{ExposeSecret as _, SecretString};

/// Serializes a `SecretString` by exposing its inner string.
pub struct ExposeSecretString;

impl SerializeAs<SecretString> for ExposeSecretString {
    fn serialize_as<'a>(value: &'a SecretString, _state: &mut State) -> Result<Chunk<'a>, Error> {
        Ok(Chunk::Atom(Atom::Str(value.expose_secret().into())))
    }
}

//! Miniserde support for secret strings.
//!
//! [`ExposeSecretString`] keeps secrets redacted in debug output and exposes them
//! during JSON serialization.
//!
//! ```
//! use serde_secrecy::miniserde::ExposeSecretString;
//!
//! #[derive(Debug, miniserde::Serialize, miniserde::Deserialize)]
//! struct Login {
//!     password: ExposeSecretString,
//! }
//!
//! let login: Login = miniserde::json::from_str(r#"{"password":"hunter2"}"#).unwrap();
//! assert!(!format!("{login:?}").contains("hunter2"));
//! assert_eq!(miniserde::json::to_string(&login), r#"{"password":"hunter2"}"#);
//! ```

use std::borrow::Cow;

use ::miniserde::{de::Visitor, ser::Fragment, Deserialize, Result, Serialize};
use secrecy::{ExposeSecret as _, SecretString};

::miniserde::make_place!(Place);

/// A secret string that is exposed during miniserde serialization.
#[derive(Debug, Clone)]
pub struct ExposeSecretString(pub SecretString);

impl Deserialize for ExposeSecretString {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
        Place::new(out)
    }
}

impl Visitor for Place<ExposeSecretString> {
    fn string(&mut self, value: &str) -> Result<()> {
        self.out = Some(ExposeSecretString(SecretString::from(value)));

        Ok(())
    }
}

impl Serialize for ExposeSecretString {
    fn begin(&self) -> Fragment<'_> {
        Fragment::Str(Cow::Borrowed(self.0.expose_secret()))
    }
}

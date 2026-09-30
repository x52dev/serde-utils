//! Serde support for [`secrecy`] types.
//!
//! The `serde` feature enables the field adapter and is enabled by default.
//! Enable `miniserde` to use the wrapper in the `miniserde` module.
//! Disable default features for a miniserde-only build.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "miniserde")]
pub mod miniserde;

#[cfg(feature = "serde")]
use secrecy::{ExposeSecret as _, SecretString};
#[cfg(feature = "serde")]
use serde_core::Serializer;

/// Enables serialization of [`secrecy::SecretString`] fields by exposing the inner string.
///
/// # Examples
///
/// ```
/// # use serde::Serialize;
/// # use secrecy::SecretString;
/// #[derive(Debug, Serialize)]
/// struct Login {
///     #[serde(serialize_with = "serde_secrecy::expose_secret_string")]
///     password: SecretString,
/// }
///
/// let req = Login {
///     password: SecretString::from("hunter2"),
/// };
///
/// let json = serde_json::to_string(&req).unwrap();
/// assert!(json.contains("hunter2"));
/// ```
#[cfg(feature = "serde")]
pub fn expose_secret_string<S: Serializer>(
    secret: &SecretString,
    ser: S,
) -> Result<S::Ok, S::Error> {
    ser.serialize_str(secret.expose_secret())
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use serde::Serialize;

    use super::*;

    #[test]
    fn serialize_secret_string() {
        #[derive(Debug, Serialize)]
        struct Login {
            email: String,
            #[serde(serialize_with = "expose_secret_string")]
            password: SecretString,
        }

        let req = Login {
            email: "foo@example.com".to_owned(),
            password: SecretString::from("hunter2"),
        };

        let debug = format!("{req:?}");
        let json = serde_json::to_string(&req).unwrap();

        assert!(debug.contains("example.com"));
        assert!(json.contains("example.com"));

        assert!(!debug.contains("hunter2"));
        assert!(json.contains("hunter2"));
    }
}

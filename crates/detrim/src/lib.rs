//! **De**serialization **trim**ming for strings in serde or deser models.
//!
//! The `serde` feature enables the Serde functions and is enabled by default.
//! Enable the `deser` feature for field adapters in `detrim::deser`.
//!
//! # Examples
//!
//! ```
//! # #[cfg(feature = "serde")]
//! # {
//! #[derive(Debug, serde::Deserialize)]
//! struct Form {
//!     #[serde(deserialize_with = "detrim::string")]
//!     name: String,
//! }
//!
//! let form = serde_json::from_str::<Form>(r#"{ "name": "ferris" }"#).unwrap();
//! assert_eq!(form.name, "ferris");
//!
//! let form = serde_json::from_str::<Form>(r#"{ "name": "  ferris   " }"#).unwrap();
//! assert_eq!(form.name, "ferris");
//! # }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(any(feature = "serde", feature = "deser"))]
extern crate alloc;

#[cfg(feature = "deser")]
pub mod deser;

#[cfg(feature = "serde")]
mod cow_str;
#[cfg(all(feature = "serde", feature = "std"))]
mod hashset_string;
#[cfg(feature = "serde")]
mod string;
#[cfg(feature = "serde")]
mod string_non_empty;
#[cfg(feature = "serde")]
mod vec_string;

#[cfg(all(feature = "serde", feature = "std"))]
pub use crate::hashset_string::hashset_string;
#[cfg(feature = "serde")]
pub use crate::{
    cow_str::cow_str,
    string::{option_string, str, string},
    string_non_empty::{option_string_non_empty, string_non_empty},
    vec_string::vec_string,
};

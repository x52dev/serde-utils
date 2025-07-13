//! The double-int format represents an integer that can be stored in an IEEE 754 double-precision
//! number without loss of precision.
//!
//! This crate has been designed for use with OpenAPI tooling that wish to support integer-based
//! `format: double-int` fields. [See docs in the OpenAPI format registry.][reg_double_int]
//!
//! # Examples
//!
//! ```
//! # use double_int::DoubleInt;
//! #[derive(Debug, serde::Deserialize)]
//! struct Config {
//!     count: DoubleInt,
//! }
//!
//! let config = toml::from_str::<Config>(r#"
//!     count = 42
//! "#).unwrap();
//! assert_eq!(config.count, 42);
//!
//! let config = toml::from_str::<Config>(r#"
//!     count = -42
//! "#).unwrap();
//! assert_eq!(config.count, -42);
//!
//! // count is outside the bounds of a double-int (> 2^53 in this case)
//! // (this would usually be accepted by an i64)
//! let config = toml::from_str::<Config>(r#"
//!     count = 36028797018963968
//! "#).unwrap_err();
//! ```
//!
//! [reg_double_int]: https://spec.openapis.org/registry/format/double-int

#![no_std]
#![deny(rust_2018_idioms, nonstandard_style, future_incompatible)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Type that only deserializes from the `true` boolean value.
///
/// # Examples
///
/// ```
/// assert_eq!(
///     serde_json::from_str::<double_int::DoubleInt>("42").unwrap(),
///     42,
/// );
///
/// serde_json::from_str::<double_int::DoubleInt>("4.2").unwrap_err();
/// serde_json::from_str::<double_int::DoubleInt>("36028797018963968").unwrap_err();
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DoubleInt(i64);

impl DoubleInt {
    const MIN: i128 = -(2_i128.pow(53)) + 1;
    const MAX: i128 = 2_i128.pow(53) - 1;
    const UMAX: u128 = 2_u128.pow(53) - 1;

    /// Returns value as a standard type.
    pub const fn as_i64(self) -> i64 {
        self.0
    }
}

macro_rules! from_impl {
    ($ty:ty) => {
        impl From<$ty> for DoubleInt {
            fn from(val: $ty) -> Self {
                Self(val as i64)
            }
        }
    };
}

from_impl!(u8);
from_impl!(u16);
from_impl!(u32);
from_impl!(i8);
from_impl!(i16);
from_impl!(i32);

macro_rules! infallible_eq_impls {
    ($ty:ty) => {
        impl PartialEq<$ty> for DoubleInt {
            fn eq(&self, val: &$ty) -> bool {
                self.0 == *val as i64
            }
        }
    };
}

infallible_eq_impls!(u8);
infallible_eq_impls!(u16);
infallible_eq_impls!(u32);
infallible_eq_impls!(i8);
infallible_eq_impls!(i16);
infallible_eq_impls!(i32);

impl PartialEq<u64> for DoubleInt {
    fn eq(&self, val: &u64) -> bool {
        match *val as u128 {
            // self cannot be larger than UMAX so val is not equal
            DoubleInt::UMAX.. => false,

            // all remaining u64s would be representable by i64
            // just cast and check equality
            _ => self.0 == *val as i64,
        }
    }
}

impl PartialEq<u128> for DoubleInt {
    fn eq(&self, val: &u128) -> bool {
        match val {
            // self cannot be larger than UMAX so val is not equal
            DoubleInt::UMAX.. => false,

            // all remaining u64s would be representable by i64
            // just cast and check equality
            _ => self.0 == *val as i64,
        }
    }
}

impl PartialEq<i64> for DoubleInt {
    fn eq(&self, val: &i64) -> bool {
        self.0 == *val
    }
}

impl PartialEq<i128> for DoubleInt {
    fn eq(&self, val: &i128) -> bool {
        match val {
            // self cannot be larger than UMAX so val is not equal
            DoubleInt::MAX.. => false,

            // all remaining u64s would be representable by i64
            // just cast and check equality
            _ => self.0 == *val as i64,
        }
    }
}

impl<'de> Deserialize<'de> for DoubleInt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match i64::deserialize(deserializer) {
            Err(err) => Err(err),

            Ok(val) if (val as i128) < DoubleInt::MIN => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(val),
                &"integer larger than -9007199254740991 / -(2^53) + 1",
            )),

            Ok(val) if (val as i128) > DoubleInt::MAX => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(val),
                &"integer smaller than 9007199254740991 / (2^53) - 1",
            )),

            Ok(val) => Ok(DoubleInt(val)),
        }
    }
}

impl Serialize for DoubleInt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.0)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[derive(Debug, Deserialize, PartialEq)]
//     struct Tru {
//         foo: DoubleInt,
//     }

//     #[test]
//     fn de_true() {
//         assert_eq!(
//             Tru { foo: DoubleInt },
//             serde_json::from_str::<Tru>(r#"{"foo": true}"#).unwrap(),
//         );

//         serde_json::from_str::<Tru>(r#"{"foo": false}"#).unwrap_err();
//         serde_json::from_str::<Tru>(r#"{"foo": 42}"#).unwrap_err();
//     }

//     #[derive(Debug, Deserialize, PartialEq)]
//     struct Fal {
//         foo: False,
//     }

//     #[test]
//     fn de_false() {
//         assert_eq!(
//             Fal { foo: False },
//             serde_json::from_str::<Fal>(r#"{"foo": false}"#).unwrap(),
//         );

//         serde_json::from_str::<Fal>(r#"{"foo": true}"#).unwrap_err();
//         serde_json::from_str::<Fal>(r#"{"foo": 42}"#).unwrap_err();
//     }

//     #[test]
//     fn ser() {
//         assert_eq!("true", serde_json::to_string(&DoubleInt).unwrap());
//         assert_eq!("false", serde_json::to_string(&False).unwrap());
//     }

//     #[test]
//     fn as_bool() {
//         assert!(DoubleInt.as_bool());
//         assert!(!False.as_bool());
//     }

//     #[test]
//     fn from() {
//         assert!(bool::from(DoubleInt));
//         assert!(!bool::from(False));
//     }

//     #[test]
//     fn eq() {
//         assert_eq!(DoubleInt, DoubleInt);
//         assert_eq!(DoubleInt, true);
//         assert_eq!(true, DoubleInt);
//         assert_eq!(False, False);
//         assert_eq!(False, false);
//         assert_eq!(false, False);

//         assert_ne!(DoubleInt, False);
//         assert_ne!(DoubleInt, false);
//         assert_ne!(False, DoubleInt);
//         assert_ne!(false, DoubleInt);

//         assert_ne!(False, DoubleInt);
//         assert_ne!(False, true);
//         assert_ne!(DoubleInt, False);
//         assert_ne!(true, False);
//     }

//     #[test]
//     fn formatting() {
//         let _ = format_args!("{:?}", DoubleInt);
//         let _ = format_args!("{:?}", False);
//     }

//     #[test]
//     fn other_implementations() {
//         #![allow(clippy::default_constructed_unit_structs)]

//         assert_eq!(DoubleInt.clone(), DoubleInt);
//         assert_eq!(False.clone(), False);

//         assert_eq!(DoubleInt::default(), DoubleInt);
//         assert_eq!(False::default(), False);
//     }
// }

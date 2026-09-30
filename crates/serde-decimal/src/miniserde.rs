//! Decimal wrappers for miniserde models.
//!
//! Use [`Str`] to preserve decimal precision in a JSON string, or [`Float`] for a
//! JSON number. Both accept strings and numbers during deserialization.
//! Numeric input and [`Float`] output can lose precision because miniserde uses
//! `f64` for non-integer JSON numbers. Arbitrary-precision JSON numbers are not supported.
//!
//! The nullable, non-required, and double-option wrappers keep the same
//! deserialization rules as the Serde adapters. Miniserde's derived serializer
//! writes absent optional values as null. To omit a field, implement
//! `miniserde::ser::Map` and skip the field when its outer option is `None`.
//!
//! ```
//! use serde_decimal::miniserde::{DoubleOptionStr, NullableFloat};
//!
//! #[derive(miniserde::Deserialize, miniserde::Serialize)]
//! struct Prices {
//!     price: NullableFloat,
//!     discount: DoubleOptionStr,
//! }
//!
//! let prices: Prices = miniserde::json::from_str(r#"{"price":null}"#).unwrap();
//! assert_eq!(prices.price.0, None);
//! assert_eq!(prices.discount.0, None);
//! ```

use std::{borrow::Cow, str::FromStr as _};

use ::miniserde::{de::Visitor, ser::Fragment, Deserialize, Error, Result, Serialize};
use rust_decimal::{
    prelude::{FromPrimitive as _, ToPrimitive as _},
    Decimal,
};

::miniserde::make_place!(Place);

/// Serializes a required decimal as a JSON string.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Str(pub Decimal);

/// Serializes a required decimal as a JSON number, with possible precision loss.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Float(pub Decimal);

/// A required field that accepts null and serializes decimals as JSON strings.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NullableStr(pub Option<Decimal>);

/// A required field that accepts null and serializes decimals as JSON numbers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NullableFloat(pub Option<Decimal>);

/// A field that can be missing, rejects null, and serializes decimals as JSON strings.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NonRequiredStr(pub Option<Decimal>);

/// A field that can be missing, rejects null, and serializes decimals as JSON numbers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NonRequiredFloat(pub Option<Decimal>);

/// A string-form decimal field that distinguishes missing, null, and a value.
///
/// Missing fields produce `None`; null produces `Some(None)`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DoubleOptionStr(pub Option<Option<Decimal>>);

/// A number-form decimal field that distinguishes missing, null, and a value.
///
/// Missing fields produce `None`; null produces `Some(None)`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DoubleOptionFloat(pub Option<Option<Decimal>>);

fn serialize_str(value: Decimal) -> Fragment<'static> {
    Fragment::Str(Cow::Owned(value.to_string()))
}

fn serialize_float(value: Decimal) -> Fragment<'static> {
    // Every Decimal is finite and within the range of f64.
    Fragment::F64(value.to_f64().expect("Decimal fits in f64"))
}

macro_rules! decimal_impls {
    ($ty:ident, $wrap:expr, $unwrap:expr, $serialize:ident $(, $default:expr)? $(; $null:expr)?) => {
        impl Deserialize for $ty {
            fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
                Place::new(out)
            }

            $(
                fn default() -> Option<Self> {
                    Some($default)
                }
            )?
        }

        impl Visitor for Place<$ty> {
            $(
                fn null(&mut self) -> Result<()> {
                    self.out = Some($null);

                    Ok(())
                }
            )?

            fn string(&mut self, value: &str) -> Result<()> {
                let value = Decimal::from_str(value)
                    .or_else(|_| Decimal::from_scientific(value))
                    .map_err(|_| Error)?;
                self.out = Some(($wrap)(value));

                Ok(())
            }

            fn negative(&mut self, value: i64) -> Result<()> {
                self.out = Some(($wrap)(Decimal::from(value)));

                Ok(())
            }

            fn nonnegative(&mut self, value: u64) -> Result<()> {
                self.out = Some(($wrap)(Decimal::from(value)));

                Ok(())
            }

            fn float(&mut self, value: f64) -> Result<()> {
                let value = Decimal::from_f64(value).ok_or(Error)?;
                self.out = Some(($wrap)(value));

                Ok(())
            }
        }

        impl Serialize for $ty {
            fn begin(&self) -> Fragment<'_> {
                match ($unwrap)(self) {
                    Some(value) => $serialize(value),
                    None => Fragment::Null,
                }
            }
        }
    };
}

decimal_impls!(Str, Str, |value: &Str| Some(value.0), serialize_str);
decimal_impls!(Float, Float, |value: &Float| Some(value.0), serialize_float);
decimal_impls!(
    NullableStr,
    |value| NullableStr(Some(value)),
    |value: &NullableStr| value.0,
    serialize_str; NullableStr(None)
);
decimal_impls!(
    NullableFloat,
    |value| NullableFloat(Some(value)),
    |value: &NullableFloat| value.0,
    serialize_float; NullableFloat(None)
);
decimal_impls!(
    NonRequiredStr,
    |value| NonRequiredStr(Some(value)),
    |value: &NonRequiredStr| value.0,
    serialize_str,
    NonRequiredStr(None)
);
decimal_impls!(
    NonRequiredFloat,
    |value| NonRequiredFloat(Some(value)),
    |value: &NonRequiredFloat| value.0,
    serialize_float,
    NonRequiredFloat(None)
);
decimal_impls!(
    DoubleOptionStr,
    |value| DoubleOptionStr(Some(Some(value))),
    |value: &DoubleOptionStr| value.0.flatten(),
    serialize_str,
    DoubleOptionStr(None); DoubleOptionStr(Some(None))
);
decimal_impls!(
    DoubleOptionFloat,
    |value| DoubleOptionFloat(Some(Some(value))),
    |value: &DoubleOptionFloat| value.0.flatten(),
    serialize_float,
    DoubleOptionFloat(None); DoubleOptionFloat(Some(None))
);

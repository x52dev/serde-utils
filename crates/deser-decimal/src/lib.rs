//! Decimal adapters for `deser`.
//!
//! Each representation supports three field rules:
//! - `Nullable`: the field is required and accepts `null`.
//! - `NonRequired`: the field may be missing but rejects `null`.
//! - `DoubleOption`: missing, `null`, and a value remain distinct.
//!
//! Use `Str` for strings, `Float` for floating-point numbers, and
//! `ArbitraryPrecision` for numbers that retain decimal precision.
//!
//! Add `#[deser(skip_serializing_if = Option::is_none)]` to omit absent fields.
//!
//! ```
//! #[derive(Debug, deser::Serialize, deser::Deserialize)]
//! struct Price {
//!     #[deser(as = deser_decimal::DoubleOptionStr)]
//!     #[deser(skip_serializing_if = Option::is_none)]
//!     amount: Option<Option<rust_decimal::Decimal>>,
//! }
//!
//! let missing = deser_json::from_str::<Price>("{}").unwrap();
//! assert_eq!(missing.amount, None);
//!
//! let null = deser_json::from_str::<Price>(r#"{"amount":null}"#).unwrap();
//! assert_eq!(null.amount, Some(None));
//! assert_eq!(deser_json::to_string(&null).unwrap(), r#"{"amount":null}"#);
//! ```

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use std::{marker::PhantomData, string::ToString as _};

use deser::{
    adapters::{DeserializeAs, SerializeAs},
    de::SinkHandle,
    ser::Chunk,
    Atom, Deserialize as _, Error, ErrorKind, Serialize, State,
};
use rust_decimal::{prelude::ToPrimitive as _, Decimal};

/// Serializes decimals as strings.
///
/// Accepts strings, integers, and floating-point numbers during deserialization.
pub struct Str;

/// Serializes decimals as floating-point numbers.
///
/// Accepts strings, integers, and floating-point numbers during deserialization.
pub struct Float;

/// Serializes decimals as numbers without loss of decimal precision.
///
/// Accepts strings, integers, and floating-point numbers during deserialization.
pub struct ArbitraryPrecision;

impl SerializeAs<Decimal> for Str {
    fn serialize_as<'a>(value: &'a Decimal, _state: &mut State) -> Result<Chunk<'a>, Error> {
        Ok(Chunk::Atom(Atom::Str(value.to_string().into())))
    }
}

impl SerializeAs<Decimal> for Float {
    fn serialize_as<'a>(value: &'a Decimal, _state: &mut State) -> Result<Chunk<'a>, Error> {
        let value = value.to_f64().ok_or_else(|| {
            Error::new(
                ErrorKind::Unexpected,
                "decimal cannot be represented as a float",
            )
        })?;

        Ok(Chunk::Atom(Atom::F64(value)))
    }
}

impl SerializeAs<Decimal> for ArbitraryPrecision {
    fn serialize_as<'a>(value: &'a Decimal, state: &mut State) -> Result<Chunk<'a>, Error> {
        Serialize::serialize(value, state)
    }
}

macro_rules! deserialize_decimal {
    ($adapter:ty) => {
        impl<'de> DeserializeAs<'de, Decimal> for $adapter {
            fn deserialize_into_as<'out>(
                out: &'out mut Option<Decimal>,
                state: &mut State,
            ) -> SinkHandle<'out, 'de> {
                Decimal::deserialize_into(out, state)
            }
        }
    };
}

deserialize_decimal!(Str);
deserialize_decimal!(Float);
deserialize_decimal!(ArbitraryPrecision);

/// Requires a decimal field and accepts `null`, using representation `A`.
pub struct Nullable<A>(PhantomData<A>);

/// Allows a missing decimal field and rejects `null`, using representation `A`.
pub struct NonRequired<A>(PhantomData<A>);

/// Keeps missing, null, and present decimal fields distinct, using representation `A`.
pub struct DoubleOption<A>(PhantomData<A>);

impl<'de, A: DeserializeAs<'de, Decimal>> DeserializeAs<'de, Option<Decimal>> for Nullable<A> {
    fn deserialize_into_as<'out>(
        out: &'out mut Option<Option<Decimal>>,
        state: &mut State,
    ) -> SinkHandle<'out, 'de> {
        A::deserialize_into_as(out.insert(None), state).ignore_null()
    }
}

impl<'de, A: DeserializeAs<'de, Decimal>> DeserializeAs<'de, Option<Decimal>> for NonRequired<A> {
    fn deserialize_into_as<'out>(
        out: &'out mut Option<Option<Decimal>>,
        state: &mut State,
    ) -> SinkHandle<'out, 'de> {
        A::deserialize_into_as(out.insert(None), state)
    }

    fn initial_value_as() -> Option<Option<Decimal>> {
        Some(None)
    }
}

impl<'de, A: DeserializeAs<'de, Decimal>> DeserializeAs<'de, Option<Option<Decimal>>>
    for DoubleOption<A>
{
    fn deserialize_into_as<'out>(
        out: &'out mut Option<Option<Option<Decimal>>>,
        state: &mut State,
    ) -> SinkHandle<'out, 'de> {
        A::deserialize_into_as(out.insert(None).insert(None), state).ignore_null()
    }

    fn initial_value_as() -> Option<Option<Option<Decimal>>> {
        Some(None)
    }
}

macro_rules! serialize_option_decimal {
    ($adapter:ident) => {
        impl<A: SerializeAs<Decimal>> SerializeAs<Option<Decimal>> for $adapter<A> {
            fn serialize_as<'a>(
                value: &'a Option<Decimal>,
                state: &mut State,
            ) -> Result<Chunk<'a>, Error> {
                match value {
                    Some(value) => A::serialize_as(value, state),
                    None => Ok(Chunk::Atom(Atom::Null)),
                }
            }

            fn is_optional_as(value: &Option<Decimal>) -> bool {
                value.is_none()
            }

            fn finish_as(value: &Option<Decimal>, state: &mut State) -> Result<(), Error> {
                match value {
                    Some(value) => A::finish_as(value, state),
                    None => Ok(()),
                }
            }
        }
    };
}

serialize_option_decimal!(Nullable);
serialize_option_decimal!(NonRequired);

impl<A: SerializeAs<Decimal>> SerializeAs<Option<Option<Decimal>>> for DoubleOption<A> {
    fn serialize_as<'a>(
        value: &'a Option<Option<Decimal>>,
        state: &mut State,
    ) -> Result<Chunk<'a>, Error> {
        match value {
            Some(Some(value)) => A::serialize_as(value, state),
            _ => Ok(Chunk::Atom(Atom::Null)),
        }
    }

    fn is_optional_as(value: &Option<Option<Decimal>>) -> bool {
        value.is_none()
    }

    fn finish_as(value: &Option<Option<Decimal>>, state: &mut State) -> Result<(), Error> {
        match value {
            Some(Some(value)) => A::finish_as(value, state),
            _ => Ok(()),
        }
    }
}

/// Required, nullable decimals serialized as strings.
pub type NullableStr = Nullable<Str>;

/// Optional, non-nullable decimals serialized as strings.
pub type NonRequiredStr = NonRequired<Str>;

/// Missing, null, or present decimals serialized as strings.
pub type DoubleOptionStr = DoubleOption<Str>;

/// Required, nullable decimals serialized as floating-point numbers.
pub type NullableFloat = Nullable<Float>;

/// Optional, non-nullable decimals serialized as floating-point numbers.
pub type NonRequiredFloat = NonRequired<Float>;

/// Missing, null, or present decimals serialized as floating-point numbers.
pub type DoubleOptionFloat = DoubleOption<Float>;

/// Required, nullable decimals serialized as numbers with decimal precision.
pub type NullableArbitraryPrecision = Nullable<ArbitraryPrecision>;

/// Optional, non-nullable decimals serialized as numbers with decimal precision.
pub type NonRequiredArbitraryPrecision = NonRequired<ArbitraryPrecision>;

/// Missing, null, or present decimals serialized as numbers with decimal precision.
pub type DoubleOptionArbitraryPrecision = DoubleOption<ArbitraryPrecision>;

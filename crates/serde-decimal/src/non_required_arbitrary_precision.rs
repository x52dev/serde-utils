//! Serialization and deserialization of not required but not nullable decimals as numbers with arbitrary precision.

use std::fmt;

use rust_decimal::Decimal;
use serde_core::{de, Deserialize, Deserializer, Serializer};

struct OptionDecimalVisitor;

impl<'de> de::Visitor<'de> for OptionDecimalVisitor {
    type Value = Option<Decimal>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Decimal type representing a fixed-point number")
    }

    fn visit_some<D>(self, d: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        <Decimal as Deserialize>::deserialize(d).map(Some)
    }
}

/// Non-required arbitrary-precision decimal deserializer.
///
/// See [module docs](self) for more.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<rust_decimal::Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(OptionDecimalVisitor)
}

/// Non-required arbitrary-precision decimal serializer.
///
/// See [module docs](self) for more.
pub fn serialize<S>(value: &Option<rust_decimal::Decimal>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    rust_decimal::serde::arbitrary_precision_option::serialize(value, serializer)
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    struct Foo {
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(with = "crate::non_required_arbitrary_precision")]
        foo: Option<rust_decimal::Decimal>,
    }

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    struct Bar {
        #[serde(flatten)]
        foo: Foo,
    }

    #[test]
    fn foo_serialize_some() {
        let serialized = serde_json::to_string(&Foo {
            foo: Some(dec!(0.1)),
        })
        .unwrap();
        assert_eq!(serialized, r#"{"foo":0.1}"#);
    }

    #[test]
    fn foo_serialize_none() {
        let serialized = serde_json::to_string(&Foo { foo: None }).unwrap();
        assert_eq!(serialized, r#"{}"#);
    }

    #[test]
    fn foo_deserialize_some() {
        let deserialized: Foo = serde_json::from_str(r#"{"foo":0.1}"#).unwrap();
        assert!(matches!(deserialized, Foo { foo: Some(_) }));
    }

    #[test]
    fn foo_deserialize_missing() {
        let deserialized: Foo = serde_json::from_str(r#"{}"#).unwrap();
        assert!(matches!(deserialized, Foo { foo: None }));
    }

    #[test]
    #[should_panic]
    fn foo_deserialize_null() {
        serde_json::from_str::<Foo>(r#"{"foo": null}"#).unwrap();
    }

    #[test]
    fn bar_serialize_some() {
        let serialized = serde_json::to_string(&Bar {
            foo: Foo {
                foo: Some(dec!(0.1)),
            },
        })
        .unwrap();
        assert_eq!(serialized, r#"{"foo":0.1}"#);
    }

    #[test]
    fn bar_serialize_none() {
        let serialized = serde_json::to_string(&Bar {
            foo: Foo { foo: None },
        })
        .unwrap();
        assert_eq!(serialized, r#"{}"#);
    }

    #[test]
    fn bar_deserialize_some() {
        let deserialized: Bar = serde_json::from_str(r#"{"foo":0.1}"#).unwrap();
        assert!(matches!(
            deserialized,
            Bar {
                foo: Foo { foo: Some(_) }
            }
        ));
    }

    #[test]
    fn bar_deserialize_missing() {
        let deserialized: Bar = serde_json::from_str(r#"{}"#).unwrap();
        assert!(matches!(
            deserialized,
            Bar {
                foo: Foo { foo: None }
            }
        ));
    }

    #[test]
    #[should_panic]
    fn bar_deserialize_null() {
        serde_json::from_str::<Bar>(r#"{"foo": null}"#).unwrap();
    }
}

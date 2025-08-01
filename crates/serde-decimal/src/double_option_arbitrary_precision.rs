//! Combination of the serde rules from [`rust_decimal::serde::arbitrary_precision`] and
//! `serde_with::rust::double_option`.
//!
//! This is necessary because it is not possible to apply multiple `#[serde(with = ...)]` attributes:
//! * `#[serde(with = "serde_with::rust::double_option")]`
//! * `#[serde(with = "rust_decimal::serde::arbitrary_precision")]`

/// Double-option arbitrary-precision decimal deserializer.
///
/// See [module docs](self) for more.
pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<Option<rust_decimal::Decimal>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    crate::nullable_arbitrary_precision::deserialize(deserializer).map(Some)
}

/// Double-option arbitrary-precision decimal serializer.
///
/// See [module docs](self) for more.
pub fn serialize<S>(
    value: &Option<Option<rust_decimal::Decimal>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        None => serializer.serialize_unit(),
        Some(v) => crate::nullable_arbitrary_precision::serialize(v, serializer),
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    struct Foo {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::double_option_arbitrary_precision")]
        foo: Option<Option<rust_decimal::Decimal>>,
    }

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    struct Bar {
        #[serde(flatten)]
        foo: Foo,
    }

    #[test]
    fn foo_serialize_some_some() {
        let serialized = serde_json::to_string(&Foo {
            foo: Some(Some(dec!(0.1))),
        })
        .unwrap();
        assert_eq!(serialized, r#"{"foo":0.1}"#);
    }

    #[test]
    fn foo_serialize_some_none() {
        let serialized = serde_json::to_string(&Foo { foo: Some(None) }).unwrap();
        assert_eq!(serialized, r#"{"foo":null}"#);
    }

    #[test]
    fn foo_serialize_none() {
        let serialized = serde_json::to_string(&Foo { foo: None }).unwrap();
        assert_eq!(serialized, r#"{}"#);
    }

    #[test]
    fn foo_deserialize_value() {
        let deserialized: Foo = serde_json::from_str(r#"{"foo":0.1}"#).unwrap();
        assert!(matches!(deserialized, Foo { foo: Some(Some(_)) }));
    }

    #[test]
    fn foo_deserialize_null() {
        let deserialized: Foo = serde_json::from_str(r#"{"foo":null}"#).unwrap();
        assert!(matches!(deserialized, Foo { foo: Some(None) }));
    }

    #[test]
    fn foo_deserialize_missing() {
        let deserialized: Foo = serde_json::from_str(r#"{}"#).unwrap();
        assert!(matches!(deserialized, Foo { foo: None }));
    }

    #[test]
    fn bar_serialize_some_some() {
        let serialized = serde_json::to_string(&Bar {
            foo: Foo {
                foo: Some(Some(dec!(0.1))),
            },
        })
        .unwrap();
        assert_eq!(serialized, r#"{"foo":0.1}"#);
    }

    #[test]
    fn bar_serialize_some_none() {
        let serialized = serde_json::to_string(&Bar {
            foo: Foo { foo: Some(None) },
        })
        .unwrap();
        assert_eq!(serialized, r#"{"foo":null}"#);
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
    fn bar_deserialize_value() {
        let deserialized: Bar = serde_json::from_str(r#"{"foo":0.1}"#).unwrap();
        assert!(matches!(
            deserialized,
            Bar {
                foo: Foo { foo: Some(Some(_)) }
            }
        ));
    }

    #[test]
    fn bar_deserialize_null() {
        let deserialized: Bar = serde_json::from_str(r#"{"foo":null}"#).unwrap();
        assert!(matches!(
            deserialized,
            Bar {
                foo: Foo { foo: Some(None) }
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
}

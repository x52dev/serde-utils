//! Tests for deser support.

#![cfg(feature = "deser")]

use double_int::DoubleInt;

#[test]
fn integer_bounds_and_roundtrip() {
    for value in [-9007199254740991_i64, -42, 0, 42, 9007199254740991] {
        let json = value.to_string();
        let parsed = deser_json::from_str::<DoubleInt>(&json).unwrap();

        assert_eq!(parsed.as_i64(), value);
        assert_eq!(deser_json::to_string(&parsed).unwrap(), json);
    }
}

#[test]
fn invalid_integers() {
    for input in [
        "-9007199254740992",
        "9007199254740992",
        "18446744073709551615",
        "4.2",
        "42.0",
        "true",
        "null",
        r#""42""#,
        "[]",
        "{}",
    ] {
        assert!(deser_json::from_str::<DoubleInt>(input).is_err(), "{input}");
    }
}

#[test]
fn derived_struct_fields() {
    #[derive(Debug, deser::Serialize, deser::Deserialize, PartialEq)]
    struct Count {
        count: DoubleInt,
    }

    let value = Count {
        count: DoubleInt::from(42_i32),
    };
    let json = r#"{"count":42}"#;

    assert_eq!(deser_json::from_str::<Count>(json).unwrap(), value);
    assert_eq!(deser_json::to_string(&value).unwrap(), json);
    assert!(deser_json::from_str::<Count>("{}").is_err());
}

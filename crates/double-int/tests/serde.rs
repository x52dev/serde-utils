//! Tests for optional Serde support.

#![cfg(feature = "serde")]

use double_int::DoubleInt;

#[test]
fn integer_bounds_and_roundtrip() {
    for value in [-9007199254740991_i64, -42, 0, 42, 9007199254740991] {
        let json = value.to_string();
        let parsed = serde_json::from_str::<DoubleInt>(&json).unwrap();

        assert_eq!(parsed.as_i64(), value);
        assert_eq!(serde_json::to_string(&parsed).unwrap(), json);
    }
}

#[test]
fn invalid_integers() {
    for input in [
        "-9007199254740992",
        "9007199254740992",
        "18446744073709551615",
        "4.2",
        "true",
        "null",
        r#""42""#,
        "[]",
        "{}",
    ] {
        assert!(serde_json::from_str::<DoubleInt>(input).is_err(), "{input}");
    }
}

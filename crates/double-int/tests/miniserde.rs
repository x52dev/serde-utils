#![cfg(feature = "miniserde")]

use double_int::DoubleInt;
use miniserde::json;

#[test]
fn round_trips_integers_at_the_bounds() {
    for input in ["-9007199254740991", "-42", "0", "42", "9007199254740991"] {
        let value: DoubleInt = json::from_str(input).unwrap();

        assert_eq!(value.as_i64(), input.parse::<i64>().unwrap());
        assert_eq!(json::to_string(&value), input);
    }
}

#[test]
fn rejects_integers_outside_the_bounds_and_other_types() {
    for input in [
        "-9007199254740992",
        "9007199254740992",
        "-9223372036854775808",
        "18446744073709551615",
        "4.2",
        "42.0",
        "null",
        "true",
        "\"42\"",
        "[]",
        "{}",
    ] {
        assert!(json::from_str::<DoubleInt>(input).is_err(), "{input}");
    }
}

#[test]
fn supports_derived_structs() {
    #[derive(miniserde::Serialize, miniserde::Deserialize)]
    struct Counter {
        count: DoubleInt,
    }

    let counter: Counter = json::from_str(r#"{"count":42}"#).unwrap();

    assert_eq!(counter.count, 42);
    assert_eq!(json::to_string(&counter), r#"{"count":42}"#);
    assert!(json::from_str::<Counter>("{}").is_err());
}

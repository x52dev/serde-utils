//! Tests for the miniserde decimal wrappers.

#![cfg(feature = "miniserde")]

use miniserde::json;
use rust_decimal_macros::dec;
use serde_decimal::miniserde::*;

#[test]
fn preserves_string_precision_and_scale() {
    for input in [
        r#""0.1234567890123456789012345678""#,
        r#""79228162514264337593543950335""#,
        r#""123.4500""#,
    ] {
        let value: Str = json::from_str(input).unwrap();

        assert_eq!(json::to_string(&value), input);
    }
}

#[test]
fn accepts_integer_float_and_scientific_input() {
    for (input, expected) in [
        ("42", dec!(42)),
        ("-42", dec!(-42)),
        ("18446744073709551615", dec!(18446744073709551615)),
        ("0.1", dec!(0.1)),
        ("1e-2", dec!(0.01)),
        (r#""1e-28""#, dec!(0.0000000000000000000000000001)),
    ] {
        assert_eq!(json::from_str::<Str>(input).unwrap().0, expected);
        assert_eq!(json::from_str::<Float>(input).unwrap().0, expected);
    }
}

#[test]
fn serializes_strings_and_numbers() {
    assert_eq!(json::to_string(&Str(dec!(0.1))), r#""0.1""#);
    assert_eq!(json::to_string(&Float(dec!(0.1))), "0.1");
    assert!(json::to_string(&Float(dec!(79228162514264337593543950335)))
        .parse::<f64>()
        .unwrap()
        .is_finite());
}

#[test]
fn rejects_invalid_decimals_and_other_types() {
    for input in ["null", "true", "[]", "{}", r#""invalid""#, "1e100"] {
        assert!(json::from_str::<Str>(input).is_err(), "{input}");
        assert!(json::from_str::<Float>(input).is_err(), "{input}");
    }
}

macro_rules! option_tests {
    ($module:ident, $nullable:ty, $non_required:ty, $double:ty, $input:literal, $output:literal) => {
        mod $module {
            use super::*;

            #[derive(miniserde::Deserialize, miniserde::Serialize)]
            struct Nullable {
                value: $nullable,
            }

            #[derive(miniserde::Deserialize, miniserde::Serialize)]
            struct NonRequired {
                value: $non_required,
            }

            #[derive(miniserde::Deserialize, miniserde::Serialize)]
            struct DoubleOption {
                value: $double,
            }

            #[test]
            fn nullable_requires_a_field_and_accepts_null() {
                assert!(json::from_str::<Nullable>("{}").is_err());

                let value: Nullable = json::from_str(r#"{"value":null}"#).unwrap();

                assert_eq!(value.value.0, None);
                assert_eq!(json::to_string(&value), r#"{"value":null}"#);

                let value: Nullable = json::from_str($input).unwrap();

                assert_eq!(value.value.0, Some(dec!(0.1)));
                assert_eq!(json::to_string(&value), $output);
            }

            #[test]
            fn non_required_accepts_missing_and_rejects_null() {
                let value: NonRequired = json::from_str("{}").unwrap();

                assert_eq!(value.value.0, None);
                assert_eq!(json::to_string(&value), r#"{"value":null}"#);
                assert!(json::from_str::<NonRequired>(r#"{"value":null}"#).is_err());

                let value: NonRequired = json::from_str($input).unwrap();

                assert_eq!(value.value.0, Some(dec!(0.1)));
                assert_eq!(json::to_string(&value), $output);
            }

            #[test]
            fn double_option_distinguishes_missing_null_and_value() {
                let value: DoubleOption = json::from_str("{}").unwrap();

                assert_eq!(value.value.0, None);
                assert_eq!(json::to_string(&value), r#"{"value":null}"#);

                let value: DoubleOption = json::from_str(r#"{"value":null}"#).unwrap();

                assert_eq!(value.value.0, Some(None));
                assert_eq!(json::to_string(&value), r#"{"value":null}"#);

                let value: DoubleOption = json::from_str($input).unwrap();

                assert_eq!(value.value.0, Some(Some(dec!(0.1))));
                assert_eq!(json::to_string(&value), $output);
            }
        }
    };
}

option_tests!(
    string,
    NullableStr,
    NonRequiredStr,
    DoubleOptionStr,
    r#"{"value":"0.1"}"#,
    r#"{"value":"0.1"}"#
);
option_tests!(
    float,
    NullableFloat,
    NonRequiredFloat,
    DoubleOptionFloat,
    r#"{"value":0.1}"#,
    r#"{"value":0.1}"#
);

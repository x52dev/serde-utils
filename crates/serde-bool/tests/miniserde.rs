#![cfg(feature = "miniserde")]

use miniserde::json;
use serde_bool::{False, True};

#[test]
fn accepts_only_the_matching_boolean() {
    assert_eq!(json::from_str::<True>("true").unwrap(), True);
    assert_eq!(json::from_str::<False>("false").unwrap(), False);

    for input in ["false", "null", "42", "\"true\"", "[]", "{}"] {
        assert!(json::from_str::<True>(input).is_err(), "{input}");
    }

    for input in ["true", "null", "42", "\"false\"", "[]", "{}"] {
        assert!(json::from_str::<False>(input).is_err(), "{input}");
    }
}

#[test]
fn serializes_boolean_fields() {
    #[derive(miniserde::Serialize, miniserde::Deserialize)]
    struct Flags {
        enabled: True,
        disabled: False,
    }

    let flags: Flags = json::from_str(r#"{"enabled":true,"disabled":false}"#).unwrap();

    assert_eq!(
        json::to_string(&flags),
        r#"{"enabled":true,"disabled":false}"#
    );
    assert!(json::from_str::<Flags>(r#"{"enabled":true}"#).is_err());
}

//! Tests for deser support.

use deser_bool::{False, True};

#[test]
fn single_boolean_values() {
    assert_eq!(deser_json::from_str::<True>("true").unwrap(), True);
    assert_eq!(deser_json::from_str::<False>("false").unwrap(), False);

    for input in ["false", "null", "0", "[]", "{}", r#""true""#] {
        assert!(deser_json::from_str::<True>(input).is_err(), "{input}");
    }

    for input in ["true", "null", "0", "[]", "{}", r#""false""#] {
        assert!(deser_json::from_str::<False>(input).is_err(), "{input}");
    }

    assert_eq!(deser_json::to_string(&True).unwrap(), "true");
    assert_eq!(deser_json::to_string(&False).unwrap(), "false");
}

#[test]
fn derived_struct_fields() {
    #[derive(Debug, deser::Serialize, deser::Deserialize, PartialEq)]
    struct Flags {
        enabled: True,
        disabled: False,
    }

    let flags = Flags {
        enabled: True,
        disabled: False,
    };
    let json = r#"{"enabled":true,"disabled":false}"#;

    assert_eq!(deser_json::from_str::<Flags>(json).unwrap(), flags);
    assert_eq!(deser_json::to_string(&flags).unwrap(), json);
    assert!(deser_json::from_str::<Flags>(r#"{"enabled":true}"#).is_err());
}

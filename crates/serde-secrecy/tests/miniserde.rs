//! Tests for the miniserde secret string wrapper.

#![cfg(feature = "miniserde")]

use miniserde::json;
use secrecy::{ExposeSecret as _, SecretString};
use serde_secrecy::miniserde::ExposeSecretString;

#[test]
fn serializes_secrets_and_keeps_debug_output_redacted() {
    #[derive(Debug, miniserde::Serialize, miniserde::Deserialize)]
    struct Login {
        email: String,
        password: ExposeSecretString,
    }

    let value = Login {
        email: "foo@example.com".to_owned(),
        password: ExposeSecretString(SecretString::from("hunter2")),
    };

    assert!(!format!("{value:?}").contains("hunter2"));
    assert_eq!(
        json::to_string(&value),
        r#"{"email":"foo@example.com","password":"hunter2"}"#
    );

    let value: Login = json::from_str(&json::to_string(&value)).unwrap();

    assert_eq!(value.password.0.expose_secret(), "hunter2");
    assert!(!format!("{value:?}").contains("hunter2"));
}

#[test]
fn supports_escaped_and_empty_secrets() {
    for input in [r#""""#, r#""line\n\"quote\"""#] {
        let value: ExposeSecretString = json::from_str(input).unwrap();

        assert_eq!(json::to_string(&value), input);
    }
}

#[test]
fn rejects_other_types_and_missing_secrets() {
    for input in ["null", "true", "42", "[]", "{}"] {
        assert!(
            json::from_str::<ExposeSecretString>(input).is_err(),
            "{input}"
        );
    }

    #[derive(miniserde::Deserialize)]
    struct Login {
        _password: ExposeSecretString,
    }

    assert!(json::from_str::<Login>("{}").is_err());
}

//! Tests for deser support.

use deser_secrecy::ExposeSecretString;
use secrecy::SecretString;

#[test]
fn expose_secret_only_for_serialization() {
    #[derive(Debug, deser::Serialize)]
    struct Login {
        email: String,
        #[deser(serialize_as = ExposeSecretString)]
        password: SecretString,
    }

    let login = Login {
        email: "foo@example.com".to_owned(),
        password: SecretString::from("hunter2"),
    };

    assert!(!format!("{login:?}").contains("hunter2"));
    assert_eq!(
        deser_json::to_string(&login).unwrap(),
        r#"{"email":"foo@example.com","password":"hunter2"}"#,
    );
}

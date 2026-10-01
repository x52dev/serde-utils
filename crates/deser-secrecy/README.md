# `deser-secrecy`

Explicit serialization of `secrecy::SecretString` with Deser 0.9.

```rust
#[derive(deser::Serialize)]
struct Login {
    #[deser(serialize_as = deser_secrecy::ExposeSecretString)]
    password: secrecy::SecretString,
}

let login = Login {
    password: secrecy::SecretString::from("hunter2"),
};

assert_eq!(
    deser_json::to_string(&login).unwrap(),
    r#"{"password":"hunter2"}"#,
);
```

Requires Rust 1.88 or later.

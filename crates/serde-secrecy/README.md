# `serde-secrecy`

<!-- prettier-ignore-start -->

[![crates.io](https://img.shields.io/crates/v/serde-secrecy?label=latest)](https://crates.io/crates/serde-secrecy)
[![Documentation](https://docs.rs/serde-secrecy/badge.svg?version=0.2.6)](https://docs.rs/serde-secrecy/0.2.6)
[![dependency status](https://deps.rs/crate/serde-secrecy/0.2.6/status.svg)](https://deps.rs/crate/serde-secrecy/0.2.6)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/serde-secrecy.svg)
<br />
[![CI](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/x52dev/serde-utils/branch/main/graph/badge.svg)](https://codecov.io/gh/x52dev/serde-utils)
![Version](https://img.shields.io/badge/rustc-1.88+-ab6000.svg)
[![Download](https://img.shields.io/crates/d/serde-secrecy.svg)](https://crates.io/crates/serde-secrecy)

<!-- prettier-ignore-end -->

Serde support for `secrecy` types.

Use `expose_secret_string` to serialize a `secrecy::SecretString` field as its inner string. The serialized output contains the secret in plain text.

## Example

```rust
use secrecy::SecretString;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Login {
    #[serde(serialize_with = "serde_secrecy::expose_secret_string")]
    password: SecretString,
}

let login = Login {
    password: SecretString::from("hunter2"),
};

assert_eq!(
    serde_json::to_string(&login).unwrap(),
    r#"{"password":"hunter2"}"#,
);
```

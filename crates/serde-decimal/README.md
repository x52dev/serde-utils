# `serde-decimal`

<!-- prettier-ignore-start -->

[![crates.io](https://img.shields.io/crates/v/serde-decimal?label=latest)](https://crates.io/crates/serde-decimal)
[![Documentation](https://docs.rs/serde-decimal/badge.svg?version=0.2.6)](https://docs.rs/serde-decimal/0.2.6)
[![dependency status](https://deps.rs/crate/serde-decimal/0.2.6/status.svg)](https://deps.rs/crate/serde-decimal/0.2.6)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/serde-decimal.svg)
<br />
[![CI](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/x52dev/serde-utils/branch/main/graph/badge.svg)](https://codecov.io/gh/x52dev/serde-utils)
![Version](https://img.shields.io/badge/rustc-1.88+-ab6000.svg)
[![Download](https://img.shields.io/crates/d/serde-decimal.svg)](https://crates.io/crates/serde-decimal)

<!-- prettier-ignore-end -->

Careful serialization and deserialization of `rust_decimal::Decimal` values with Serde.

Use these adapters with `#[serde(with = "...")]` to control whether a field can be missing or null.

| Adapter           | Field type                | Missing field | Null value |
| ----------------- | ------------------------- | ------------- | ---------- |
| `double_option_*` | `Option<Option<Decimal>>` | Allowed       | Allowed    |
| `non_required_*`  | `Option<Decimal>`         | Allowed       | Rejected   |
| `nullable_*`      | `Option<Decimal>`         | Rejected      | Allowed    |

The `float` adapters use JSON numbers. The `str` adapters use strings. The `arbitrary_precision` adapters require the `rust-decimal-arbitrary-precision` feature.

## Example

```rust
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Price {
    #[serde(with = "serde_decimal::nullable_str")]
    amount: Option<Decimal>,
}

let price: Price = serde_json::from_str(r#"{"amount":"12.50"}"#).unwrap();
assert_eq!(price.amount, Some(Decimal::new(1250, 2)));

let price: Price = serde_json::from_str(r#"{"amount":null}"#).unwrap();
assert_eq!(price.amount, None);

assert!(serde_json::from_str::<Price>("{}").is_err());
```

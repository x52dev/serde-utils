# serde-utils

[![CI](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml)

A collection of [`serde`][serde] and [`deser`][deser] utility crates.

`serde-*` crates support Serde. Separate `deser-*` crates support Deser 0.9. `detrim` and `double-int` have independent, optional `serde` and `deser` features. Serde is enabled by default in `detrim`. Both backends are disabled by default in `double-int`.

For Deser support without Serde:

```toml
[dependencies]
deser-bool = "0.1"
deser-decimal = "0.1"
detrim = { version = "0.1", default-features = false, features = ["std", "deser"] }
double-int = { version = "0.1", features = ["deser"] }
```

| Crate           | Backend                  | Purpose                              |
| --------------- | ------------------------ | ------------------------------------ |
| `serde-bool`    | Serde                    | Fixed-value `True` and `False` types |
| `deser-bool`    | Deser                    | Fixed-value `True` and `False` types |
| `serde-decimal` | Serde                    | Decimal field modules                |
| `deser-decimal` | Deser                    | Decimal field adapters               |
| `serde-secrecy` | Serde                    | Explicit secret exposure             |
| `detrim`        | Optional Serde and Deser | String and collection trimming       |
| `double-int`    | Optional Serde and Deser | Integers within the double-int range |

[serde]: https://crates.io/crates/serde
[deser]: https://crates.io/crates/deser

# serde-utils

[![CI](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml)

A collection of [`serde`][serde] and [`deser`][deser] utility crates.

`serde-*` crates support Serde. `detrim` has independent, optional `serde` and `deser` features. Serde is enabled by default in `detrim`.

For Deser support without Serde:

```toml
[dependencies]
detrim = { version = "0.1", default-features = false, features = ["std", "deser"] }
```

| Crate           | Backend                  | Purpose                              |
| --------------- | ------------------------ | ------------------------------------ |
| `serde-bool`    | Serde                    | Fixed-value `True` and `False` types |
| `serde-decimal` | Serde                    | Decimal field modules                |
| `serde-secrecy` | Serde                    | Explicit secret exposure             |
| `detrim`        | Optional Serde and Deser | String and collection trimming       |
| `double-int`    | Serde                    | Integers within the double-int range |

[serde]: https://crates.io/crates/serde
[deser]: https://crates.io/crates/deser

# serde-utils

[![CI](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml)

A collection of [`serde`][serde] utility crates.

All crates also support [`miniserde`][miniserde] through the optional `miniserde` feature. The `serde` feature is enabled by default. For a miniserde-only build, disable default features and enable `miniserde`:

```toml
serde-bool = { version = "0.1", default-features = false, features = ["miniserde"] }
```

| Crate           | Miniserde API                                                                    |
| --------------- | -------------------------------------------------------------------------------- |
| `serde-bool`    | `True` and `False` implement miniserde traits.                                   |
| `double-int`    | `DoubleInt` implements miniserde traits and checks the same integer bounds.      |
| `detrim`        | `miniserde::Trimmed`, `NonEmptyString`, and `OptionNonEmptyString` wrap strings. |
| `serde-decimal` | `miniserde` contains string and float wrappers with nullable and optional forms. |
| `serde-secrecy` | `miniserde::ExposeSecretString` exposes a secret during serialization.           |

The `detrim` set wrapper also requires the `std` feature. Miniserde allocates even when `std` is disabled. Decimal strings preserve precision; decimal JSON numbers use `f64` and can lose precision. Miniserde's derived serializer writes absent optional fields as null. Use a custom map serializer to omit these fields.

[serde]: https://crates.io/crates/serde
[miniserde]: https://crates.io/crates/miniserde

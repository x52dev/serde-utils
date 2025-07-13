# `double-int`

<!-- prettier-ignore-start -->

[![crates.io](https://img.shields.io/crates/v/double-int?label=latest)](https://crates.io/crates/double-int)
[![Documentation](https://docs.rs/double-int/badge.svg?version=0.1.3)](https://docs.rs/double-int/0.1.3)
[![dependency status](https://deps.rs/crate/double-int/0.1.3/status.svg)](https://deps.rs/crate/double-int/0.1.3)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/double-int.svg)
<br />
[![CI](https://github.com/x52dev/double-int/actions/workflows/ci.yml/badge.svg)](https://github.com/x52dev/double-int/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/x52dev/double-int/branch/main/graph/badge.svg)](https://codecov.io/gh/x52dev/double-int)
![Version](https://img.shields.io/badge/rustc-1.56+-ab6000.svg)
[![Download](https://img.shields.io/crates/d/double-int.svg)](https://crates.io/crates/double-int)

<!-- prettier-ignore-end -->

<!-- cargo-rdme start -->

The double-int format represents an integer that can be stored in an IEEE 754 double-precision number without loss of precision.

This crate has been designed for use with OpenAPI tooling that wish to support integer-based `format: double-int` fields. [See docs in the OpenAPI format registry.][reg_double_int]

## Examples

```rust
#[derive(Debug, serde::Deserialize)]
struct Config {
    count: DoubleInt,
}

let config = toml::from_str::<Config>(r#"
    count = 42
"#).unwrap();
assert_eq!(config.count, 42);

let config = toml::from_str::<Config>(r#"
    count = -42
"#).unwrap();
assert_eq!(config.count, -42);

// count is outside the bounds of a double-int (> 2^53 in this case)
// (this would usually be accepted by an i64)
let config = toml::from_str::<Config>(r#"
    count = 36028797018963968
"#).unwrap_err();
```

[reg_double_int]: https://spec.openapis.org/registry/format/double-int

<!-- cargo-rdme end -->

# `deser-bool`

Fixed-value boolean types for Deser 0.9. `True` accepts only `true`, and `False` accepts only `false`. Both types implement Deser serialization and deserialization.

```rust
#[derive(Debug, deser::Deserialize)]
struct Flags {
    enabled: deser_bool::True,
}

let flags = deser_json::from_str::<Flags>(r#"{"enabled":true}"#).unwrap();
assert!(flags.enabled.as_bool());
assert!(deser_json::from_str::<Flags>(r#"{"enabled":false}"#).is_err());
```

This crate supports `no_std` with an allocator and requires Rust 1.88 or later.

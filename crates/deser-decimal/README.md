# `deser-decimal`

Field adapters for `rust_decimal::Decimal` with Deser 0.9.

Choose `Str`, `Float`, or `ArbitraryPrecision` for the representation, then use `Nullable`, `NonRequired`, or `DoubleOption` for the field rules. Aliases such as `NullableStr` and `DoubleOptionArbitraryPrecision` combine these choices.

```rust
#[derive(Debug, deser::Serialize, deser::Deserialize)]
struct Price {
    #[deser(as = deser_decimal::DoubleOptionStr)]
    #[deser(skip_serializing_if = Option::is_none)]
    amount: Option<Option<rust_decimal::Decimal>>,
}

assert_eq!(deser_json::from_str::<Price>("{}").unwrap().amount, None);
assert_eq!(
    deser_json::from_str::<Price>(r#"{"amount":null}"#).unwrap().amount,
    Some(None),
);
```

Requires Rust 1.88 or later.

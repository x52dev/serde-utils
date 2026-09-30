//! Careful serialization and deserialization of [`rust_decimal`] types.
//!
//! The `serde` feature enables the field adapters and is enabled by default.
//! Enable `miniserde` to use the wrappers in the `miniserde` module.
//! Disable default features for a miniserde-only build.
//!
//! Several tests in these modules will fail if one were to naively apply e.g.,
//! `#[serde(with = "rust_decimal::serde::float_option")]`.
//! This module provides alternative modules to be used with `#[serde(with = ...)]`.
//! This circumvents bugs in the `rust_decimal::serde` modules and adds modules for serialization
//! and deserialization of `Option<Option<Decimal>>`.
//!
//! * use `double_option_float` for `Option<Option<Decimal>>` where the field may be missing and may
//!   be null.
//! * use `non_required_float` for `Option<Decimal>` where the field may be missing but may not be
//!   null.
//! * use `nullable_float` for `Option<Decimal>` where the field is required but may be null.
//! * use `double_option_str` for `Option<Option<Decimal>>` where the field may be missing and may
//!   be null.
//! * use `non_required_str` for `Option<Decimal>` where the field may be missing but may not be
//!   null.
//! * use `nullable_str` for `Option<Decimal>` where the field is required but may be null.
//! * use `double_option_arbitrary_precision` for `Option<Option<Decimal>>` where the field may be
//!   missing and may be null.
//! * use `non_required_arbitrary_precision` for `Option<Decimal>` where the field may be missing
//!   but may not be null.
//! * use `nullable_arbitrary_precision` for `Option<Decimal>` where the field is required but may
//!   be null.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "miniserde")]
pub mod miniserde;

#[cfg(feature = "serde")]
pub mod double_option_float;
#[cfg(feature = "serde")]
pub mod non_required_float;
#[cfg(feature = "serde")]
pub mod nullable_float;

#[cfg(feature = "serde")]
pub mod double_option_str;
#[cfg(feature = "serde")]
pub mod non_required_str;
#[cfg(feature = "serde")]
pub mod nullable_str;

#[cfg(feature = "rust-decimal-arbitrary-precision")]
pub mod double_option_arbitrary_precision;
#[cfg(feature = "rust-decimal-arbitrary-precision")]
pub mod non_required_arbitrary_precision;
#[cfg(feature = "rust-decimal-arbitrary-precision")]
pub mod nullable_arbitrary_precision;

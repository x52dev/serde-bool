//! Careful serialization and deserialization of [`rust_decimal`] types.
//!
//! Several tests in these modules will fail if one were to naively apply e.g.,
//! `#[serde(with = "rust_decimal::serde::float_option")]`.
//! This module provides alternative modules to be used with `#[serde(with = ...)]`.
//! This circumvents bugs in the [rust_decimal::serde] modules and adds modules for serialization
//! and deserialization of `Option<Option<Decimal>>`.
//!
//! * use [double_option_float] for `Option<Option<Decimal>>` where the field may be missing and may
//!   be null.
//! * use [non_required_float] for `Option<Decimal>` where the field may be missing but may not be
//!   null.
//! * use [nullable_float] for `Option<Decimal>` where the field is required but may be null.
//! * use [double_option_str] for `Option<Option<Decimal>>` where the field may be missing and may
//!   be null.
//! * use [non_required_str] for `Option<Decimal>` where the field may be missing but may not be
//!   null.
//! * use [nullable_str] for `Option<Decimal>` where the field is required but may be null.
//! * use `double_option_arbitrary_precision` for `Option<Option<Decimal>>` where the field may be
//!   missing and may be null.
//! * use `non_required_arbitrary_precision` for `Option<Decimal>` where the field may be missing
//!   but may not be null.
//! * use `nullable_arbitrary_precision` for `Option<Decimal>` where the field is required but may
//!   be null.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod double_option_float;
pub mod non_required_float;
pub mod nullable_float;

pub mod double_option_str;
pub mod non_required_str;
pub mod nullable_str;

#[cfg(feature = "rust-decimal-arbitrary-precision")]
pub mod double_option_arbitrary_precision;
#[cfg(feature = "rust-decimal-arbitrary-precision")]
pub mod non_required_arbitrary_precision;
#[cfg(feature = "rust-decimal-arbitrary-precision")]
pub mod nullable_arbitrary_precision;

//! **De**serialization **trim**ming for strings in serde models.
//!
//! # Examples
//!
//! ```
//! #[derive(Debug, serde::Deserialize)]
//! struct Form {
//!     #[serde(deserialize_with = "detrim::string")]
//!     name: String,
//! }
//!
//! let form = serde_json::from_str::<Form>(r#"{ "name": "ferris" }"#).unwrap();
//! assert_eq!(form.name, "ferris");
//!
//! let form = serde_json::from_str::<Form>(r#"{ "name": "  ferris   " }"#).unwrap();
//! assert_eq!(form.name, "ferris");
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

mod cow_str;
#[cfg(feature = "std")]
mod hashset_string;
mod string;
mod string_non_empty;
mod vec_string;

#[cfg(feature = "std")]
pub use crate::hashset_string::hashset_string;
pub use crate::{
    cow_str::cow_str,
    string::{option_string, str, string},
    string_non_empty::{option_string_non_empty, string_non_empty},
    vec_string::vec_string,
};

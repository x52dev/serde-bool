//! Single-side boolean deserializers.
//!
//! # Examples
//!
//! Supporting serde untagged enums where only one boolean value is valid, allowing fallthrough to
//! the next variant. Avoids need to wrap all fields in `Option<_>` just in case feature is disabled.
//!
//! ```
//! #[derive(Debug, serde::Deserialize)]
//! struct Config {
//!     feature: FeatureConfig,
//! }
//!
//! #[derive(Debug, serde::Deserialize)]
//! #[serde(untagged)]
//! enum FeatureConfig {
//!     Disabled {
//!         enabled: serde_bool::False
//!     },
//!
//!     Enabled {
//!         #[serde(default)]
//!         enabled: serde_bool::True,
//!         key: String,
//!         secret: String,
//!     }
//! }
//!
//! // disabled variant is matched
//! let config = toml::from_str::<Config>(r#"
//!     [feature]
//!     enabled = false
//! "#).unwrap();
//! assert!(matches!(config.feature, FeatureConfig::Disabled { .. }));
//!
//! // if the type used `enabled: bool`, this would cause issues and require Option<_> wrappers plus
//! // further validation... instead an error is returned immediately regarding the missing fields
//! let config = toml::from_str::<Config>(r#"
//!     [feature]
//!     enabled = true
//! "#).unwrap_err();
//!
//! // using a `#[serde(default)]` annotation makes `enabled = true` optional here
//! let config = toml::from_str::<Config>(r#"
//!     [feature]
//!     key = "foo"
//!     secret = "bar"
//! "#).unwrap();
//! assert!(matches!(config.feature, FeatureConfig::Enabled { .. }));
//!
//! // extra keys can exists in the disabled case, but as usual will not be captured
//! let config = toml::from_str::<Config>(r#"
//!     [feature]
//!     enabled = false
//!     key = "foo"
//!     secret = "bar"
//! "#).unwrap();
//! assert!(matches!(config.feature, FeatureConfig::Disabled { .. }));
//! ```

#![no_std]
#![deny(rust_2018_idioms, nonstandard_style, future_incompatible)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Type that only deserializes from the `true` boolean value.
///
/// # Examples
///
/// ```
/// assert_eq!(
///     serde_json::from_str::<serde_bool::True>("true").unwrap().as_bool(),
///     true,
/// );
///
/// serde_json::from_str::<serde_bool::True>("false").unwrap_err();
/// serde_json::from_str::<serde_bool::True>("42").unwrap_err();
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct True;

impl True {
    /// Returns `true`.
    pub const fn as_bool(self) -> bool {
        true
    }
}

impl From<True> for bool {
    fn from(_: True) -> Self {
        true
    }
}

impl PartialEq<False> for True {
    fn eq(&self, _: &False) -> bool {
        false
    }
}

impl PartialEq<bool> for True {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}

impl PartialEq<True> for bool {
    fn eq(&self, other: &True) -> bool {
        *self == other.as_bool()
    }
}

impl<'de> Deserialize<'de> for True {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if bool::deserialize(deserializer)? {
            Ok(Self)
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Bool(false),
                &"the `true` boolean",
            ))
        }
    }
}

impl Serialize for True {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(true)
    }
}

/// Type that only deserializes from the `false` boolean value.
///
/// # Examples
///
/// ```
/// assert_eq!(
///     serde_json::from_str::<serde_bool::False>("false").unwrap().as_bool(),
///     false,
/// );
///
/// serde_json::from_str::<serde_bool::False>("true").unwrap_err();
/// serde_json::from_str::<serde_bool::False>("42").unwrap_err();
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct False;

impl False {
    /// Returns `false`.
    pub const fn as_bool(self) -> bool {
        false
    }
}

impl From<False> for bool {
    fn from(_: False) -> Self {
        false
    }
}

impl PartialEq<True> for False {
    fn eq(&self, _: &True) -> bool {
        false
    }
}

impl PartialEq<bool> for False {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}

impl PartialEq<False> for bool {
    fn eq(&self, other: &False) -> bool {
        *self == other.as_bool()
    }
}

impl<'de> Deserialize<'de> for False {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if bool::deserialize(deserializer)? {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Bool(true),
                &"the `false` boolean",
            ))
        } else {
            Ok(Self)
        }
    }
}

impl Serialize for False {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Tru {
        foo: True,
    }

    #[test]
    fn de_true() {
        assert_eq!(
            Tru { foo: True },
            serde_json::from_str::<Tru>(r#"{"foo": true}"#).unwrap(),
        );

        serde_json::from_str::<Tru>(r#"{"foo": false}"#).unwrap_err();
        serde_json::from_str::<Tru>(r#"{"foo": 42}"#).unwrap_err();
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Fal {
        foo: False,
    }

    #[test]
    fn de_false() {
        assert_eq!(
            Fal { foo: False },
            serde_json::from_str::<Fal>(r#"{"foo": false}"#).unwrap(),
        );

        serde_json::from_str::<Fal>(r#"{"foo": true}"#).unwrap_err();
        serde_json::from_str::<Fal>(r#"{"foo": 42}"#).unwrap_err();
    }

    #[test]
    fn ser() {
        assert_eq!("true", serde_json::to_string(&True).unwrap());
        assert_eq!("false", serde_json::to_string(&False).unwrap());
    }

    #[test]
    fn as_bool() {
        assert!(True.as_bool());
        assert!(!False.as_bool());
    }

    #[test]
    fn from() {
        assert!(bool::from(True));
        assert!(!bool::from(False));
    }

    #[test]
    fn eq() {
        assert_eq!(True, True);
        assert_eq!(True, true);
        assert_eq!(true, True);
        assert_eq!(False, False);
        assert_eq!(False, false);
        assert_eq!(false, False);

        assert_ne!(True, False);
        assert_ne!(True, false);
        assert_ne!(False, True);
        assert_ne!(false, True);

        assert_ne!(False, True);
        assert_ne!(False, true);
        assert_ne!(True, False);
        assert_ne!(true, False);
    }

    #[test]
    fn formatting() {
        let _ = format_args!("{:?}", True);
        let _ = format_args!("{:?}", False);
    }

    #[test]
    fn other_implementations() {
        #![allow(clippy::default_constructed_unit_structs)]

        assert_eq!(True.clone(), True);
        assert_eq!(False.clone(), False);

        assert_eq!(True::default(), True);
        assert_eq!(False::default(), False);
    }
}

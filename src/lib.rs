//! Single-side boolean deserializers.

#![deny(rust_2018_idioms, nonstandard_style, future_incompatible)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Type that only deserializes from the `true` boolean value.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct True;

impl True {
    /// Returns `true`.
    pub fn as_bool(self) -> bool {
        true
    }
}

impl From<True> for bool {
    fn from(_: True) -> Self {
        true
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct False;

impl False {
    /// Returns `false`.
    pub fn as_bool(self) -> bool {
        false
    }
}

impl From<False> for bool {
    fn from(_: False) -> Self {
        false
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
    }

    #[test]
    fn se_true() {
        assert_eq!(
            Tru { foo: True },
            serde_json::from_str::<Tru>(r#"{"foo": true}"#).unwrap(),
        );

        serde_json::from_str::<Tru>(r#"{"foo": false}"#).unwrap_err();
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
    }
}

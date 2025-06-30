//! Serialization and deserialization of required but nullable decimals as floats.

use std::fmt::{self};

use rust_decimal::Decimal;

struct OptionDecimalVisitor;

impl<'de> serde::de::Visitor<'de> for OptionDecimalVisitor {
    type Value = Option<Decimal>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Decimal type representing a fixed-point number")
    }

    fn visit_none<E>(self) -> Result<Option<Decimal>, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> Result<Option<Decimal>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        <Decimal as serde::Deserialize>::deserialize(d).map(Some)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }
}

/// Nullable float-form decimal deserializer.
///
/// See [module docs](self) for more.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<rust_decimal::Decimal>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    deserializer.deserialize_option(OptionDecimalVisitor)
}

/// Nullable float-form decimal serializer.
///
/// See [module docs](self) for more.
pub fn serialize<S>(value: &Option<rust_decimal::Decimal>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    rust_decimal::serde::float_option::serialize(value, serializer)
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    struct Foo {
        #[serde(with = "crate::nullable_float")]
        foo: Option<rust_decimal::Decimal>,
    }

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    struct Bar {
        #[serde(flatten)]
        foo: Foo,
    }

    #[test]
    fn foo_serialize_some() {
        let serialized = serde_json::to_string(&Foo {
            foo: Some(dec!(0.1)),
        })
        .unwrap();
        assert_eq!(serialized, r#"{"foo":0.1}"#);
    }

    #[test]
    fn foo_serialize_none() {
        let serialized = serde_json::to_string(&Foo { foo: None }).unwrap();
        assert_eq!(serialized, r#"{"foo":null}"#);
    }

    #[test]
    fn foo_deserialize_some() {
        let deserialized: Foo = serde_json::from_str(r#"{"foo":0.1}"#).unwrap();
        assert!(matches!(deserialized, Foo { foo: Some(_) }));
    }

    #[test]
    #[should_panic]
    fn foo_deserialize_missing() {
        serde_json::from_str::<Foo>(r#"{}"#).unwrap();
    }

    #[test]
    fn foo_deserialize_null() {
        let deserialized: Foo = serde_json::from_str(r#"{"foo":null}"#).unwrap();
        assert!(matches!(deserialized, Foo { foo: None }));
    }

    #[test]
    fn bar_serialize_some() {
        let serialized = serde_json::to_string(&Bar {
            foo: Foo {
                foo: Some(dec!(0.1)),
            },
        })
        .unwrap();
        assert_eq!(serialized, r#"{"foo":0.1}"#);
    }

    #[test]
    fn bar_serialize_none() {
        let serialized = serde_json::to_string(&Bar {
            foo: Foo { foo: None },
        })
        .unwrap();
        assert_eq!(serialized, r#"{"foo":null}"#);
    }

    #[test]
    fn bar_deserialize_some() {
        let deserialized: Bar = serde_json::from_str(r#"{"foo":0.1}"#).unwrap();
        assert!(matches!(
            deserialized,
            Bar {
                foo: Foo { foo: Some(_) }
            }
        ));
    }

    #[test]
    #[should_panic]
    fn bar_deserialize_missing() {
        serde_json::from_str::<Bar>(r#"{}"#).unwrap();
    }

    #[test]
    fn bar_deserialize_null() {
        let deserialized: Bar = serde_json::from_str(r#"{"foo":null}"#).unwrap();
        assert!(matches!(
            deserialized,
            Bar {
                foo: Foo { foo: None }
            }
        ));
    }
}

use alloc::{borrow::ToOwned as _, string::String};

use serde::{Deserialize as _, Deserializer};

/// Trims a string slice during deserialization.
pub fn str<'a, 'de: 'a, D: Deserializer<'de>>(de: D) -> Result<&'a str, D::Error> {
    <&'a str>::deserialize(de).map(|val| val.trim())
}

/// Trims a string during deserialization.
pub fn string<'de, D: Deserializer<'de>>(de: D) -> Result<String, D::Error> {
    String::deserialize(de).map(|val| val.trim().to_owned())
}

/// Trims an optional string during deserialization.
pub fn option_string<'de, D: Deserializer<'de>>(de: D) -> Result<Option<String>, D::Error> {
    let val = Option::<String>::deserialize(de)?;
    Ok(val.map(|val| val.trim().to_owned()))
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn str() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Foo<'a> {
            #[serde(borrow, deserialize_with = "super::str")]
            foo: &'a str,
        }

        impl<'a> Foo<'a> {
            fn new(foo: &'a str) -> Self {
                Self { foo }
            }
        }

        serde_json::from_str::<Foo<'static>>(r#"{ "foo": 1 }"#).unwrap_err();
        serde_json::from_str::<Foo<'static>>(r#"{ "foo": true }"#).unwrap_err();

        assert_eq!(
            Foo::new(""),
            serde_json::from_str(r#"{ "foo": "" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new(""),
            serde_json::from_str(r#"{ "foo": " " }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": " bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "  bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "bar " }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "  bar  " }"#).unwrap(),
        );
    }

    #[test]
    fn string() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Foo {
            #[serde(deserialize_with = "super::string")]
            foo: String,
        }

        impl Foo {
            fn new(foo: impl Into<String>) -> Self {
                Self { foo: foo.into() }
            }
        }

        serde_json::from_str::<Foo>(r#"{ "foo": 1 }"#).unwrap_err();
        serde_json::from_str::<Foo>(r#"{ "foo": true }"#).unwrap_err();

        assert_eq!(
            Foo::new(""),
            serde_json::from_str(r#"{ "foo": "" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new(""),
            serde_json::from_str(r#"{ "foo": " " }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": " bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "  bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "bar " }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "  bar  " }"#).unwrap(),
        );
    }

    #[test]
    fn option_string() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Foo {
            #[serde(deserialize_with = "super::option_string")]
            foo: Option<String>,
        }

        impl Foo {
            fn none() -> Self {
                Self { foo: None }
            }

            fn new(foo: impl Into<String>) -> Self {
                Self {
                    foo: Some(foo.into()),
                }
            }
        }

        assert_eq!(
            Foo::none(),
            serde_json::from_str(r#"{ "foo": null }"#).unwrap(),
        );
        assert_eq!(
            Foo::new(""),
            serde_json::from_str(r#"{ "foo": "" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new(""),
            serde_json::from_str(r#"{ "foo": " " }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": " bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "  bar" }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "bar " }"#).unwrap(),
        );
        assert_eq!(
            Foo::new("bar"),
            serde_json::from_str(r#"{ "foo": "  bar  " }"#).unwrap(),
        );
    }
}

use alloc::{borrow::ToOwned, string::String};

use serde::{de, Deserialize as _, Deserializer};

/// Trims string during deserialization, returning error if it ends up empty.
pub fn string_non_empty<'a, D: Deserializer<'a>>(de: D) -> Result<String, D::Error> {
    match String::deserialize(de) {
        Ok(val) if val.trim().is_empty() => Err(de::Error::invalid_value(
            de::Unexpected::Other("empty string"),
            &"non-empty string",
        )),
        Ok(val) => Ok(val.trim().to_owned()),
        Err(err) => Err(err),
    }
}

/// Trims string during deserialization, returning `None` if it ends up empty.
pub fn option_string_non_empty<'a, D: Deserializer<'a>>(de: D) -> Result<Option<String>, D::Error> {
    match Option::<String>::deserialize(de)? {
        None => Ok(None),
        Some(val) if val.trim().is_empty() => Ok(None),
        Some(val) => Ok(Some(val.trim().to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn string_non_empty() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Foo {
            #[serde(deserialize_with = "super::string_non_empty")]
            foo: String,
        }

        impl Foo {
            fn new(foo: impl Into<String>) -> Self {
                Self { foo: foo.into() }
            }
        }

        serde_json::from_str::<Foo>(r#"{ "foo": "" }"#).unwrap_err();
        serde_json::from_str::<Foo>(r#"{ "foo": "  " }"#).unwrap_err();
        serde_json::from_str::<Foo>(r#"{ "foo": null }"#).unwrap_err();

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
    fn option_string_non_empty() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Foo {
            #[serde(deserialize_with = "super::option_string_non_empty")]
            foo: Option<String>,
        }

        impl Foo {
            fn new(foo: impl Into<String>) -> Self {
                Self {
                    foo: Some(foo.into()),
                }
            }

            fn none() -> Self {
                Self { foo: None }
            }
        }

        assert_eq!(
            Foo::none(),
            serde_json::from_str(r#"{ "foo": null }"#).unwrap(),
        );
        assert_eq!(
            Foo::none(),
            serde_json::from_str(r#"{ "foo": "" }"#).unwrap(),
        );
        assert_eq!(
            Foo::none(),
            serde_json::from_str(r#"{ "foo": "  " }"#).unwrap(),
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

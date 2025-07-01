use alloc::{borrow::ToOwned as _, string::String, vec::Vec};

use serde::{Deserialize as _, Deserializer};

/// Trims list of strings during deserialization.
pub fn vec_string<'a, D: Deserializer<'a>>(de: D) -> Result<Vec<String>, D::Error> {
    let mut list = Vec::<String>::deserialize(de)?;

    for item in &mut list {
        #[allow(clippy::assigning_clones)]
        {
            *item = item.trim().to_owned();
        }
    }

    Ok(list)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn vec_string() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Foo {
            #[serde(deserialize_with = "super::vec_string")]
            foo: Vec<String>,
        }

        impl Foo {
            fn new(foo: impl IntoIterator<Item = impl Into<String>>) -> Self {
                Self {
                    foo: foo.into_iter().map(Into::into).collect(),
                }
            }
        }

        serde_json::from_str::<Foo>(r#"{ "foo": 1 }"#).unwrap_err();
        serde_json::from_str::<Foo>(r#"{ "foo": "" }"#).unwrap_err();

        assert_eq!(
            Foo::new([""; 0]),
            serde_json::from_str(r#"{ "foo": [] }"#).unwrap(),
        );
        assert_eq!(
            Foo::new([""]),
            serde_json::from_str(r#"{ "foo": [""] }"#).unwrap(),
        );
        assert_eq!(
            Foo::new([""]),
            serde_json::from_str(r#"{ "foo": [" "] }"#).unwrap(),
        );
        assert_eq!(
            Foo::new(["bar"]),
            serde_json::from_str(r#"{ "foo": ["bar"] }"#).unwrap(),
        );
        assert_eq!(
            Foo::new(["bar"]),
            serde_json::from_str(r#"{ "foo": [" bar"] }"#).unwrap(),
        );
        assert_eq!(
            Foo::new(["bar"]),
            serde_json::from_str(r#"{ "foo": ["  bar"] }"#).unwrap(),
        );
        assert_eq!(
            Foo::new(["bar"]),
            serde_json::from_str(r#"{ "foo": ["bar "] }"#).unwrap(),
        );
        assert_eq!(
            Foo::new(["bar"]),
            serde_json::from_str(r#"{ "foo": ["  bar  "] }"#).unwrap(),
        );
    }
}

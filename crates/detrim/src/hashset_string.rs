use std::{
    borrow::ToOwned as _, collections::HashSet, iter::FromIterator as _, string::String, vec::Vec,
};

use serde::{Deserialize as _, Deserializer};

/// Trims set of strings during deserialization.
///
/// Strings are deduplicated _after_ being trimmed (i.e., differences in extraneous whitespace are
/// handled).
pub fn hashset_string<'a, D: Deserializer<'a>>(de: D) -> Result<HashSet<String>, D::Error> {
    let mut set = Vec::<String>::deserialize(de)?;

    for item in &mut set {
        #[allow(clippy::assigning_clones)]
        {
            *item = item.trim().to_owned();
        }
    }

    Ok(HashSet::from_iter(set))
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn hashset_string() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Foo {
            #[serde(deserialize_with = "super::hashset_string")]
            foo: HashSet<String>,
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
        assert_eq!(
            Foo::new(["bar"]),
            serde_json::from_str(r#"{ "foo": ["  bar  ", "  bar  "] }"#).unwrap(),
        );
        assert_eq!(
            Foo::new(["bar"]),
            serde_json::from_str(r#"{ "foo": ["  bar  ", "  bar"] }"#).unwrap(),
        );
    }
}

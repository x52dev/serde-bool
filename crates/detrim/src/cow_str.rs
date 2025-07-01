use alloc::{
    borrow::{Cow, ToOwned as _},
    str,
    string::String,
    vec::Vec,
};
use core::fmt;

use serde::{de, Deserializer};

/// Trims a CoW string during deserialization.
pub fn cow_str<'a, 'de: 'a, D: Deserializer<'de>>(de: D) -> Result<Cow<'a, str>, D::Error> {
    struct CowStrVisitor;

    impl<'a> de::Visitor<'a> for CowStrVisitor {
        type Value = Cow<'a, str>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E: de::Error>(self, val: &str) -> Result<Self::Value, E> {
            Ok(Cow::Owned(val.trim().to_owned()))
        }

        fn visit_borrowed_str<E: de::Error>(self, val: &'a str) -> Result<Self::Value, E> {
            Ok(Cow::Borrowed(val.trim()))
        }

        fn visit_string<E: de::Error>(self, val: String) -> Result<Self::Value, E> {
            Ok(Cow::Owned(val.trim().to_owned()))
        }

        fn visit_bytes<E: de::Error>(self, val: &[u8]) -> Result<Self::Value, E> {
            match str::from_utf8(val) {
                Ok(val) => Ok(Cow::Owned(val.trim().to_owned())),
                Err(_) => Err(de::Error::invalid_value(de::Unexpected::Bytes(val), &self)),
            }
        }

        fn visit_borrowed_bytes<E: de::Error>(self, val: &'a [u8]) -> Result<Self::Value, E> {
            match str::from_utf8(val) {
                Ok(val) => Ok(Cow::Borrowed(val.trim())),
                Err(_) => Err(de::Error::invalid_value(de::Unexpected::Bytes(val), &self)),
            }
        }

        fn visit_byte_buf<E: de::Error>(self, val: Vec<u8>) -> Result<Self::Value, E> {
            match String::from_utf8(val) {
                Ok(val) => Ok(Cow::Owned(val.trim().to_owned())),
                Err(err) => Err(de::Error::invalid_value(
                    de::Unexpected::Bytes(&err.into_bytes()),
                    &self,
                )),
            }
        }
    }

    de.deserialize_str(CowStrVisitor)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[test]
    fn cow_str() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Foo<'a> {
            #[serde(borrow, deserialize_with = "super::cow_str")]
            foo: Cow<'a, str>,
        }

        impl<'a> Foo<'a> {
            fn new(foo: impl Into<Cow<'a, str>>) -> Self {
                Self { foo: foo.into() }
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
    fn cow_str_allows_borrows() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct Foo<'a> {
            #[serde(borrow, deserialize_with = "super::cow_str")]
            foo: Cow<'a, str>,
        }

        // borrowed when no trimming is needed
        let source = br#"{ "foo": "bar" }"#.to_vec();
        let json = serde_json::from_slice::<Foo<'_>>(&source).unwrap();
        assert!(matches!(&json.foo, Cow::Borrowed(_)));
        assert_eq!(json.foo, "bar");

        // borrowed and trimmed
        let source = br#"{ "foo": " bar " }"#.to_vec();
        let json = serde_json::from_slice::<Foo<'_>>(&source).unwrap();
        assert!(matches!(&json.foo, Cow::Borrowed(_)));
        assert_eq!(json.foo, "bar");

        // owned and trimmed when escape sequences need processing
        let source = br#"{ "foo": " b\\ar " }"#.to_vec();
        let json = serde_json::from_slice::<Foo<'_>>(&source).unwrap();
        assert!(matches!(&json.foo, Cow::Owned(_)));
        assert_eq!(json.foo, "b\\ar");
    }
}

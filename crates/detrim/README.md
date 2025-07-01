# `detrim`

<!-- prettier-ignore-start -->

[![crates.io](https://img.shields.io/crates/v/detrim?label=latest)](https://crates.io/crates/detrim)
[![Documentation](https://docs.rs/detrim/badge.svg?version=0.1.5)](https://docs.rs/detrim/0.1.5)
[![dependency status](https://deps.rs/crate/detrim/0.1.5/status.svg)](https://deps.rs/crate/detrim/0.1.5)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/detrim.svg)
<br />
[![CI](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/x52dev/detrim/branch/main/graph/badge.svg)](https://codecov.io/gh/x52dev/detrim)
![Version](https://img.shields.io/badge/rustc-1.70.0+-ab6000.svg)
[![Download](https://img.shields.io/crates/d/detrim.svg)](https://crates.io/crates/detrim)

<!-- prettier-ignore-end -->

<!-- cargo-rdme start -->

**De**serialization **trim**ming for strings in serde models.

## Examples

```rust
#[derive(Debug, serde::Deserialize)]
struct Form {
    #[serde(deserialize_with = "detrim::string")]
    name: String,
}

let form = serde_json::from_str::<Form>(r#"{ "name": "ferris" }"#).unwrap();
assert_eq!(form.name, "ferris");

let form = serde_json::from_str::<Form>(r#"{ "name": "  ferris   " }"#).unwrap();
assert_eq!(form.name, "ferris");
```

<!-- cargo-rdme end -->

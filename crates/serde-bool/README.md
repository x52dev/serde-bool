# `serde-bool`

<!-- prettier-ignore-start -->

[![crates.io](https://img.shields.io/crates/v/serde-bool?label=latest)](https://crates.io/crates/serde-bool)
[![Documentation](https://docs.rs/serde-bool/badge.svg?version=0.1.3)](https://docs.rs/serde-bool/0.1.3)
[![dependency status](https://deps.rs/crate/serde-bool/0.1.3/status.svg)](https://deps.rs/crate/serde-bool/0.1.3)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/serde-bool.svg)
<br />
[![CI](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/x52dev/serde-utils/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/x52dev/serde-bool/branch/main/graph/badge.svg)](https://codecov.io/gh/x52dev/serde-bool)
![Version](https://img.shields.io/badge/rustc-1.65+-ab6000.svg)
[![Download](https://img.shields.io/crates/d/serde-bool.svg)](https://crates.io/crates/serde-bool)

<!-- prettier-ignore-end -->

<!-- cargo-rdme start -->

Single value, true or false, boolean deserializers.

## Examples

Supporting serde untagged enums where only one boolean value is valid, allowing fallthrough to the next variant. Avoids need to wrap all fields in `Option<_>` just in case feature is disabled.

```rust
#[derive(Debug, serde::Deserialize)]
struct Config {
    feature: FeatureConfig,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum FeatureConfig {
    Disabled {
        enabled: serde_bool::False
    },

    Enabled {
        #[serde(default)]
        enabled: serde_bool::True,
        key: String,
        secret: String,
    }
}

// disabled variant is matched
let config = toml::from_str::<Config>(r#"
    [feature]
    enabled = false
"#).unwrap();
assert!(matches!(config.feature, FeatureConfig::Disabled { .. }));

// if the type used `enabled: bool`, this would cause issues and require Option<_> wrappers plus
// further validation... instead an error is returned immediately regarding the missing fields
let config = toml::from_str::<Config>(r#"
    [feature]
    enabled = true
"#).unwrap_err();

// using a `#[serde(default)]` annotation makes `enabled = true` optional here
let config = toml::from_str::<Config>(r#"
    [feature]
    key = "foo"
    secret = "bar"
"#).unwrap();
assert!(matches!(config.feature, FeatureConfig::Enabled { .. }));

// extra keys can exists in the disabled case, but as usual will not be captured
let config = toml::from_str::<Config>(r#"
    [feature]
    enabled = false
    key = "foo"
    secret = "bar"
"#).unwrap();
assert!(matches!(config.feature, FeatureConfig::Disabled { .. }));
```

<!-- cargo-rdme end -->

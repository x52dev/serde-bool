_list:
    @just --list

toolchain := ""
msrv := ```
    cargo metadata --format-version=1 \
    | jq -r 'first(.packages[] | select(.source == null and .rust_version)) | .rust_version' \
    | sed -E 's/^1\.([0-9]{2})$/1\.\1\.0/'
```
msrv_rustup := "+" + msrv

# Check project.
check: && clippy
    just --unstable --fmt --check
    fd -e=md -e=yml --exec-batch prettier --check
    taplo lint
    cargo +nightly fmt -- --check

# Format project.
fmt:
    just --unstable --fmt
    fd -e=md -e=yml --exec-batch prettier --write
    taplo format
    cargo +nightly fmt

# Clippy check workspace.
clippy:
    cargo clippy --workspace --no-default-features
    cargo clippy --workspace --all-features
    cargo hack --feature-powerset --depth=3 clippy --workspace

# Run workspace test suite.
test:
    cargo {{ toolchain }} nextest run
    cargo {{ toolchain }} test --doc
    RUSTDOCFLAGS="-D warnings" cargo {{ toolchain }} doc --workspace --no-deps --all-features

# Run workspace test suite using MSRV.
test-msrv:
    @just test +1.56.1

# Run workspace test suite, capturing coverage.
test-coverage:
    @just test-coverage-codecov {{ toolchain }}
    @just test-coverage-lcov {{ toolchain }}

# Run workspace test suite, capturing coverage info in Codecov format.
test-coverage-codecov:
    cargo {{ toolchain }} llvm-cov --workspace --all-features --codecov --output-path codecov.json

# Run workspace test suite, capturing coverage info in Lcov format.
test-coverage-lcov:
    cargo {{ toolchain }} llvm-cov --workspace --all-features --lcov --output-path lcov.info

# Build workspace documentation.
doc:
    RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features

# Build workspace documentation, open it, and watch for changes.
doc-watch:
    RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features --open
    cargo watch -- RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features

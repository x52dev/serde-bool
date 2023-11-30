_list:
    @just --list

# Clippy check workspace.
clippy:
    cargo clippy --workspace --no-default-features
    cargo clippy --workspace --all-features
    cargo hack --feature-powerset --depth=3 clippy --workspace

# Run workspace test suite.
test toolchain="":
    cargo {{ toolchain }} test
    RUSTDOCFLAGS="-D warnings" cargo {{ toolchain }} doc --workspace --no-deps --all-features

# Run workspace test suite using MSRV.
test-msrv:
    @just test +1.56.1

# Run workspace test suite, capturing coverage.
test-coverage toolchain="":
    @just test-coverage-codecov {{ toolchain }}
    @just test-coverage-lcov {{ toolchain }}

# Run workspace test suite, capturing coverage info in Codecov format.
test-coverage-codecov toolchain="":
    cargo {{ toolchain }} llvm-cov --workspace --all-features --codecov --output-path codecov.json

# Run workspace test suite, capturing coverage info in Lcov format.
test-coverage-lcov toolchain="":
    cargo {{ toolchain }} llvm-cov --workspace --all-features --lcov --output-path lcov.info

# Build workspace documentation.
doc:
    RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features

# Build workspace documentation, open it, and watch for changes.
doc-watch:
    RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features --open
    cargo watch -- RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc --no-deps --workspace --all-features

# Check project.
check:
    just --unstable --fmt --check
    npx -y prettier --check '**/*.{md,yml,yaml}'
    taplo lint
    cargo +nightly fmt -- --check
    @just clippy

# Format project.
fmt:
    just --unstable --fmt
    npx -y prettier --write '**/*.{md,yml,yaml}'
    taplo format
    cargo +nightly fmt

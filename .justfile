default:
  just --list

set shell := ['bash', '-cu']

fmt:
    cargo fmt --all

clippy:
    cargo clippy --all-targets -- -D warnings

fix:
    cargo clippy --all-targets --fix --allow-dirty

build:
    cargo build

build-release:
    cargo build --release

test:
    cargo test

clean:
    cargo clean

doc:
    cargo doc --open

layout-test:
    RUST_LOG=debug cargo test -p arcivis-layout -- --nocapture

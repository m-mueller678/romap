set -e

cargo clippy --no-default-features -- --deny warnings
cargo clippy --all-features --all-targets -- -D warnings
cargo +nightly fmt --all --check
cargo test --all-features

echo ok
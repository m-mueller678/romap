set -e

cargo clippy --no-default-features -- --deny warnings
cargo clippy --all-features --all-targets -- -D warnings
cargo +nightly fmt --all --check
cargo test --all-features

README_PATH="Readme.md"
if ! cargo readme | diff -u "$README_PATH" - > /dev/null; then
    echo -e "\033[1;31mReadme files differ. Please regenerate Readme.md.\033[0m" >&2
    exit 1
fi

echo ok
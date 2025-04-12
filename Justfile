check:
    cargo fmt --all
    cargo test
    cargo clippy --all-targets  --workspace -- -D warnings
    cargo doc

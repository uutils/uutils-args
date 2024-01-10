check:
    cargo fmt --all
    cargo test --features complete
    cargo clippy --all-targets --features complete  --workspace -- -D warnings
    cargo doc

fmt:
    cargo fmt

lint:
    cargo clippy -- -D warnings

test:
    cargo test

ci:
    just fmt && just lint && just test

doc:
    cargo doc

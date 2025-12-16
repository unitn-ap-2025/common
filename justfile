fmt:
    cargo fmt

lint:
    cargo clippy -- -D warnings -W clippy::pedantic -A unused

test:
    cargo test

ci:
    just fmt && just lint && just test

doc:
    cargo doc

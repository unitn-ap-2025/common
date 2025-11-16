fmt:
    cargo fmt

lint:
    cargo clippy

test:
    cargo test

ci:
    just fmt && just lint && just test

doc:
    cargo doc

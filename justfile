raw:
    cargo build

run:
    cargo run dsl.txt

log:
    RUST_LOG=debug cargo run dsl.txt

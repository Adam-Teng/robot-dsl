raw:
    cargo build --release

run:
    cargo run --release

runfile:
    cargo run example/dsl.txt --release

doc:
    cargo doc

mod-tree-lib:
    cargo-modules generate tree --lib --with-types --with-orphans

mod-tree-bin:
    cargo-modules generate tree --bin robot-dsl --with-types --with-tests

mod-graph:
    cargo-modules generate graph --lib --with-types --with-orphans | dot -Tpng -o doc/images/mod-graph.png

count:
    cloc src/ tests/

test:
    cargo test

clean:
    cargo clean

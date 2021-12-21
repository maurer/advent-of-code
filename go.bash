mv ~/input.txt inputs/2021/20
target/debug/aoc-gen < inputs/2021/20 > src/bin/aoc-2021-20.rs
rustfmt src/bin/aoc-2021-20.rs
cargo build --bin aoc-2021-20

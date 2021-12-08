mv ~/input.txt inputs/2021/8
target/debug/aoc-gen < inputs/2021/8 > src/bin/aoc-2021-8.rs
rustfmt src/bin/aoc-2021-8.rs
cargo build --bin aoc-2021-8

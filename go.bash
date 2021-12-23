mv ~/input.txt inputs/2021/23
target/debug/aoc-gen < inputs/2021/23 > src/bin/aoc-2021-23.rs
rustfmt src/bin/aoc-2021-23.rs
cargo build --bin aoc-2021-23

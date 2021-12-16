mv ~/input.txt inputs/2021/15
target/debug/aoc-gen < inputs/2021/15 > src/bin/aoc-2021-15.rs
rustfmt src/bin/aoc-2021-15.rs
cargo build --bin aoc-2021-15

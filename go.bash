mv ~/input.txt inputs/2021/17
target/debug/aoc-gen < inputs/2021/17 > src/bin/aoc-2021-17.rs
rustfmt src/bin/aoc-2021-17.rs
cargo build --bin aoc-2021-17

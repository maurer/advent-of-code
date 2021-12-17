mv ~/input.txt inputs/2021/16
target/debug/aoc-gen < inputs/2021/16 > src/bin/aoc-2021-16.rs
rustfmt src/bin/aoc-2021-16.rs
cargo build --bin aoc-2021-16

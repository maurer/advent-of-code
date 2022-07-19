mv ~/input.txt inputs/2021/25
target/debug/aoc-gen < inputs/2021/25 > src/bin/aoc-2021-25.rs
rustfmt src/bin/aoc-2021-25.rs
cargo build --bin aoc-2021-25

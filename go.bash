mv ~/input.txt inputs/2021/24
target/debug/aoc-gen < inputs/2021/24 > src/bin/aoc-2021-24.rs
rustfmt src/bin/aoc-2021-24.rs
cargo build --bin aoc-2021-24

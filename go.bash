mv ~/input.txt inputs/2021/7
target/debug/aoc-gen < inputs/2021/7 > src/bin/aoc-2021-7.rs
rustfmt src/bin/aoc-2021-7.rs
cargo build --bin aoc-2021-7

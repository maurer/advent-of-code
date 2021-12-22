mv ~/input.txt inputs/2021/22
target/debug/aoc-gen < inputs/2021/22 > src/bin/aoc-2021-22.rs
rustfmt src/bin/aoc-2021-22.rs
cargo build --bin aoc-2021-22

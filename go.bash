mv ~/input.txt inputs/2021/12
target/debug/aoc-gen < inputs/2021/12 > src/bin/aoc-2021-12.rs
rustfmt src/bin/aoc-2021-12.rs
cargo build --bin aoc-2021-12

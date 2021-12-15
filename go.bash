mv ~/input.txt inputs/2021/14
target/debug/aoc-gen < inputs/2021/14 > src/bin/aoc-2021-14.rs
rustfmt src/bin/aoc-2021-14.rs
cargo build --bin aoc-2021-14

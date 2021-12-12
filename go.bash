mv ~/input.txt inputs/2021/11
target/debug/aoc-gen < inputs/2021/11 > src/bin/aoc-2021-11.rs
rustfmt src/bin/aoc-2021-11.rs
cargo build --bin aoc-2021-11

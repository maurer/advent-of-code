mv ~/input.txt inputs/2021/10
target/debug/aoc-gen < inputs/2021/10 > src/bin/aoc-2021-10.rs
rustfmt src/bin/aoc-2021-10.rs
cargo build --bin aoc-2021-10

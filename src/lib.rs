#![feature(stdio_locked)]
use std::io::BufRead;

pub fn stdin_input() -> impl Iterator<Item = String> {
    std::io::stdin_locked().lines().map(|line| line.unwrap())
}

pub fn str_input(input: &'static str) -> impl Iterator<Item = String> {
    std::io::Cursor::new(input).lines().map(|line| line.unwrap())
}

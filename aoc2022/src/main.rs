#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod day10;
use crate::day10::{compute_solution_1,compute_solution_2};

fn main() {
    let day: u32 = 10;
    let use_dummy_input: bool = false;
    compute_solution_1(get_input(day, use_dummy_input));
    compute_solution_2(get_input(day, use_dummy_input));
}

fn get_input(day: u32, dummy: bool) -> String {
    let dummy_prefix = if dummy { "dummy_" } else { "" };
    return std::fs::read_to_string(format!(
        "/home/stijn/Projects/Rust_Advent_Of_Code_2022/aoc2022/src/day{day}/{dummy_prefix}input.txt"
    ))
    .unwrap();
}

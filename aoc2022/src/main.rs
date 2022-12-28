#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod day15;
use std::time::Instant;

use crate::day15::{compute_solution_1, compute_solution_2};

fn main() {
    let day: u32 = 15;
    let use_dummy_input: bool = false;
    let now = Instant::now();
    compute_solution_1(get_input(day, use_dummy_input));
    println!("Part 1 - took {} ms", now.elapsed().as_millis());

    let now = Instant::now();
    compute_solution_2(get_input(day, use_dummy_input));
    println!("Part 2 - took {} ms", now.elapsed().as_millis());
}

fn get_input(day: u32, dummy: bool) -> String {
    let dummy_prefix = if dummy { "dummy_" } else { "" };
    return std::fs::read_to_string(format!("/home/stijn/Projects/Rust_Advent_Of_Code_2022/aoc2022/src/day{day}/{dummy_prefix}input.txt")).unwrap();
}

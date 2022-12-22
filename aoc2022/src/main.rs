mod day5;
use crate::day5::{compute_solution_1, compute_solution_2};

fn main() {
    compute_solution_1(get_input(5, false));
}

fn get_input(day: u32, dummy:bool) -> String {
    let dummy_prefix = if dummy {"_dummy"} else {""};
    return std::fs::read_to_string(format!(
        "/home/stijn/Projects/Rust_Advent_Of_Code_2022/aoc2022/src/day{day}/{dummy_prefix}input.txt"
    ))
    .unwrap()
}

use std::cmp::{max, min};

pub fn n_overlaps() {
    let input = std::fs::read_to_string(
        "/home/stijn/Projects/Rust_Advent_Of_Code_2022/aoc2022/src/day4/input.txt",
    )
    .unwrap();
    let outcome: u32 = input
        .split_whitespace()
        .map(|s| has_partial_overlap(s))
        .map(|c| c as u32)
        .sum();
    println!("{outcome}");
}

fn has_full_overlap(a: &str) -> bool {
    let lr: Vec<u32> = a.split(['-', ',']).map(|a| a.parse().unwrap()).collect();

    return max(lr[1], lr[3]) - min(lr[0], lr[2]) == max(lr[1] - lr[0], lr[3] - lr[2]);
}

fn has_partial_overlap(a: &str) -> bool {
    let lr: Vec<u32> = a.split(['-', ',']).map(|a| a.parse().unwrap()).collect();

    return (max(lr[1], lr[3]) - min(lr[0], lr[2])) <= ((lr[1] - lr[0]) + (lr[3] - lr[2]));
}

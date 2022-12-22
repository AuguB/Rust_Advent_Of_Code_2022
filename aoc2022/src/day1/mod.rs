pub fn sum_of_largest_n(n: usize) {
    let my_string = std::fs::read_to_string(
        "/home/stijn/Projects/Rust_Advent_Of_Code_2022/aoc2022/src/day1/day1_input.txt",
    )
    .unwrap();
    let mut _max: Vec<i32>= my_string
        .split("\n\n")
        .map(|f| {
            f.split("\n")
                .map(|a| a.parse().unwrap())
                .sum()
        })
        .collect();
    _max.sort();
    println!("{}", (_max[_max.len() - n..]).iter().sum::<i32>())
}

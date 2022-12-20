
pub fn sum_of_largest_n(n:usize){
    let my_string = std::fs::read_to_string("/home/stijn/Projects/Rust_Advent_Of_Code_2022/aoc2022/src/day1/day1_input.txt").unwrap();
    let mut _max = my_string.split("\n\n").map(|f:&str| f.split("\n").map(|a:&str| a.parse::<i32>().unwrap()).sum::<i32>()).collect::<Vec<i32>>();
    _max.sort();
    println!("{}",(_max[_max.len()-n..]).iter().sum::<i32>())
}

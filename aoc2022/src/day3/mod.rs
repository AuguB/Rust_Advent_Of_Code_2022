pub fn _get_sum_of_priorities() {
    let input = std::fs::read_to_string(
        "/home/stijn/Projects/Rust_Advent_Of_Code_2022/aoc2022/src/day3/input.txt",
    )
    .unwrap();

    let _output: u32 = input
        .lines()
        .map(|a| (&a[..a.len() / 2], &a[a.len() / 2..]))
        .map(|tup| intersection(tup))
        .map(|c| priority(c))
        .sum();
    println!("{_output}");
}

pub fn get_badge_priorities() {
    let input = std::fs::read_to_string(
        "/home/stijn/Projects/Rust_Advent_Of_Code_2022/aoc2022/src/day3/input.txt",
    ).unwrap();


    let list: Vec<&str> = input.split_whitespace().collect::<Vec<&str>>();

    let mut _sum = 0;
    for i in (0..list.len()).step_by(3) {
        _sum += priority(intersection3((list[i], list[i + 1], list[i + 2])));
    }
    println!("{_sum}");
}

fn intersection(tup: (&str, &str)) -> char {
    let (left, right) = tup;
    for char in left.chars() {
        if right.contains(char) {
            return char;
        }
    }
    return ' ';
}

fn intersection3(tup: (&str, &str, &str)) -> char {
    let (left, mid, right) = tup;
    for char in left.chars() {
        if mid.contains(char) & right.contains(char) {
            return char;
        }
    }
    return ' ';
}

fn priority(c: char) -> u32 {
    let points = c as u32;
    if points >= 97 {
        return points - 96;
    }
    return points - 64 + 26;
}

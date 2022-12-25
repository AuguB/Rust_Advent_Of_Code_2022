pub fn compute_solution_1(input: String) {
    let mut cycle_number: i32 = 0;
    let mut X: i32 = 1;
    let mut interesting_values: Vec<i32> = Vec::new();

    for line in input.lines() {
        increment_cycle(&mut cycle_number, &mut X, &mut interesting_values);
        if !line.starts_with("noop") {
            increment_cycle(&mut cycle_number, &mut X, &mut interesting_values);
            let increment = line
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            X += increment
        }
    }
    println!("{}", interesting_values.iter().sum::<i32>())
}

fn increment_cycle(cycle_number: &mut i32, X: &mut i32, interesting_values: &mut Vec<i32>) {
    *cycle_number += 1;
    if (*cycle_number - 20) % 40 == 0 {
        interesting_values.push(*X * *cycle_number);
    }
}

pub fn compute_solution_2(input: String) {
    let mut pixels = vec![false; 240];
    let mut cycle_number: i32 = 0;
    let mut X: i32 = 1;

    
    for (i_line, line) in input.lines().enumerate() {
        increment_cycle_2(&mut cycle_number, &mut X, &mut pixels);
        if !line.starts_with("noop") {
            increment_cycle_2(&mut cycle_number, &mut X, &mut pixels);
            let increment = line
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            X += increment
        }
    }

    for i in 0..240{
        if i%40 == 0{
            println!();
        }
        print!("{}", if pixels[i] {"#"} else {"."});
    }
}
fn increment_cycle_2(cycle_number: &mut i32, X: &mut i32, pixels: &mut Vec<bool>) {
    if (*cycle_number % 40 - *X%40).abs() <= 1 {
        pixels[(*cycle_number as usize)%240] = true;
    }
    *cycle_number += 1;
}

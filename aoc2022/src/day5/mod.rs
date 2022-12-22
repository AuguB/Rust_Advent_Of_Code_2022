use std::{collections::HashMap};

pub fn compute_solution_1(input: String) {
    let (construction_string, procedure_string) = split_input(&input);
    let mut stacks = build_stacks(construction_string);
    perform_procedure(procedure_string, &mut stacks, false);
    print_top_of_stacks(stacks, construction_string);
}

fn print_top_of_stacks(stacks: HashMap<char, Vec<char>>, construction:&str){
    let line1 = construction.lines().rev().nth(0).unwrap();
    for (j, (box_nr)) in line1.chars().enumerate() {
        // If we are at a box
        if (j + 3) % 4 == 0 {
            // First create the stacks
            let top_of_stack = stacks.get(&box_nr).unwrap().last().unwrap();
            print!("{}", top_of_stack);
        }
    }
    println!();
}

fn perform_procedure(procedure: &str, stacks: &mut HashMap<char, Vec<char>>, retain_order:bool) {
    for step in procedure
        .lines()
        .map(|a| a.split(' ').collect::<Vec<&str>>())
    {
        let number_of_steps:usize = step[1].parse::<usize>().unwrap() as usize;

        let from: char = step[3].chars().collect::<Vec<char>>()[0];
        let to: char = step[5].chars().collect::<Vec<char>>()[0];

        if retain_order{
            let from = stacks.get_mut(&from).unwrap();
            let swap = &mut from.split_off(from.len()-number_of_steps);
            let to = stacks.get_mut(&to).unwrap();
            to.append(swap);
        }
        else{
            for _ in 0..number_of_steps{
                let from = stacks.get_mut(&from).unwrap();
                let char_to_be_swapped = from.pop().unwrap();
                let to = stacks.get_mut(&to).unwrap();
                to.push(char_to_be_swapped);
            }
        }

    }
}

fn build_stacks(construction: &str) -> HashMap<char, Vec<char>> {
    // Create new hashmap with mutable vectors
    let mut stacks: HashMap<char, Vec<char>> = HashMap::new();
    let mut line1: &str = "";
    // Loop over the lines
    for (i, line) in construction.lines().rev().enumerate() {
        if i == 0 {
            line1 = line.clone();
        }
        // For each char in the line
        for (j, (box_nr, char)) in line1.chars().zip(line.chars()).enumerate() {
            // If we are at a box
            if (j + 3) % 4 == 0 {
                // First create the stacks
                if i == 0 {
                    stacks.insert(box_nr, Vec::new());
                } else {
                    // Then fill them up
                    if char != ' ' {
                        stacks.get_mut(&box_nr).unwrap().push(char);
                    }
                }
            }
        }
    }
    return stacks;
}

pub fn compute_solution_2(input: String) {
    let (construction_string, procedure_string) = split_input(&input);
    let mut stacks = build_stacks(construction_string);
    perform_procedure(procedure_string, &mut stacks, true);
    print_top_of_stacks(stacks, construction_string);

}

fn split_input(input: &String) -> (&str, &str) {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let construction_string = split[0];
    let procedure_string = split[1];
    (construction_string, procedure_string)
}

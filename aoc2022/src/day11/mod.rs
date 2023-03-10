use regex::{Captures, Regex};
use std::cell::RefCell;

struct Monkey {
    items: RefCell<Vec<u64>>,
    operation: Box<dyn Fn(u64) -> u64>,
    divisor: u64,
    receiver_1: u64,
    receiver_2: u64,
    num_inspections: RefCell<u64>,
}

impl Monkey {
    fn from_capture(capt: Captures) -> Self {
        Monkey {
            items: RefCell::new(
                capt[1]
                    .replace(&[','], "")
                    .split_whitespace()
                    .map(|f| f.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            ),
            operation: Monkey::parse_operator(&capt),
            divisor: capt[4].parse::<u64>().unwrap(),
            receiver_1: capt[5].parse::<u64>().unwrap(),
            receiver_2: capt[6].parse::<u64>().unwrap(),
            num_inspections: RefCell::new(0),
        }
    }
    fn parse_operator(capt: &Captures) -> Box<dyn Fn(u64) -> u64> {
        match (&capt[2], &capt[3]) {
            ("+", "old") => Box::new(|f| f + f),
            ("*", "old") => Box::new(|f| f * f),
            ("+", a) => {
                let num = a.parse::<u64>().unwrap();
                Box::new(move |f| f + num.clone())
            }
            ("*", a) => {
                let num = a.parse::<u64>().unwrap();
                Box::new(move |f| f * num.clone())
            }
            _ => unreachable!(),
        }
    }
}

pub fn compute_solution_1(input: String) {
    let monkeys = make_monkeys(input);
    let worry_level_management_function = Box::new(|f: u64| f / 3);
    throw_stuff_around(monkeys, 20, worry_level_management_function)
}

pub fn compute_solution_2(input: String) {
    let monkeys = make_monkeys(input);
    let modulus: u64 = monkeys.iter().fold(1, |a, m| a * m.divisor);
    let worry_level_management_function = Box::new(move |f: u64| f % modulus);
    throw_stuff_around(monkeys, 10000, worry_level_management_function)
}

fn make_monkeys(input: String) -> Vec<Monkey> {
    let extreme_pattern = Regex::new(
        r".*\d{1}:\s*[^\n\d]*([^\n]*)[^+*]*([+*]{1}) (\d*|old)\n[^\d]*(\d*)[^\d]*(\d*)[^\d]*(\d*)",
    )
    .unwrap();
    extreme_pattern
        .captures_iter(&input)
        .map(|capture| Monkey::from_capture(capture))
        .collect::<Vec<Monkey>>()
}

fn throw_stuff_around(
    monkeys: Vec<Monkey>,
    n_rounds: u64,
    worry_level_management_function: Box<dyn Fn(u64) -> u64>,
) {
    for round in 0..n_rounds {
        for monkey_i in 0..monkeys.len() {
            let monkey = &monkeys[monkey_i];
            for item in monkey.items.borrow().iter() {
                let worry_level = worry_level_management_function((monkey.operation)(*item));
                let receiver = match worry_level % monkey.divisor {
                    0 => monkey.receiver_1,
                    _ => monkey.receiver_2,
                };
                let mut monkey_items = monkeys[receiver as usize].items.borrow_mut();
                monkey_items.push(worry_level);
                monkey.num_inspections.replace_with(|old| *old + 1);
            }
            monkey.items.replace(Vec::new());
        }
    }
    let mut monkey_scores = monkeys
        .iter()
        .map(|a| a.num_inspections.replace(0))
        .collect::<Vec<u64>>();
    monkey_scores.sort();
    monkey_scores.reverse();
    println!("{}", monkey_scores[0] * monkey_scores[1]);
}

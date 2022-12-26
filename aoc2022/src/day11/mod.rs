use std::cell::RefCell;

use regex::{Captures, Regex};

struct Monkey {
    items: RefCell<Vec<u32>>,
    operation: Box<dyn Fn(u32) -> u32>,
    divisor: u32,
    true_case: u32,
    false_case: u32,
    num_inspections: RefCell<u32>,
}

impl Monkey {
    fn from_capture(capt: Captures) -> Self {
        Monkey {
            items: RefCell::new(
                capt[1]
                    .replace(&[','], "")
                    .split_whitespace()
                    .map(|f| f.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            ),
            operation: Monkey::parse_operator(&capt),
            divisor: capt[4].parse::<u32>().unwrap(),
            true_case: capt[5].parse::<u32>().unwrap(),
            false_case: capt[6].parse::<u32>().unwrap(),
            num_inspections: RefCell::new(0),
        }
    }
    fn parse_operator(capt: &Captures) -> Box<dyn Fn(u32) -> u32> {
        match (&capt[2], &capt[3]) {
            ("+", "old") => Box::new(|f| f + f),
            ("*", "old") => Box::new(|f| f * f),
            ("+", a) => {
                let num = a.parse::<u32>().unwrap();
                Box::new(move |f| f + num.clone())
            }
            ("*", a) => {
                let num = a.parse::<u32>().unwrap();
                Box::new(move |f| f * num.clone())
            }
            _ => unreachable!(),
        }
    }
    fn money_to_pass_to_idx(&self, worry_level: u32) -> u32 {
        return if ((self.operation)(worry_level) / 3) % 3 == 0 {
            self.true_case
        } else {
            self.false_case
        };
    }
}

pub fn compute_solution_1(input: String) {
    let extreme_pattern = Regex::new(
        r".*\d{1}:\s*[^\n\d]*([^\n]*)[^+*]*([+*]{1}) (\d*|old)\n[^\d]*(\d*)[^\d]*(\d*)[^\d]*(\d*)",
    )
    .unwrap();

    let monkeys = extreme_pattern
        .captures_iter(&input)
        .map(|capture| Monkey::from_capture(capture))
        .collect::<Vec<Monkey>>();

    for round in 0..20 {
        for monkey_i in 0..monkeys.len() {
            let monkey = &monkeys[monkey_i];
            for item in monkey.items.borrow().iter() {
                let new_worry_level = (monkey.operation)(*item) / 3;
                let monkey_to_pass_to_idx = if new_worry_level % monkey.divisor == 0 {
                    monkey.true_case
                } else {
                    monkey.false_case
                };
                let mut monkey_items = monkeys[monkey_to_pass_to_idx as usize].items.borrow_mut();
                monkey_items.push(new_worry_level);
                monkey.num_inspections.replace_with(|old| *old + 1);
            }
            monkey.items.replace(Vec::new());
        }
    }

    let monkey_scores = monkeys
        .iter()
        .for_each(|a| println!("{}", &a.num_inspections.borrow()));
}

pub fn compute_solution_2(input: String) {}

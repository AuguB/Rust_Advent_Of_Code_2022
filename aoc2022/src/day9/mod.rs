use std::{
    cmp::{self, max},
    collections::HashSet,
    fmt::Display,
    str::FromStr,
};

use regex::{internal::Inst, Captures, Regex};

pub fn compute_solution_1(input: String) {
    trace_rope(2, input);
}

pub fn compute_solution_2(input: String) {
    trace_rope(10, input);
}

pub fn trace_rope(length: u32, input: String) {
    let pattern = Regex::new(r"(\w{1}) (\d*)").unwrap();
    let instructions = pattern
        .captures_iter(&input)
        .map(|m| (0..(m[2].parse::<i32>().unwrap())).map(move |_| Instruction::from_string(&m[1])))
        .flatten();
    let mut rope: Rope = Rope::make_at(length, (0, 0));
    let mut t_locs: HashSet<(i32, i32)> = HashSet::new();
    t_locs.insert(rope.tail());

    for inst in instructions {
        rope.apply_instruction(inst);
        t_locs.insert(rope.tail());
    }

    println!("{}", t_locs.len());
}

enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Instruction {
    fn from_string(m:&str) -> Self {
        match m {
            "U" => Instruction::Up,
            "D" => Instruction::Down,
            "L" => Instruction::Left,
            "R" => Instruction::Right,
            _ => panic!("No valid direction"),
        }
    }

    fn apply_on(&self, loc: (i32, i32)) -> (i32, i32) {
        let (x, y) = loc;
        match self {
            Instruction::Up => (x, y + 1),
            Instruction::Down => (x, y - 1),
            Instruction::Left => (x - 1, y),
            Instruction::Right => (x + 1, y),
        }
    }
}

struct Rope {
    knots: Vec<(i32, i32)>,
}

impl Rope {
    fn make_at(length: u32, loc: (i32, i32)) -> Self {
        Rope {
            knots: vec![(0, 0); length as usize],
        }
    }

    fn apply_instruction(&mut self, inst: Instruction) {
        self.knots[0] = inst.apply_on(self.knots[0]);
        for knot_i in 1..self.knots.len() {
            self.knots[knot_i] = Rope::compute_new_loc(self.knots[knot_i], self.knots[knot_i - 1]);
        }
    }

    fn compute_new_loc(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
        let (dif_x, dif_y) = (head.0 - tail.0, head.1 - tail.1);
        if max(dif_x.abs(), dif_y.abs()) > 1 {
            return match (dif_x.abs(), dif_y.abs()) {
                (_, 0) => (tail.0 + dif_x.signum(), tail.1),
                (0, _) => (tail.0, tail.1 + dif_y.signum()),
                _ => (tail.0 + dif_x.signum(), tail.1 + dif_y.signum()),
            };
        }
        tail
    }

    fn tail(&self) -> (i32, i32) {
        self.knots.last().copied().unwrap()
    }
}

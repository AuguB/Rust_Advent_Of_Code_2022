use std::{cmp, collections::HashSet, fmt::Display, str::FromStr};

use regex::{internal::Inst, Captures, Regex};

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

    fn compute_new_loc(this_loc: (i32, i32), parent_loc: (i32, i32)) -> (i32, i32) {
        let (this_x, this_y) = this_loc;
        let (parent_x, parent_y) = parent_loc;
        let dif_x = parent_x - this_x;
        let dif_y = parent_y - this_y;
        if (dif_x.abs() <= 1) & (dif_y.abs() <= 1) {
            return this_loc;
        } else {
            match (dif_x.abs(), dif_y.abs()) {
                (2, 0) => (this_x + dif_x.signum(), this_y),
                (0, 2) => (this_x, this_y + dif_y.signum()),
                _ => (this_x + dif_x.signum(), this_y + dif_y.signum()),
            }
        }
    }

    fn tail(&self) -> Option<(i32, i32)> {
        self.knots.last().copied()
    }
}

struct Instruction {
    direction: Direction,
    length: i32,
}

impl Instruction {
    fn from_match(m: Captures) -> Self {
        Instruction {
            direction: Direction::from_string(&m[1]),
            length: m[2].parse::<i32>().unwrap(),
        }
    }

    fn apply_on(&self, loc: (i32, i32)) -> (i32, i32) {
        let (x, y) = loc;
        match self.direction {
            Direction::Up => (x, y + self.length),
            Direction::Down => (x, y - self.length),
            Direction::Left => (x - self.length, y),
            Direction::Right => (x + self.length, y),
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_string(name: &str) -> Self {
        match name {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction"),
        }
    }
}

fn close_to(h: (i32, i32), t: (i32, i32)) -> bool {
    ((h.0 - t.0).abs() <= 1) & ((h.1 - t.1).abs() <= 1)
}

pub fn compute_solution_1(input: String) {
    trace_rope(2, input);
}

pub fn trace_rope(length: u32, input: String) {
    let pattern = Regex::new(r"(\w{1}) (\d*)").unwrap();
    let instructions = pattern
        .captures_iter(&input)
        .map(|m| {
            (0..(m[2].parse::<i32>().unwrap())).map(move |_| Instruction {
                direction: Direction::from_string(&m[1]),
                length: 1,
            })
        })
        .flatten();
    let mut rope: Rope = Rope::make_at(length, (0, 0));
    let mut t_locs: HashSet<(i32, i32)> = HashSet::new();
    t_locs.insert(rope.tail().unwrap());

    for inst in instructions {
        rope.apply_instruction(inst);
        t_locs.insert(rope.tail().unwrap());
    }

    println!("{}", t_locs.len());
}
pub fn compute_solution_2(input: String) {
    trace_rope(10, input);
}

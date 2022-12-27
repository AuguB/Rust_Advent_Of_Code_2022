use ndarray::Array2;
use regex::{Captures, Match, Regex};
use std::{
    cmp::{max, min},
    collections::HashMap,
    ffi::IntoStringError,
};
use std::{thread, time};

struct Cave {
    map: HashMap<(i32, i32), char>,
    top_left: (i32, i32),
    bottom_right: (i32, i32),
    floor: bool,
}

pub fn compute_solution_1(input: String) {
    let mut cave = Cave::new(input, false);
    let mut count = 0;
    while cave.this_grain_of_sand_will_come_to_rest((0, 500)) {
        count += 1;
    }
    println!("{}", count);
    cave.show();
}

pub fn compute_solution_2(input: String) {
    let mut cave = Cave::new(input, true);
    let mut count = 0;
    while !cave.full() {
        cave.this_grain_of_sand_will_come_to_rest((0, 500));
        count += 1;
    }
    println!("{}", count);
    cave.show();
}

impl Cave {
    fn new(input: String, floor: bool) -> Self {
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let pattern = Regex::new(r"(\d*),(\d*)").unwrap();
        for line in input.lines() {
            let captures = pattern.captures_iter(line).collect::<Vec<Captures>>();
            for i in 0..captures.len() - 1 {
                let from = (parse_int(&captures[i], 2), parse_int(&captures[i], 1));
                let to = (parse_int(&captures[i + 1], 2), parse_int(&captures[i + 1], 1));
                create_wall(from, to, &mut map);
            }
        }
        let top_left = map.keys().fold((0, 500), |a, c| (min(a.0, c.0), min(a.1, c.1)));
        let bottom_right = map.keys().fold((0, 500), |a, c| (max(a.0, c.0), max(a.1, c.1)));
        Cave { map, top_left, bottom_right, floor }
    }

    fn get_map_arr(&self) -> Array2<char> {
        let width = self.bottom_right.1 - self.top_left.1 + 1;
        let height = self.bottom_right.0 - self.top_left.0 + 1 + if self.floor { 2 } else { 0 };
        let mut arr = Array2::from_elem((height as usize, width as usize), '.');
        self.map.iter().for_each(|((r, c), v)| arr[[(*r - self.top_left.0) as usize, (*c - self.top_left.1) as usize]] = *v);
        if self.floor {
            (self.top_left.1..=self.bottom_right.1).for_each(|j| arr[[(height - 1 - self.top_left.0) as usize, (j - self.top_left.1) as usize]] = '#')
        }
        arr
    }

    fn show(&self) {
        for row in self.get_map_arr().rows() {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }

    fn this_grain_of_sand_will_come_to_rest(&mut self, loc: (i32, i32)) -> bool {
        if self.out_of_range(loc) {
            return false;
        }
        loop {
            // Check if we are on the floor
            if self.floor && (loc.0 == self.bottom_right.0 + 1) {
                self.map.insert(loc, 'o');
                self.update_range(loc);
                return true;
            }
            if !self.map.contains_key(&(loc.0 + 1, loc.1)) {
                return self.this_grain_of_sand_will_come_to_rest((loc.0 + 1, loc.1));
            } else if !self.map.contains_key(&(loc.0 + 1, loc.1 - 1)) {
                return self.this_grain_of_sand_will_come_to_rest((loc.0 + 1, loc.1 - 1));
            } else if !self.map.contains_key(&(loc.0 + 1, loc.1 + 1)) {
                return self.this_grain_of_sand_will_come_to_rest((loc.0 + 1, loc.1 + 1));
            } else {
                self.map.insert(loc, 'o');
                self.update_range(loc);
                return true;
            }
        }
    }

    fn out_of_range(&self, loc: (i32, i32)) -> bool {
        if self.floor {
            return loc.0 >= self.bottom_right.0 + 2;
        }
        return loc.0 > self.bottom_right.0 || loc.1 > self.bottom_right.1 || loc.1 < self.top_left.1;
    }

    fn full(&self) -> bool {
        self.map.contains_key(&(0, 500))
    }

    fn update_range(&mut self, loc: (i32, i32)) {
        self.bottom_right.1 = max(self.bottom_right.1, loc.1);
        self.top_left.1 = min(self.top_left.1, loc.1);
    }
}

fn parse_int(capture: &Captures, index: usize) -> i32 {
    return capture.get(index).unwrap().as_str().parse::<i32>().unwrap();
}

fn create_wall(from: (i32, i32), to: (i32, i32), map: &mut HashMap<(i32, i32), char>) {
    let difx = to.0 - from.0;
    let dify = to.1 - from.1;
    match (difx.signum(), dify.signum()) {
        (0, 1) => (from.1..=to.1).for_each(|c| {
            map.insert((from.0, c), '#');
        }),
        (0, -1) => (to.1..=from.1).for_each(|c| {
            map.insert((from.0, c), '#');
        }),
        (1, 0) => (from.0..=to.0).for_each(|c| {
            map.insert((c, from.1), '#');
        }),
        (-1, 0) => (to.0..=from.0).for_each(|c| {
            map.insert((c, from.1), '#');
        }),
        _ => println!("This wall is weird"),
    }
}

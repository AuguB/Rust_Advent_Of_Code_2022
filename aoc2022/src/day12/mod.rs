use std::{
    cell::{Cell, RefCell},
    collections::BinaryHeap,
};

use itertools::Itertools;
use ndarray::{prelude::*, OwnedRepr, RawData, ViewRepr};
use regex::Regex;

pub fn compute_solution_1(input: String) {
    // Create a landscape
    let landscape: MountainRange = MountainRange::new(input);

    // Create a Heap with all our paths
    let mut heap: BinaryHeap<Path> = BinaryHeap::new();

    // Create a path at the start position
    let initial_path = Path::new(vec![landscape.start_loc], &landscape);

    // throw it in a heap
    heap.push(initial_path);

    // Store solution
    let mut solution = Path::new(vec![(0, 0)], &landscape);

    let mut n_evaluations = 0;

    // While the heap is not empty, and solution has not been found:
    while !heap.is_empty() {
        //  take the next value from the heap
        let next_path = heap.pop().unwrap();
        let next_endpoint = next_path.get_endpoint();
        landscape.visited_map.borrow_mut()[[next_endpoint.0 as usize, next_endpoint.1 as usize]] =
            true;

        //  if is the solution
        if next_path.is_solution() {
            solution = next_path;
            break;
        } else {
            for neighbor in landscape.neighbors(next_path.get_endpoint()) {
                let mut cloned_path = next_path.path.clone();
                cloned_path.push(neighbor);
                heap.push(Path {
                    path: cloned_path,
                    mountainrange: &landscape,
                })
            }
        }
        n_evaluations += 1;
    }
    println!(
        "Found a path of len {} in {} evaluations",
        solution.length(),
        n_evaluations
    );
    solution.draw();
}

pub fn compute_solution_2(input: String) {
    // Create a landscape
    let landscape: MountainRange = MountainRange::new(input);

    // Create a Heap with all our paths
    let mut heap: BinaryHeap<Path> = BinaryHeap::new();

    // Create a path at the start position
    let initial_path = Path::new(vec![landscape.start_loc], &landscape);

    // throw it in a heap
    heap.push(initial_path);

    // Store solution
    let mut solution = Path::new(vec![(0, 0)], &landscape);

    let mut n_evaluations = 0;

    // While the heap is not empty, and solution has not been found:
    while !heap.is_empty() {
        //  take the next value from the heap
        let next_path = heap.pop().unwrap();
        let next_endpoint = next_path.get_endpoint();
        landscape.visited_map.borrow_mut()[[next_endpoint.0 as usize, next_endpoint.1 as usize]] =
            true;

        //  if is the solution
        if next_path.is_solution() {
            solution = next_path;
            break;
        } else {
            for neighbor in landscape.neighbors(next_path.get_endpoint()) {
                let mut cloned_path = next_path.path.clone();
                cloned_path.push(neighbor);
                heap.push(Path {
                    path: cloned_path,
                    mountainrange: &landscape,
                })
            }
        }
        n_evaluations += 1;
    }
    println!(
        "Found a path of len {} in {} evaluations",
        solution.length(),
        n_evaluations
    );
    solution.draw();
}

struct Path<'a> {
    path: Vec<(i32, i32)>,
    mountainrange: &'a MountainRange,
}

impl<'a> Path<'a> {
    fn new(path: Vec<(i32, i32)>, mountainrange: &'a MountainRange) -> Path<'a> {
        Path {
            path,
            mountainrange,
        }
    }
    fn length(&self) -> u32 {
        (self.path.len() as u32) - 1
    }
    fn heuristic(&self) -> u32 {
        self.mountainrange.heuristic(self.path[self.path.len() - 1])
    }
    fn priority(&self) -> i32 {
        -(self.length() as i32 + self.heuristic() as i32)
    }
    fn is_solution(&self) -> bool {
        self.mountainrange
            .is_endpoint(self.path[self.path.len() - 1])
    }
    fn get_endpoint(&self) -> (i32, i32) {
        *self.path.last().unwrap()
    }
    fn draw(&self) {
        let shape = self.mountainrange.heightmap.shape();
        let mut chars = Array2::from_elem((shape[0], shape[1]), '.');

        for i in 0..self.path.len() - 1 {
            let j = i as usize;
            let this = self.path[j];
            let next = self.path[j + 1];
            match (next.0 - this.0, next.1 - this.1) {
                (-1, 0) => chars[[this.0 as usize, this.1 as usize]] = '^',
                (1, 0) => chars[[this.0 as usize, this.1 as usize]] = 'v',
                (0, 1) => chars[[this.0 as usize, this.1 as usize]] = '>',
                (0, -1) => chars[[this.0 as usize, this.1 as usize]] = '<',
                _ => panic!("EHMAGAWD"),
            }
            chars[[next.0 as usize, next.1 as usize]] = 'E'
        }
        for i in chars.rows() {
            for j in i {
                print!("{j}");
            }
            println!();
        }
    }
}

impl PartialEq for Path<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.priority() == other.priority()
    }
}

impl PartialOrd for Path<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority().partial_cmp(&other.priority())
    }
}

impl Eq for Path<'_> {}

impl Ord for Path<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

struct MountainRange {
    heightmap: Array2<char>,
    visited_map: RefCell<Array2<bool>>,
    n_cols: i32,
    n_rows: i32,
    start_loc: (i32, i32),
    end_loc: (i32, i32),
}

impl MountainRange {
    fn new(input: String) -> Self {
        let n_rows = input.lines().count();
        let n_cols = input.replace("\n", "").chars().count() / n_rows;

        let binding = input.replace("\n", "");
        let mut chars = binding.chars().collect::<Vec<char>>();

        let start_loc = chars.iter().position(|f| *f == 'S').unwrap();
        let end_loc = chars.iter().position(|f| *f == 'E').unwrap();

        let start_tup = ((start_loc / n_cols) as i32, (start_loc % n_cols) as i32);
        let end_tup = ((end_loc / n_cols) as i32, (end_loc % n_cols) as i32);

        chars[start_loc] = 'a';
        chars[end_loc] = 'z';

        let visited = Array2::from_elem((n_rows, n_cols), false);

        MountainRange {
            heightmap: Array2::from_shape_vec((n_rows, n_cols), chars).unwrap(),
            n_cols: n_cols as i32,
            n_rows: n_rows as i32,
            start_loc: start_tup,
            end_loc: end_tup,
            visited_map: RefCell::new(visited),
        }
    }

    fn neighbors(&self, loc: (i32, i32)) -> Vec<(i32, i32)> {
        let offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        offsets
            .iter()
            .map(|(row, col)| (row + loc.0, col + loc.1))
            .filter(|(row, col)| 0 <= *row && *row < self.n_rows && 0 <= *col && *col < self.n_cols)
            .filter(|(row, col)| {
                self.heightmap[[*row as usize, *col as usize]] as i32
                    <= self.heightmap[[loc.0 as usize, loc.1 as usize]] as i32 + 1
            })
            .filter(|(row, col)| !self.visited_map.borrow()[[*row as usize, *col as usize]])
            .collect::<Vec<(i32, i32)>>()
    }

    fn show_range(&self) {
        println!("{}", self.heightmap.to_string());
    }
    fn show_visited(&self) {
        println!("{}", self.visited_map.borrow().to_string());
    }

    fn heuristic(&self, loc: (i32, i32)) -> u32 {
        ((loc.0 - self.end_loc.0).abs() + (loc.1 - self.end_loc.1).abs())
            .try_into()
            .unwrap()
    }

    fn is_endpoint(&self, loc: (i32, i32)) -> bool {
        loc.0 == self.end_loc.0 && loc.1 == self.end_loc.1
    }
}

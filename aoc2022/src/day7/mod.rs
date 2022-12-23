//cargo watch -x test
use std::collections::HashMap;

use regex::{CaptureMatches, Regex};

pub fn compute_solution_1(_input: String) {
    let commands = Regex::new(r"\$ (cd (..|/|\w+){1}|ls)|(\d+)\s([\w\.]+)|dir (\w*)").unwrap();
    let mut _matches = commands.captures_iter(&_input);
}

pub fn compute_solution_2(_input: String) {}

enum Tree<'a> {
    Branch(
        Option<Box<&'a Tree<'a>>>,
        String,
        HashMap<&'a str, Tree<'a>>,
    ),
    Leaf(Option<Box<&'a Tree<'a>>>, String, u32),
}

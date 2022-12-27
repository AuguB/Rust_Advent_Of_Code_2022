use std::{
    borrow::Borrow,
    char,
    collections::{BinaryHeap, HashMap, HashSet},
};

use ndarray::Array2;

pub fn compute_solution_1(input: String) {
    let nodes = parse_mountains(input);
    let end_loc = nodes.iter().filter(|(loc, (char, _))| *char == 'E').collect::<Vec<(&(i32, i32), &(char, bool))>>();
    let end_loc = end_loc[0].0.clone();
    let manhattan_distance = |n: (i32, i32)| (n.0 - end_loc.0).abs() + (n.1 - end_loc.1).abs();
    a_star(nodes, 'S', 'E', manhattan_distance, false);
}

pub fn compute_solution_2(input: String) {
    let nodes = parse_mountains(input);
    let heuristic = |n: (i32, i32)| 0;
    a_star(nodes, 'E', 'a', heuristic, true);
}

pub fn a_star<F: Fn((i32, i32)) -> i32>(
    mut nodes: HashMap<(i32, i32), (char, bool)>,
    start: char,
    search: char,
    heuristic: F,
    invert_direction: bool,
) {
    let start_loc =
        nodes.iter().filter(|(loc, (char, _))| *char == start).collect::<Vec<(&(i32, i32), &(char, bool))>>();

    if start_loc.len() == 1 {
        let start_loc = start_loc[0];
        let n_nodes = nodes.len() as i32;
        let mut queue: Vec<(i32, i32, (i32, i32))> = Vec::new();
        let mut parents: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut n_nodes_visited = 0;
        queue.push((0, 0, *start_loc.0));

        while !queue.is_empty() {
            queue.sort();
            let (priority, length, loc) = queue.pop().unwrap();
            let (char, visited) = nodes.get(&loc).unwrap();
            if *char == search {
                println!("Path with length {} found, visited {} nodes", length, n_nodes_visited);
                draw_path(loc, nodes, parents, invert_direction);
                break;
            } else {
                for n in neighbors(loc, &nodes, invert_direction) {
                    if !nodes.get(&n).unwrap().1 {
                        match queue.iter().enumerate().filter(|(i, a)| a.2 == n).last() {
                            None => {
                                queue.push((-(length + 1 + heuristic(n)), length + 1, n));
                                parents.insert(n, loc);
                            }
                            Some((i, (f, l, neighbor_loc))) => {
                                if length + 1 < *l {
                                    parents.insert(n, loc);
                                    queue.remove(i);
                                    queue.push((-(length + 1 + heuristic(n)), length + 1, n));
                                }
                            }
                        }
                    }
                }
            };
            nodes.get_mut(&loc).unwrap().1 = true;
            n_nodes_visited += 1;
        }
    } else {
        println!("Ambiguous start node")
    }
}

fn draw_path(
    mut loc: (i32, i32),
    nodes: HashMap<(i32, i32), (char, bool)>,
    parents: HashMap<(i32, i32), (i32, i32)>,
    invert_direction: bool,
) {
    let n_rows = nodes.keys().map(|a| a.0).max().unwrap();
    let n_cols = nodes.keys().map(|a| a.1).max().unwrap();
    let mut map = Array2::from_elem((n_rows as usize + 1, n_cols as usize + 1), '.');
    loop {
        match parents.get(&loc) {
            None => break,
            Some(parent) => {
                map[[parent.0 as usize, parent.1 as usize]] = match (parent.0 - loc.0, parent.1 - loc.1) {
                    (1, 0) => '^',
                    (-1, 0) => 'v',
                    (0, 1) => '<',
                    (0, -1) => '>',
                    _ => unreachable!(),
                };
                loc = parent.clone();
            }
        }
    }
    for i in map.rows() {
        for j in i {
            print!("{j}");
        }
        println!();
    }
}

fn parse_mountains(input: String) -> HashMap<(i32, i32), (char, bool)> {
    let n_rows = input.lines().count();
    let n_elements = input.replace("\n", "").chars().count();
    let n_cols = n_elements / n_rows;
    HashMap::from_iter(
        input
            .replace("\n", "")
            .chars()
            .enumerate()
            .map(|(i, v)| (((i / n_cols) as i32, (i % n_cols) as i32), (v, false))),
    )
}

fn neighbors(loc: (i32, i32), nodes: &HashMap<(i32, i32), (char, bool)>, invert_direction: bool) -> Vec<(i32, i32)> {
    let offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    offsets
        .iter()
        .map(|o| (o.0 + loc.0, o.1 + loc.1))
        .filter(|n| nodes.contains_key(n)) // The nodes exists
        .filter(|n| !nodes.get(&n).unwrap().1)
        .filter(|n| reachable(loc, *n, &nodes, invert_direction)) // The node is reachable from my position
        .collect::<Vec<(i32, i32)>>()
}

fn char_height(char: char) -> usize {
    match char {
        'E' => 25,
        'S' => 0,
        a => a as usize - 97,
    }
}

fn reachable(
    from: (i32, i32),
    to: (i32, i32),
    nodes: &HashMap<(i32, i32), (char, bool)>,
    invert_direction: bool,
) -> bool {
    let (from, to) = if invert_direction { (to, from) } else { (from, to) };
    let from_height = char_height(nodes.get(&from).unwrap().0);
    let to_height = char_height(nodes.get(&to).unwrap().0);
    from_height + 1 >= to_height
}

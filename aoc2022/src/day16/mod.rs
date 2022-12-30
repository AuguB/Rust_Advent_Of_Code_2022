use regex::Regex;
use std::{cmp::min, collections::BinaryHeap};

pub fn compute_solution_1(input: String) {
    let (n_nodes, nodes, flows, neighbors, distmat) = parse_input(&input);

    let relevant_nodes = flows
        .iter()
        .enumerate()
        .filter(|(_, v)| **v > (0 as isize))
        .map(|(k, _)| k as isize)
        .collect::<Vec<isize>>();

    let distance_to_closest_relevant_node = nodes
        .iter()
        .map(|i| {
            relevant_nodes
                .iter()
                .filter(|r| i.0 != **r)
                .map(|r| distmat[(i.0 * n_nodes + *r) as usize])
                .min()
                .unwrap()
        })
        .collect::<Vec<isize>>();

    let mut best_score = 0;
    // This is a tuple of priority, score, timer, current, opened
    let mut best_tup: (isize, isize, isize, isize, Vec<isize>) = (0, 0, 0, 0, vec![]);

    let mut queue = BinaryHeap::new();
    let start_index = nodes.iter().position(|f| f.1 == "AA").unwrap() as isize;
    let init: (isize, isize, isize, isize, Vec<isize>) = (0, 0, 30, start_index, vec![]);
    queue.push(init);
    while !queue.is_empty() {
        let (priority, score, timer, current, opened) = queue.pop().unwrap();
        if priority < 0 {
            break;
        }
        if score > best_score {
            best_score = score;
            best_tup = (priority, score, timer, current, opened.clone());
        }

        let to_visit = relevant_nodes.iter().filter(|k| !opened.contains(k)).map(|k| *k).collect::<Vec<isize>>();
        for next in &to_visit {
            let distance_to_next = distmat[(current * n_nodes + next) as usize];
            let new_timer = timer - (distance_to_next + 1);
            let new_score = score + new_timer * flows[*next as usize];
            let new_priority = new_timer - distance_to_closest_relevant_node[*next as usize] - 1;
            let new_current = next;
            let mut new_opened = opened.clone();
            new_opened.push(next.clone());
            if new_timer > 0 {
                queue.push((new_priority, new_score, new_timer, *new_current, new_opened));
            }
        }
    }
    let best_route = best_tup.4;
    println!("{best_route:?}: {}", best_tup.1);
}

pub fn compute_solution_2(input: String) {}

// Returns n_nodes, nodes, flows, neighbors, distances
fn parse_input<'a>(input: &'a String) -> (isize, Vec<(isize, &'a str)>, Vec<isize>, Vec<Vec<isize>>, Vec<isize>) {
    let valve_pattern = Regex::new(r"Valve (\w\w) has.*=(\d*).*ves? ([\w, ]*)").unwrap();
    let neighbor_pattern = Regex::new(r"\w\w").unwrap();

    let nodes = valve_pattern
        .captures_iter(&input)
        .map(|c| c.get(1).unwrap().as_str())
        .enumerate()
        .map(|(i, v)| (i as isize, v))
        .collect::<Vec<(isize, &str)>>();

    let flows = valve_pattern
        .captures_iter(&input)
        .map(|c| c.get(2).unwrap().as_str().parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    let neighbors = valve_pattern
        .captures_iter(&input)
        .map(|c| {
            neighbor_pattern
                .captures_iter(&c.get(3).unwrap().as_str())
                .map(|neighbor| {
                    nodes
                        .iter()
                        .position(|node| node.1 == neighbor.get(0).unwrap().as_str())
                        .map(|v| v as isize)
                        .unwrap()
                })
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    let n_nodes = nodes.iter().count() as isize;
    let mut admat = vec![200000; n_nodes.pow(2).try_into().unwrap()];
    neighbors.iter().enumerate().for_each(|(from, list)| {
        admat[from * (n_nodes as usize) + from] = 0;
        list.iter()
            .for_each(|to| admat[(from * (n_nodes as usize) + (*to as usize)) as usize] = 1)
    });

    floyd_warshal(&mut admat, n_nodes);
    (n_nodes, nodes, flows, neighbors, admat)
}

fn floyd_warshal(d: &mut Vec<isize>, n: isize) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[(i * n + j) as usize] = min(d[(i * n + j) as usize], d[(i * n + k) as usize] + d[(k * n + j) as usize])
            }
        }
    }
}

use itertools::Itertools;
use regex::Regex;
use std::{cmp::min, collections::BinaryHeap};

pub fn compute_solution_1(input: String) {
    find_best_path(input, 1, 30);
}

pub fn compute_solution_2(input: String) {
    find_best_path(input, 2, 26);
}

fn find_best_path(input: String, n_players: usize, time: i16) {
    let (n_nodes, nodes, flows, neighbors, distmat) = parse_input(&input);

    let relevant_nodes = flows
        .iter()
        .enumerate()
        .filter(|(_, v)| **v > (0 as i16))
        .map(|(k, _)| k as i16)
        .collect::<Vec<i16>>();

    let mut best_score = 0;
    let mut best_tup: (i16, i16, Vec<i16>, Vec<i16>, Vec<Vec<i16>>) = (0, 0, vec![], vec![], vec![vec![]; n_players]);
    //timer, score, currents, headings, openeds

    let start_index = nodes.iter().position(|f| f.1 == "AA").unwrap() as i16;
    let init: (i16, i16, Vec<i16>, Vec<i16>, Vec<Vec<i16>>) = (
        time + 1, // We need to give ourselves 1 extra minute, because we start at 0 and we also set our initial heading to 0, so nothing will happen the first minute
        0,
        vec![start_index; n_players],
        vec![start_index; n_players],
        vec![vec![]; n_players],
    );

    let mut queue = BinaryHeap::new();
    queue.push(init);

    while !queue.is_empty() {
        // do stuff
        // Get the next tuple, do bookkeeping
        let (timer, score, cur, head, open) = queue.pop().unwrap();
        if timer < 0 {
            break;
        }
        if score > best_score {
            best_score = score;
            best_tup = (timer, score, cur.clone(), head.clone(), open.clone());
        }
        let mut new_score = score;
        let mut arrived_players: Vec<i16> = vec![];
        let mut new_cur = cur.clone();
        let mut new_open = open.clone();

        // For each player
        for p in 0..n_players {
            // Check if we are on the heading.
            if cur[p] == head[p] {
                let points_gained = (timer - 1) * flows[cur[p] as usize];
                if points_gained > 0 {
                    new_score += (timer - 1) * flows[cur[p] as usize];
                    new_open[p as usize].push(cur[p].clone());
                }
                arrived_players.push(p as i16);
            } else {
                // Find the neighbor that is closest to the heading
                let next = neighbors[cur[p] as usize]
                    .iter()
                    .map(|v| (distmat[(v * n_nodes + head[p]) as usize], v))
                    .min()
                    .unwrap()
                    .1;
                new_cur[p] = next.clone();
            }
        }

        if arrived_players.len() > 0 {
            let mut no_options = new_open.clone().iter().flatten().map(|k| *k).collect::<Vec<i16>>();
            let mut relevant_heads = head
                .iter()
                .enumerate()
                .filter(|(i, k)| !arrived_players.contains(&(*i as i16)))
                .map(|(i, v)| *v)
                .collect::<Vec<i16>>();
            no_options.append(&mut relevant_heads);
            let options = relevant_nodes
                .clone()
                .iter()
                .filter(|k| !no_options.contains(k))
                .map(|k| *k)
                .collect::<Vec<i16>>();
            if options.len() > 0 {
                for perm in options.iter().permutations(arrived_players.len()) {
                    let mut new_head = head.clone();
                    for (p, option) in perm.iter().enumerate() {
                        new_head[arrived_players[p] as usize] = **option;
                    }
                    queue.push((timer - 1, new_score, new_cur.clone(), new_head, new_open.clone()));
                }
            } else {
                let mut new_head = head.clone();
                for p in arrived_players.iter() {
                    new_head[*p as usize] = 0;
                }
                queue.push((timer - 1, new_score, new_cur.clone(), new_head.clone(), new_open.clone()));
            }
        } else {
            queue.push((timer - 1, new_score, new_cur.clone(), head.clone(), new_open.clone()));
        }
    }
    if queue.is_empty() {
        println!("Emptied the whole queue");
    }
    println!("Best routes:");
    for (i, v) in best_tup.4.iter().enumerate() {
        println!("Player {}: {v:?}", i);
    }
    println!("Best score: {best_score}, tup {best_tup:?}")
}

// Returns n_nodes, nodes, flows, neighbors, distances
fn parse_input<'a>(input: &'a String) -> (i16, Vec<(i16, &'a str)>, Vec<i16>, Vec<Vec<i16>>, Vec<i16>) {
    let valve_pattern = Regex::new(r"Valve (\w\w) has.*=(\d*).*ves? ([\w, ]*)").unwrap();
    let neighbor_pattern = Regex::new(r"\w\w").unwrap();

    let nodes = valve_pattern
        .captures_iter(&input)
        .map(|c| c.get(1).unwrap().as_str())
        .enumerate()
        .map(|(i, v)| (i as i16, v))
        .collect::<Vec<(i16, &str)>>();

    let flows = valve_pattern
        .captures_iter(&input)
        .map(|c| c.get(2).unwrap().as_str().parse::<i16>().unwrap())
        .collect::<Vec<i16>>();

    let neighbors = valve_pattern
        .captures_iter(&input)
        .map(|c| {
            neighbor_pattern
                .captures_iter(&c.get(3).unwrap().as_str())
                .map(|neighbor| {
                    nodes
                        .iter()
                        .position(|node| node.1 == neighbor.get(0).unwrap().as_str())
                        .map(|v| v as i16)
                        .unwrap()
                })
                .collect::<Vec<i16>>()
        })
        .collect::<Vec<Vec<i16>>>();

    let n_nodes = nodes.iter().count() as i16;
    let mut admat = vec![59; n_nodes.pow(2).try_into().unwrap()];
    neighbors.iter().enumerate().for_each(|(from, list)| {
        admat[from * (n_nodes as usize) + from] = 0;
        list.iter()
            .for_each(|to| admat[(from * (n_nodes as usize) + (*to as usize)) as usize] = 1)
    });

    floyd_warshal(&mut admat, n_nodes);
    (n_nodes, nodes, flows, neighbors, admat)
}

fn floyd_warshal(d: &mut Vec<i16>, n: i16) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[(i * n + j) as usize] = min(d[(i * n + j) as usize], d[(i * n + k) as usize] + d[(k * n + j) as usize])
            }
        }
    }
}

// let (n_nodes, nodes, flows, neighbors, distmat) = parse_input(&input);

// let relevant_nodes = flows
//     .iter()
//     .enumerate()
//     .filter(|(_, v)| **v > (0 as i16))
//     .map(|(k, _)| k as i16)
//     .collect::<Vec<i16>>();

// let distance_to_closest_relevant_node = nodes
//     .iter()
//     .map(|i| {
//         relevant_nodes
//             .iter()
//             .filter(|r| i.0 != **r)
//             .map(|r| distmat[(i.0 * n_nodes + *r) as usize])
//             .min()
//             .unwrap()
//     })
//     .collect::<Vec<i16>>();

// let mut best_score = 0;
// // This is a tuple of priority, score, timer, current, opened
// let mut best_tup: (i16, i16, i16, i16, Vec<i16>) = (0, 0, 0, 0, vec![]);

// let mut queue = BinaryHeap::new();
// let start_index = nodes.iter().position(|f| f.1 == "AA").unwrap() as i16;
// let init: (i16, i16, i16, i16, Vec<i16>) = (0, 0, 30, start_index, vec![]);
// queue.push(init);
// while !queue.is_empty() {
//     let (priority, score, timer, current, opened) = queue.pop().unwrap();
//     if priority < 0 {
//         break;
//     }
//     if score > best_score {
//         best_score = score;
//         best_tup = (priority, score, timer, current, opened.clone());
//     }

//     let to_visit = relevant_nodes.iter().filter(|k| !opened.contains(k)).map(|k| *k).collect::<Vec<i16>>();
//     for next in &to_visit {
//         let distance_to_next = distmat[(current * n_nodes + next) as usize];
//         let new_timer = timer - (distance_to_next + 1);
//         let new_score = score + new_timer * flows[*next as usize];
//         let new_priority = new_timer - distance_to_closest_relevant_node[*next as usize] - 1;
//         let new_current = next;
//         let mut new_opened = opened.clone();
//         new_opened.push(next.clone());
//         if new_timer > 0 {
//             queue.push((new_priority, new_score, new_timer, *new_current, new_opened));
//         }
//     }
// }
// let best_route = best_tup.4;
// println!("{best_route:?}: {}", best_tup.1);
// }

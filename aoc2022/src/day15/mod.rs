use std::{
    cmp::{max, min},
    collections::HashSet,
    num::ParseIntError,
    ops::BitOr,
};

use regex::Regex;

type Range = (i64, i64);
type Pos = (i64, i64);
pub fn compute_solution_1(input: String) {
    let (sensors, beacons) = get_sensors_and_beacons(input);
    let y_coord = 2_000_000;
    let ranges = get_unavailable_ranges_at(&sensors, &beacons, y_coord);
    let unavailable_sum: i64 = ranges.iter().map(|r| r.1 - r.0).sum();
    println!("{}", unavailable_sum);
}

pub fn compute_solution_2(input: String) {
    let (sensors, beacons) = get_sensors_and_beacons(input);
    for y_coord in 0..=4_000_000 {
        let ranges = get_unavailable_ranges_at(&sensors, &beacons, y_coord);
        let mut trunc_ranges = truncate(ranges, 0, 4_000_000);
        let unavailable_sum: i64 = trunc_ranges.iter().map(|r| (r.1 - r.0).abs()).sum();
        if unavailable_sum < 4_000_000 {
            trunc_ranges.sort();
            let x_coord = trunc_ranges[0].1 + 1;
            let tuning_freq = x_coord * 4_000_000 + y_coord;
            println!("{tuning_freq}");
        }
    }
}

fn truncate(ranges: Vec<Range>, from: i64, to: i64) -> Vec<Pos> {
    ranges.iter().map(|r| (max(r.0, from), min(r.1, to))).filter(|r| r.0 != r.1).collect::<Vec<Range>>()
}

fn get_unavailable_ranges_at(sensors: &Vec<Pos>, beacons: &Vec<Pos>, y_coord: i64) -> Vec<(i64, i64)> {
    let mut ranges: Vec<Range> = Vec::new();

    for (s, b) in sensors.iter().zip(beacons.iter()) {
        let sb_dist = (s.0 - b.0).abs() + (s.1 - b.1).abs();
        let sy_dist = (s.1 - y_coord).abs();
        let diff = sb_dist - sy_dist;
        if diff > 0 {
            ranges.push((-diff + s.0, diff + s.0))
        }
    }

    let mut unions: Vec<Range> = Vec::new();

    while !ranges.is_empty() {
        let mut joined;
        let mut acc = ranges.pop().unwrap();
        if !ranges.is_empty() {
            'mergin: loop {
                joined = false;
                for (i, r) in ranges.iter().enumerate() {
                    if overlaps(*r, acc) {
                        acc = merge(*r, acc);
                        joined = true;
                        ranges.remove(i);
                        break;
                    }
                }
                if !joined {
                    break 'mergin;
                }
            }
        }
        unions.push(acc);
    }
    unions
}

fn get_sensors_and_beacons(input: String) -> (Vec<Pos>, Vec<Pos>) {
    let pattern = Regex::new(r"(-?\d*), y=(-?\d*):.*x=(-?\d*), y=(-?\d*)").unwrap();
    let mut sensors: Vec<Range> = Vec::new();
    let mut beacons: Vec<Range> = Vec::new();

    for line in input.lines() {
        let captures = pattern.captures(line).unwrap();
        let mut it = captures.iter();
        it.next();
        let stuff = it.map(|v| v.unwrap().as_str().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        sensors.push((stuff[0], stuff[1]));
        beacons.push((stuff[2], stuff[3]));
    }
    (sensors, beacons)
}

fn overlaps(one: Range, two: Range) -> bool {
    (one.1 - one.0) + (two.1 - two.0) >= max(one.1, two.1) - min(one.0, two.0)
}

fn merge(one: Range, two: Range) -> Range {
    (min(one.0, two.0), max(one.1, two.1))
}

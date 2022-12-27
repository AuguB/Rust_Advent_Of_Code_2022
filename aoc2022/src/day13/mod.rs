use std::{
    cmp::{max, Ordering},
    iter::Peekable,
    ops::Index,
    str::Chars,
    string,
};

use ndarray::Order;

pub fn compute_solution_1(input: String) {
    let mut sum = 0;
    for (i, v) in input.split("\n\n").enumerate() {
        let p1p2 = v.split("\n").collect::<Vec<&str>>();

        let p1 = Packet::new(p1p2[0].to_string());
        let p2 = Packet::new(p1p2[1].to_string());
        if let Some(true) = p1.is_smaller_than(p2) {
            sum += 1 + i;
        }
    }
    println!("{sum}")
}

pub fn compute_solution_2(input: String) {
    let mut list: Vec<Packet> = Vec::new();
    for i in input.lines() {
        if i.len() >= 1 {
            list.push(Packet::new(String::from(i)));
        }
    }
    list.push(Packet::new(String::from("[[2]]")));
    list.push(Packet::new(String::from("[[6]]")));
    list.sort();
    let i1 = list.binary_search(&Packet::new(String::from("[[2]]"))).unwrap() + 1;
    let i2 = list.binary_search(&Packet::new(String::from("[[6]]"))).unwrap() + 1;
    println!("{}", i1 * i2);

    list.iter().for_each(|f| println!("{}", f.to_string()))
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct Packet {
    list: NestedList,
}
impl Packet {
    fn new(str: String) -> Self {
        let mut iter = str.chars().peekable();
        Packet { list: NestedList::new(&mut iter).unwrap() }
    }
    fn to_string(&self) -> String {
        self.list.to_string()
    }
    fn is_smaller_than(&self, other: Packet) -> Option<bool> {
        self.list.is_smaller_than(&other.list)
    }
}

enum NestedList {
    List(Box<Vec<NestedList>>),
    Number(u32),
}
impl PartialEq for NestedList {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            _ => false,
        }
    }
}
impl Eq for NestedList {}
impl PartialOrd for NestedList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(result) = self.is_smaller_than(other) {
            return match result {
                true => Some(Ordering::Less),
                _ => Some(Ordering::Greater),
            };
        }
        Some(Ordering::Equal)
    }
}
impl Ord for NestedList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(result) = self.is_smaller_than(other) {
            return match result {
                true => Ordering::Less,
                _ => Ordering::Greater,
            };
        }
        Ordering::Equal
    }
}
impl Clone for NestedList {
    fn clone(&self) -> Self {
        match self {
            Self::List(arg0) => Self::List(arg0.clone()),
            Self::Number(arg0) => Self::Number(arg0.clone()),
        }
    }
}

impl NestedList {
    fn new(iter: &mut Peekable<Chars>) -> Option<Self> {
        match iter.peek().unwrap() {
            '[' => {
                iter.next();
                let mut my_vec: Vec<Self> = Vec::new();
                while iter.peek().unwrap().clone() != ']' {
                    if let Some(list) = Self::new(iter) {
                        my_vec.push(list);
                    }
                }
                iter.next();
                return Some(Self::List(Box::new(my_vec)));
            }
            ',' => {
                iter.next();
                None
            }
            num => {
                let mut my_string = String::from(*num);
                loop {
                    iter.next();
                    match iter.peek().unwrap().clone() {
                        '[' | ']' | ',' => return Some(Self::Number(my_string.parse::<u32>().unwrap())),
                        a => my_string += &String::from(a),
                    }
                }
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            NestedList::List(l) => {
                let mut acc = String::from('[');
                for s in l.iter() {
                    acc += &s.to_string();
                    acc += &String::from(',');
                }
                if l.len() > 1 {
                    acc.pop();
                }
                acc + &String::from(']')
            }
            NestedList::Number(a) => a.to_string(),
        }
    }

    fn is_smaller_than(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Self::Number(ll), Self::Number(rr)) if ll == rr => None,
            (Self::Number(ll), Self::Number(rr)) => Some(ll < rr),
            (Self::List(ll), Self::List(rr)) => {
                let max_length = max(ll.len() as usize, rr.len() as usize);
                for ind in 0..max_length {
                    if let Some(left_item) = ll.get(ind) {
                        if let Some(right_item) = rr.get(ind) {
                            if let Some(outcome) = left_item.is_smaller_than(right_item) {
                                return Some(outcome);
                            }
                        } else {
                            return Some(false);
                        }
                    } else {
                        return Some(true);
                    }
                }
                None
            }
            (Self::List(ll), Self::Number(rr)) => Self::List(ll.clone()).is_smaller_than(&Self::List(Box::new(vec![Self::Number(*rr)]))),
            (Self::Number(ll), Self::List(rr)) => Self::List(Box::new(vec![Self::Number(*ll)])).is_smaller_than(&Self::List(rr.clone())),
        }
    }
}

#[test]
fn test_nested_list_constructor() {
    let str = "[[4,4],4,4]";
    assert_eq!(str, Packet::new(String::from(str)).to_string());

    let str = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
    assert_eq!(str, Packet::new(String::from(str)).to_string());

    let str = "[]";
    assert_eq!(str, Packet::new(String::from(str)).to_string());
}

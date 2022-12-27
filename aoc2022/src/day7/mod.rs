//cargo watch -x test
use ndarray::ArrayD;
use regex::Regex;
use std::collections::HashMap;

pub fn compute_solution_1(_input: String) {
    let commands = Regex::new(r"\$ (cd (\.\.|/|\w*){1}|ls)|(\d+)\s([\w\.]+)|dir (\w*)").unwrap();
    let mut _matches = commands.captures_iter(&_input).peekable();

    // Create a hashmap with mutable entries to store directory and sizes
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    dir_sizes.insert(String::from("/"), 0);

    // We need a Vec to store the names of the parent folders
    let mut parents: Vec<String> = Vec::new();

    // Loop over the lines
    'outer: loop {
        let next_match = _matches.next().unwrap();
        match next_match.get(1) {
            Some(command) => {
                // If we got a command:
                match command.as_str() {
                    "ls" => 'ls: loop {
                        if let Some(peek) = _matches.peek() {
                            if let Some(_) = peek.get(1) {
                                break 'ls;
                            }
                        } else {
                            break 'outer;
                        }
                        if let Some(next_match) = _matches.next() {
                            if let Some(filesize) = next_match.get(3) {
                                let filesize = filesize.as_str().parse::<u32>().unwrap();
                                for parent in parents.iter() {
                                    dir_sizes.entry(parent.to_string()).and_modify(|size| *size += filesize).or_insert(filesize);
                                }
                            } else if let Some(dirname) = next_match.get(5) {
                                let dir_path: String = get_path(&parents, Some(dirname.as_str()));
                                dir_sizes.insert(dir_path, 0);
                            }
                        }
                    },
                    _ => match next_match.get(2).unwrap().as_str() {
                        ".." => {
                            parents.pop();
                        }
                        "/" => {
                            parents = Vec::new();
                            parents.push(String::from("/"));
                        }
                        a => {
                            let dir_path: String = get_path(&parents, Some(&a.to_string()));
                            parents.push(dir_path);
                        }
                    },
                }
            }
            _ => {
                println!("Unhandled case");
            }
        }
    }
    let mut sum_of_small_folders: u32 = 0;
    let size_of_small_folder: u32 = 100000;
    let root = String::from("/");
    let mut size_of_smallest_dir_that_is_large_enough: &u32 = dir_sizes.get(&root).unwrap();
    let total_space = 70000000;
    let target_space = 30000000;

    let available_space = total_space - size_of_smallest_dir_that_is_large_enough;
    let space_to_be_freed = target_space - available_space;

    for (key, value) in dir_sizes.iter() {
        if value <= &size_of_small_folder {
            sum_of_small_folders += value;
        }
        if value >= &space_to_be_freed {
            if value < size_of_smallest_dir_that_is_large_enough {
                size_of_smallest_dir_that_is_large_enough = value;
            }
        }
    }
    println!("sum_of_small_folders {}", sum_of_small_folders);

    println!("size_of_smallest_dir_that_is_large_enough {}", size_of_smallest_dir_that_is_large_enough)
}

fn get_path(parents: &Vec<String>, child: Option<&str>) -> String {
    let mut path: String = String::new();
    for i in 0..parents.len() {
        path += "/";
        path += parents.get(i).unwrap();
    }
    if let Some(childname) = child {
        path += "/";
        path += childname;
    }
    path
}

pub fn compute_solution_2(_input: String) {}

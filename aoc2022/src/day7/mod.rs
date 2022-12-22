use regex::Regex;

pub fn compute_solution_1(input: String) {
    let mut mytree: Tree;
    let previousdepth = 0;
    let mut parents: Vec<&Tree> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let (mut node, depth) = Tree::make_node(line);
        if i == 0 {
            mytree = node;
            parents.push(&mytree);
        } else if depth > previousdepth {
            let lastNode = parents.last().unwrap();
            // lastNode.append(node);
            // parents.push(&node);
        } else {
        }
    }
}

pub fn compute_solution_2(input: String) {}

enum Tree {
    Branch(String, Vec<Tree>),
    Leaf(u32),
}

impl Tree {
    fn make_node(line: &str) -> (Self, usize) {
        let regex_pattern = Regex::new(r"^(\s*)- (\S*) \((dir|file, size=(\d*))\)$").unwrap();
        let captures = regex_pattern.captures(line).unwrap();
        let depth = (captures[0].len() + 1) / 2 as usize;

        if captures[3].len() == 0 {
            return (Tree::Branch(captures[1].to_string(), Vec::new()), depth);
        } else {
            let size = captures[3].parse().unwrap();
            return (Tree::Leaf(size), depth);
        }
    }

    fn append(&mut self, newchild: Tree) {
        match self {
            Tree::Branch(_, mut children) => {
                children.push(newchild);
            }
            Tree::Leaf(_) => println!("Can not append to a leaf"),
        }
    }

    fn total_sum(&self) -> u32 {
        match self {
            Tree::Branch(_, children) => children.iter().map(|c| c.total_sum()).sum(),
            Tree::Leaf(a) => *a,
        }
    }

    fn get_all_names(&self, ref mut acc: &mut Vec<String>) {
        match self {
            Tree::Branch(name, children) => {
                acc.push(name.clone());
                children.iter().for_each(|c| c.get_all_names(acc))
            }
            Tree::Leaf(_) => (),
        }
    }
}

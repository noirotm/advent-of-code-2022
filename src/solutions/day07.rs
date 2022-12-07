use crate::solver::Solver;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = FileSystem;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        FileSystem::from_reader(r)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        (0..input.nodes.len())
            .filter(|&n| input.is_dir(n))
            .map(|n| input.node_size(n))
            .filter(|&s| s <= 100000)
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let used_space = input.node_size(0);
        let unused_space = 70000000 - used_space;
        let min_needed_cleanup = 30000000 - unused_space;

        (0..input.nodes.len())
            .filter(|&n| input.is_dir(n))
            .map(|n| input.node_size(n))
            .filter(|&s| s >= min_needed_cleanup)
            .min()
            .unwrap_or_default()
    }
}

#[derive(Debug)]
pub enum Node {
    Dir,
    File(u64),
}

#[derive(Debug)]
pub struct FileSystem {
    nodes: Vec<Node>,
    children: HashMap<usize, Vec<usize>>,
}

impl FileSystem {
    fn from_reader<R: Read>(r: R) -> FileSystem {
        let mut fs = Self {
            nodes: vec![],
            children: Default::default(),
        };
        let mut current_node_stack = vec![];

        let reader = BufReader::new(r);
        for line in reader.lines().flatten() {
            // enter directory, add it to the node list, push exploration stack
            // or pop the stack if we go up
            if let Ok(dir) = scan_fmt!(&line, "$ cd {}", String) {
                match dir.as_str() {
                    ".." => {
                        current_node_stack.pop();
                    }
                    _ => {
                        fs.nodes.push(Node::Dir);
                        let idx = fs.nodes.len() - 1;

                        if let Some(&parent_idx) = current_node_stack.last() {
                            let e = fs.children.entry(parent_idx).or_default();
                            e.push(idx);
                        }

                        current_node_stack.push(idx);
                    }
                }
            }
            // we find a file in the current directory, add it to node list
            if let Ok((size, _)) = scan_fmt!(&line, "{} {}", u64, String) {
                fs.nodes.push(Node::File(size));
                let idx = fs.nodes.len() - 1;

                if let Some(&parent_idx) = current_node_stack.last() {
                    let e = fs.children.entry(parent_idx).or_default();
                    e.push(idx);
                }
            }
        }

        fs
    }

    fn is_dir(&self, idx: usize) -> bool {
        self.nodes
            .get(idx)
            .map(|n| matches!(n, Node::Dir))
            .unwrap_or(false)
    }

    fn node_size(&self, idx: usize) -> u64 {
        if let Some(Node::File(size)) = self.nodes.get(idx) {
            *size
        } else if let Some(children) = self.children.get(&idx) {
            children.iter().map(|&i| self.node_size(i)).sum()
        } else {
            0
        }
    }
}

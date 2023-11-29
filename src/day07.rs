use regex::Regex;
use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let command_parse = Regex::new(r"\$ (\w+) ?([\w/\.]+)?").unwrap();
    let dir_parse = Regex::new(r"dir (\w+)").unwrap();
    let file_parse = Regex::new(r"(\d+) ((?:\w|\.)+)").unwrap();

    let mut file_tree = FileTree::default();
    let root_idx = file_tree.node("/", 0);

    let mut cur_idx = root_idx;
    for l in input.lines().skip(1) {
        if let Some(cap) = command_parse.captures(l) {
            match cap.get(1).unwrap().as_str() {
                "cd" => {
                    let dst_name = cap.get(2).unwrap().as_str();
                    let dst_idx = match dst_name {
                        ".." => file_tree.arena[cur_idx].parent.unwrap(),
                        dst_name => *file_tree.arena[cur_idx]
                            .children
                            .iter()
                            .find(|&&n| file_tree.arena[n].name == dst_name)
                            .unwrap(),
                    };

                    cur_idx = dst_idx;
                }
                "ls" => {}

                _ => panic!("Unknown command"),
            }
        } else if let Some(cap) = dir_parse.captures(l) {
            let new_name = cap.get(1).unwrap().as_str();
            let new_idx = file_tree.node(new_name, 0);
            file_tree.arena[cur_idx].children.push(new_idx);
            file_tree.arena[new_idx].parent = Some(cur_idx);
        } else if let Some(cap) = file_parse.captures(l) {
            let new_size = cap.get(1).unwrap().as_str().parse().unwrap();
            let new_name = cap.get(2).unwrap().as_str();
            let new_idx = file_tree.node(new_name, new_size);
            file_tree.arena[cur_idx].children.push(new_idx);
            file_tree.arena[new_idx].parent = Some(cur_idx);
        }
    }

    let mut total = 0;
    for n in &file_tree.arena {
        if n.size == 0 {
            let size = file_tree.dir_size(n.idx);
            if size <= 100000 {
                total += size;
            }
        }
    }

    total
}

fn part2(input: &str) -> impl Display {
    let command_parse = Regex::new(r"\$ (\w+) ?([\w/\.]+)?").unwrap();
    let dir_parse = Regex::new(r"dir (\w+)").unwrap();
    let file_parse = Regex::new(r"(\d+) ((?:\w|\.)+)").unwrap();

    let mut file_tree = FileTree::default();
    let root_idx = file_tree.node("/", 0);

    let mut cur_idx = root_idx;
    for l in input.lines().skip(1) {
        if let Some(cap) = command_parse.captures(l) {
            match cap.get(1).unwrap().as_str() {
                "cd" => {
                    let dst_name = cap.get(2).unwrap().as_str();
                    let dst_idx = match dst_name {
                        ".." => file_tree.arena[cur_idx].parent.unwrap(),
                        dst_name => *file_tree.arena[cur_idx]
                            .children
                            .iter()
                            .find(|&&n| file_tree.arena[n].name == dst_name)
                            .unwrap(),
                    };

                    cur_idx = dst_idx;
                }
                "ls" => {}
                _ => panic!("Unknown command"),
            }
        } else if let Some(cap) = dir_parse.captures(l) {
            let new_name = cap.get(1).unwrap().as_str();
            let new_idx = file_tree.node(new_name, 0);
            file_tree.arena[cur_idx].children.push(new_idx);
            file_tree.arena[new_idx].parent = Some(cur_idx);
        } else if let Some(cap) = file_parse.captures(l) {
            let new_size = cap.get(1).unwrap().as_str().parse().unwrap();
            let new_name = cap.get(2).unwrap().as_str();
            let new_idx = file_tree.node(new_name, new_size);
            file_tree.arena[cur_idx].children.push(new_idx);
            file_tree.arena[new_idx].parent = Some(cur_idx);
        }
    }

    let total_used = file_tree.dir_size(0);
    let rm_target = 30000000 - (70000000 - total_used);
    let mut best_rm = 70000000;
    for n in &file_tree.arena {
        if n.size == 0 {
            let size = file_tree.dir_size(n.idx);
            if size >= rm_target && size < best_rm {
                best_rm = size;
            }
        }
    }

    best_rm
}

#[derive(Debug, Default)]
struct FileTree {
    arena: Vec<FileNode>,
}

impl FileTree {
    fn node(&mut self, name: &str, size: usize) -> usize {
        let idx = self.arena.len();
        self.arena.push(FileNode::new(idx, name, size));
        idx
    }

    fn dir_size(&self, idx: usize) -> usize {
        let mut total = 0;
        for c in &self.arena[idx].children {
            if self.arena[*c].size == 0 {
                total += self.dir_size(*c);
            } else {
                total += self.arena[*c].size
            }
        }
        total
    }
}

#[derive(Debug)]
struct FileNode {
    idx: usize,
    name: String,
    size: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl FileNode {
    fn new(idx: usize, name: &str, size: usize) -> Self {
        Self {
            idx,
            name: name.to_string(),
            size,
            parent: None,
            children: Vec::new(),
        }
    }
}

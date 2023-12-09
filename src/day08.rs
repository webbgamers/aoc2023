use core::panic;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let mut input = input.lines();
    let instructions = input.next().unwrap().chars().collect::<Vec<_>>();

    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    let mut nodes = HashMap::new();

    for l in input.skip(1) {
        println!("{l}");
        let (_, [node, left, right]) = re.captures_iter(l).next().unwrap().extract();
        nodes.insert(node, (left, right));
    }

    let mut i = 0;
    let mut cur_node = "AAA";
    loop {
        if cur_node == "ZZZ" {
            return i;
        }

        let &node = nodes.get(cur_node).unwrap();
        match instructions[i % instructions.len()] {
            'L' => cur_node = node.0,
            'R' => cur_node = node.1,
            _ => panic!("Invalid instruction"),
        }

        i += 1;
    }
}

fn part2(input: &str) -> impl Display {
    let mut input = input.lines();
    let instructions = input.next().unwrap().chars().collect::<Vec<_>>();

    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    let mut nodes = HashMap::new();

    for l in input.skip(1) {
        let (_, [node, left, right]) = re.captures_iter(l).next().unwrap().extract();
        nodes.insert(node, (left, right));
    }

    let start_nodes = nodes
        .keys()
        .filter(|n| n.ends_with('A'))
        .copied()
        .collect::<Vec<_>>();
    let mut iterations = Vec::new();

    for mut cur_node in start_nodes {
        let mut i = 0;
        loop {
            if cur_node.ends_with('Z') {
                println!("Found result {cur_node} in {i}");
                iterations.push(i);
                break;
            }

            let &node = nodes.get(cur_node).unwrap();
            match instructions[i % instructions.len()] {
                'L' => cur_node = node.0,
                'R' => cur_node = node.1,
                _ => panic!("Invalid instruction"),
            }

            i += 1;
        }
    }

    while iterations.len() > 1 {
        let chunks = iterations.chunks_exact(2);
        let remainder = chunks.remainder();
        let mut new_iterations = Vec::new();
        for chunk in chunks {
            if let &[a, b] = chunk {
                new_iterations.push(lcm(a, b));
                if let &[r] = remainder {
                    new_iterations.push(r);
                }
            }
        }
        iterations = new_iterations;
    }

    iterations[0]
}

fn lcm(n1: usize, n2: usize) -> usize {
    let mut x;
    let mut y;

    if n1 > n2 {
        x = n1;
        y = n2;
    } else {
        x = n2;
        y = n1;
    }

    let mut rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }

    n1 * n2 / y
}

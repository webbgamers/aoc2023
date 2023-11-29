use regex::Regex;
use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let crate_input = input
        .lines()
        .take(8)
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut crates = Vec::new();
    for x in 0..9 {
        let mut column = Vec::new();
        for row in crate_input.iter().take(8) {
            column.push(row[x * 4 + 1]);
        }
        column.reverse();
        column = column
            .iter()
            .filter(|&c| !c.is_whitespace())
            .copied()
            .collect::<Vec<_>>();
        crates.push(column);
    }

    // and this is when i remembered regex was a thing

    let command_input = input.lines().skip(10);
    for l in command_input {
        let re = Regex::new("move (\\d+) from (\\d) to (\\d)").unwrap();
        let cap = re.captures_iter(l).next().unwrap();

        let amount = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let start = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let end = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();

        for _ in 0..amount {
            let val = crates[start - 1].pop().unwrap();
            crates[end - 1].push(val);
        }
    }

    let mut result = Vec::new();
    for mut c in crates {
        result.push(c.pop().unwrap());
    }

    result.iter().collect::<String>()
}

fn part2(input: &str) -> impl Display {
    let crate_input = input
        .lines()
        .take(8)
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut crates = Vec::new();
    for x in 0..9 {
        let mut column = Vec::new();
        for row in crate_input.iter().take(8) {
            column.push(row[x * 4 + 1]);
        }
        column.reverse();
        column = column
            .iter()
            .filter(|&c| !c.is_whitespace())
            .copied()
            .collect::<Vec<_>>();
        crates.push(column);
    }

    let command_input = input.lines().skip(10);
    for l in command_input {
        let re = Regex::new("move (\\d+) from (\\d) to (\\d)").unwrap();
        let cap = re.captures_iter(l).next().unwrap();

        let amount = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let start = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let end = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();

        let mut tmp = Vec::new();
        for _ in 0..amount {
            tmp.push(crates[start - 1].pop().unwrap());
        }
        tmp.reverse();
        crates[end - 1].append(&mut tmp);
    }

    let mut result = Vec::new();
    for mut c in crates {
        result.push(c.pop().unwrap());
    }

    result.iter().collect::<String>()
}

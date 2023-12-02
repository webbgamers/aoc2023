use std::fmt::Display;

use regex::Regex;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let input = input.lines();
    let mut sum = 0;
    for l in input {
        let mut first = 0;
        for c in l.chars() {
            if first == 0 {
                first = c.to_string().parse::<i32>().unwrap_or(0);
            }
        }
        let mut last = 0;
        for c in l.chars().rev() {
            if last == 0 {
                last = c.to_string().parse::<i32>().unwrap_or(0);
            }
        }
        let value = first * 10 + last;
        sum += value;
    }
    sum
}

fn part2(input: &str) -> impl Display {
    let mut sum = 0;
    let re1 = Regex::new(r"((?:zero|one|two|three|four|five|six|seven|eight|nine)|\d).*").unwrap();
    let re2 = Regex::new(r".*((?:zero|one|two|three|four|five|six|seven|eight|nine)|\d)").unwrap();
    let input = input.lines().filter(|&l| !l.is_empty());
    for l in input {
        let m = re1.captures(l).unwrap().get(1).unwrap().as_str();
        let first = match m.parse::<i32>() {
            Ok(n) => n,
            Err(_) => match m {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => 0,
            },
        };
        let m = re2.captures(l).unwrap().get(1).unwrap().as_str();
        let last = match m.parse::<i32>() {
            Ok(n) => n,
            Err(_) => match m {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => 0,
            },
        };
        let value = first * 10 + last;
        sum += value;
    }
    sum
}

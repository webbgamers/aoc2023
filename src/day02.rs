use std::fmt::Display;

use regex::Regex;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let games = input.lines();
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut sum = 0;

    let re1 = Regex::new(r"Game (\d+):(.+)").unwrap();
    let re2 = Regex::new(r"(\d+) (blue|red|green)").unwrap();

    for g in games {
        let mut cap1 = re1.captures_iter(g);
        let (_, [id, games]) = cap1.next().unwrap().extract();
        let id = id.parse::<u32>().unwrap();
        let rounds = games.split("; ");
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for r in rounds {
            let cubes = r.split(", ");
            for c in cubes {
                let (_, [amount, color]) = re2.captures_iter(c).next().unwrap().extract();
                let amount = amount.parse::<u32>().unwrap();
                match color {
                    "red" => {
                        if amount > red {
                            red = amount
                        }
                    }
                    "green" => {
                        if amount > green {
                            green = amount
                        }
                    }
                    "blue" => {
                        if amount > blue {
                            blue = amount
                        }
                    }
                    _ => {}
                }
            }
        }
        if red <= max_red && green <= max_green && blue <= max_blue {
            sum += id;
        }
    }
    sum
}

fn part2(input: &str) -> impl Display {
    let games = input.lines();

    let mut sum = 0;

    let re1 = Regex::new(r"Game (\d+):(.+)").unwrap();
    let re2 = Regex::new(r"(\d+) (blue|red|green)").unwrap();

    for g in games {
        let mut cap1 = re1.captures_iter(g);
        let (_, [_, games]) = cap1.next().unwrap().extract();
        let rounds = games.split("; ");
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for r in rounds {
            let cubes = r.split(", ");
            for c in cubes {
                let (_, [amount, color]) = re2.captures_iter(c).next().unwrap().extract();
                let amount = amount.parse::<u32>().unwrap();
                match color {
                    "red" => {
                        if amount > red {
                            red = amount
                        }
                    }
                    "green" => {
                        if amount > green {
                            green = amount
                        }
                    }
                    "blue" => {
                        if amount > blue {
                            blue = amount
                        }
                    }
                    _ => {}
                }
            }
        }
        sum += red * green * blue;
    }
    sum
}

use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let re = Regex::new(r"Card +(\d+): +([0-9 ]+) \| +([0-9 ]+)$").unwrap();

    let mut sum = 0;

    for c in input.lines() {
        let m = re.captures(c).unwrap();
        let _card = m.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let winners = m
            .get(2)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let numbers = m
            .get(3)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut wins = 0;
        for n in numbers {
            if winners.contains(&n) {
                wins += 1;
            }
        }

        if wins > 0 {
            sum += 2_u32.pow(wins - 1)
        }
    }

    sum
}

fn part2(input: &str) -> impl Display {
    let re = Regex::new(r"Card +(\d+): +([0-9 ]+) \| +([0-9 ]+)$").unwrap();

    let mut cards = HashMap::new();
    let mut game = Vec::new();

    for c in input.lines() {
        let m = re.captures(c).unwrap();
        let card_num = m.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let winners = m
            .get(2)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let numbers = m
            .get(3)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut wins = 0;
        for n in numbers {
            if winners.contains(&n) {
                wins += 1;
            }
        }

        cards.insert(card_num, wins);
        game.push(card_num);
    }

    let mut i = 0;
    while i < game.len() {
        let card_num = game[i];
        let wins = *cards.get(&card_num).unwrap();

        if wins > 0 {
            for n in 1..=wins {
                game.push(card_num + n);
            }
        }

        i += 1;
    }

    game.len()
}

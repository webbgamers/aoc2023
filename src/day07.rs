use core::panic;
use std::collections::HashMap;
use std::iter::zip;
use std::{cmp::Ordering, fmt::Display};

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let mut hands = input
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            (
                l.next().unwrap(),
                l.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let charmap = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    hands.sort_unstable_by(|&a, &b| {
        let a_type = find_type_p1(a.0);
        let b_type = find_type_p1(b.0);
        match a_type.cmp(&b_type) {
            Ordering::Equal => {
                let mut order = Ordering::Equal;
                for (a_char, b_char) in zip(a.0.chars(), b.0.chars()) {
                    let a_val = charmap.iter().position(|&c| c == a_char).unwrap();
                    let b_val = charmap.iter().position(|&c| c == b_char).unwrap();

                    order = b_val.cmp(&a_val);
                    if order != Ordering::Equal {
                        break;
                    }
                }
                order
            },
            order => order,
        }
    });

    let mut winnings = 0;

    for (i, &(_, bet)) in hands.iter().enumerate() {
        winnings += (i + 1) * bet;
    }

    winnings
}

fn part2(input: &str) -> impl Display {
    let mut hands = input
        .lines()
        .map(|l| {
            let mut l = l.split_whitespace();
            (
                l.next().unwrap(),
                l.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let charmap = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    hands.sort_unstable_by(|&a, &b| {
        let a_type = find_type_p2(a.0);
        let b_type = find_type_p2(b.0);
        match a_type.cmp(&b_type) {
            Ordering::Equal => {
                let mut order = Ordering::Equal;
                for (a_char, b_char) in zip(a.0.chars(), b.0.chars()) {
                    let a_val = charmap.iter().position(|&c| c == a_char).unwrap();
                    let b_val = charmap.iter().position(|&c| c == b_char).unwrap();

                    order = b_val.cmp(&a_val);
                    if order != Ordering::Equal {
                        break;
                    }
                }
                order
            },
            order => order,
        }
    });

    let mut winnings = 0;

    for (i, &(_, bet)) in hands.iter().enumerate() {
        winnings += (i + 1) * bet;
    }

    winnings
}

fn find_type_p1(hand: &str) -> u8 {
    let mut chars = HashMap::new();
    for c in hand.chars() {
        chars
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    if *chars.iter().next().unwrap().1 == 5 {
        return 6;
    } else if chars.iter().any(|(_, &n)| n == 4) {
        return 5;
    } else if chars.iter().any(|(_, &n)| n == 3) && chars.iter().any(|(_, &n)| n == 2) {
        return 4;
    } else if chars.iter().any(|(_, &n)| n == 3) {
        return 3;
    } else if chars.iter().filter(|(_, &n)| n == 2).count() == 2 {
        return 2;
    } else if chars.iter().filter(|(_, &n)| n == 2).count() == 1 {
        return 1;
    } else if chars.iter().all(|(_, &n)| n == 1) {
        return 0;
    }

    panic!("Unhandled case {hand}")
}

fn find_type_p2(hand: &str) -> u8 {
    let mut chars = HashMap::new();
    for c in hand.chars() {
        chars
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let jokers = chars.remove(&'J').unwrap_or(0);
    if jokers == 5 {
        return 6;
    }

    if chars.iter().any(|(_, &n)| n == 5) {
        return 6;
    } else if chars.iter().any(|(_, &n)| n == 4) {
        return match jokers {
            0 => 5,
            1 => 6,
            _ => panic!("Too many jokers {hand}"),
        };
    } else if chars.iter().any(|(_, &n)| n == 3) && chars.iter().any(|(_, &n)| n == 2) {
        return 4;
    } else if chars.iter().any(|(_, &n)| n == 3) {
        return match jokers {
            0 => 3,
            1 => 5,
            2 => 6,
            _ => panic!("Too many jokers {hand}"),
        };
    } else if chars.iter().filter(|(_, &n)| n == 2).count() == 2 {
        return match jokers {
            0 => 2,
            1 => 4,
            _ => panic!("Too many jokers {hand}"),
        };
    } else if chars.iter().filter(|(_, &n)| n == 2).count() == 1 {
        return match jokers {
            0 => 1,
            1 => 3,
            2 => 5,
            3 => 6,
            _ => panic!("Too many jokers {hand}"),
        };
    } else if chars.iter().all(|(_, &n)| n == 1) {
        return match jokers {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 5,
            4 => 6,
            _ => panic!("Too many jokers {hand}"),
        };
    }

    panic!("Unhandled case {hand}")
}

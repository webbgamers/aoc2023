use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    // Organize input
    let input = input.lines().map(|l| {
        let mut l = l.split_whitespace();
        (l.next().expect("bad"), l.next().expect("bad"))
    });

    // Calculate score
    let mut score = 0;
    for i in input {
        score += match i.0 {
            "A" => match i.1 {
                "X" => 4,
                "Y" => 8,
                "Z" => 3,
                _ => panic!("bad"),
            },
            "B" => match i.1 {
                "X" => 1,
                "Y" => 5,
                "Z" => 9,
                _ => panic!("bad"),
            },
            "C" => match i.1 {
                "X" => 7,
                "Y" => 2,
                "Z" => 6,
                _ => panic!("bad"),
            },
            _ => panic!("bad"),
        }
    }

    score
}

fn part2(input: &str) -> impl Display {
    // Organize input
    let input = input.lines().map(|l| {
        let mut l = l.split_whitespace();
        (l.next().expect("bad"), l.next().expect("bad"))
    });

    // Calculate score
    let mut score = 0;
    for i in input {
        score += match i.0 {
            "A" => match i.1 {
                "X" => 3,
                "Y" => 4,
                "Z" => 8,
                _ => panic!("bad"),
            },
            "B" => match i.1 {
                "X" => 1,
                "Y" => 5,
                "Z" => 9,
                _ => panic!("bad"),
            },
            "C" => match i.1 {
                "X" => 2,
                "Y" => 6,
                "Z" => 7,
                _ => panic!("bad"),
            },
            _ => panic!("bad"),
        }
    }

    score
}

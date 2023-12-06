use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let mut input = input.lines().map(|l| {
        l.split_whitespace()
            .skip(1)
            .map(|n| n.parse::<u32>().unwrap())
    });

    let times = input.next().unwrap().collect::<Vec<_>>();
    let distances = input.next().unwrap().collect::<Vec<_>>();

    let mut prod = 1;

    for i in 0..times.len() {
        let mut wins = 0;
        let time = times[i];
        let dist = distances[i];
        for t in 0..time {
            if (time - t) * (t) > dist {
                wins += 1;
            }
        }

        prod *= wins;
    }

    prod
}

fn part2(input: &str) -> impl Display {
    let mut input = input.lines().map(|l| {
        l.split_whitespace()
            .skip(1)
            .flat_map(|s| s.chars())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    });
	
    let time = input.next().unwrap();
    let dist = input.next().unwrap();

    let mut wins = 0;

    for t in 0..time {
        if (time - t) * (t) > dist {
            wins += 1;
        }
    }

    wins
}

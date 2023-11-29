use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let input = input.lines().map(|l| {
        l.split(',')
            .map(|r| {
                r.split('-')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    });

    let mut count = 0;
    for p in input {
        if (p[0][0] <= p[1][0] && p[0][1] >= p[1][1]) || (p[0][0] >= p[1][0] && p[0][1] <= p[1][1])
        {
            count += 1;
        }
    }

    count
}

fn part2(input: &str) -> impl Display {
    let input = input.lines().map(|l| {
        l.split(',')
            .map(|r| {
                r.split('-')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    });

    let mut count = 0;
    for p in input {
        if !(p[0][0] > p[1][1] || p[0][1] < p[1][0]) {
            count += 1;
        }
    }

    count
}

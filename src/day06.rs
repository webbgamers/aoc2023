use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    for (i, w) in input.chars().collect::<Vec<_>>().windows(4).enumerate() {
        if !w
            .iter()
            .any(|c1| w.iter().filter(|&c2| c1 == c2).count() > 1)
        {
            return i + 4;
        }
    }
    panic!("No answer");
}

fn part2(input: &str) -> impl Display {
    for (i, w) in input.chars().collect::<Vec<_>>().windows(14).enumerate() {
        if !w
            .iter()
            .any(|c1| w.iter().filter(|&c2| c1 == c2).count() > 1)
        {
            return i + 14;
        }
    }
    panic!("No answer");
}

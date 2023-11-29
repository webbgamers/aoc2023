use core::panic;
use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
	(part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let input = input.lines().map(|l| l.split_whitespace() );

    let mut cycle = 0;
    let mut x = 1;
    let mut strength = 0;
    for mut i in input {
        
        let (cycles, add) = match i.next().unwrap() {
            "noop" => (1, 0),
            "addx" => (2, i.next().unwrap().parse().unwrap()),
            _ => panic!("Unknown instruction"),
        };

        for _ in 0..cycles {
            cycle += 1;
            if (cycle + 20) % 40 == 0 {
                //println!("Cycle {cycle}, x={x}");
                strength += cycle * x;
            }
        }

        x += add;
    }

	strength
}

fn part2(input: &str) -> impl Display {
    let input = input.lines().map(|l| l.split_whitespace() );

    let mut cycle = 0;
    let mut x = 1;
    let mut output = String::from("");
    for mut i in input {
        
        let (cycles, add) = match i.next().unwrap() {
            "noop" => (1, 0),
            "addx" => (2, i.next().unwrap().parse().unwrap()),
            _ => panic!("Unknown instruction"),
        };

        for _ in 0..cycles {
            cycle += 1;

            let pix: i32 = cycle % 40;
            if (x+1 - pix).abs() <= 1 {
                output.push('#');
            } else {
                output.push('.');
            }
            
            if pix == 0 {
                output.push('\n');
            }
        }

        x += add;
    }

	output
}

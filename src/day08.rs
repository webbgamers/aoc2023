use std::collections::HashMap;
use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let input = input.lines().map(|l| l.chars());

    let mut seen = HashMap::new();

    let mut visible = 0;
    for (i, y) in input.clone().enumerate() {
        // from left
        let mut highest = -1;
        for (j, x) in y.clone().enumerate() {
            let c = x.to_digit(10).unwrap() as i32;
            if c > highest {
                highest = c;
                if seen.insert((j, i), true).is_none() {
                    visible += 1;
                }
            }
        }

        // from right
        let mut highest = -1;
        for (j, x) in y.clone().collect::<Vec<_>>().iter().enumerate().rev() {
            let c = x.to_digit(10).unwrap() as i32;
            if c > highest {
                highest = c;
                if seen.insert((j, i), true).is_none() {
                    visible += 1;
                }
            }
        }
    }

    let mut highests: Vec<i32> = vec![-1; input.clone().next().unwrap().count()];
    for (i, y) in input.clone().enumerate() {
        // from top
        for (x, c) in y.clone().enumerate() {
            let c = c.to_digit(10).unwrap() as i32;
            if c > highests[x] {
                highests[x] = c;
                if seen.insert((x, i), true).is_none() {
                    visible += 1;
                }
            }
        }
    }

    let mut highests: Vec<i32> = vec![-1; input.clone().next().unwrap().count()];
    for (i, y) in input.clone().collect::<Vec<_>>().iter().enumerate().rev() {
        // from bottom
        for (x, c) in y.clone().enumerate() {
            let c = c.to_digit(10).unwrap() as i32;
            if c > highests[x] {
                highests[x] = c;
                if seen.insert((x, i), true).is_none() {
                    visible += 1;
                }
            }
        }
    }

    /*for (y, l) in input.enumerate() {
        for (x, c) in l.enumerate() {
            print!("{}", if seen.get(&(x, y)).is_some() {"X"} else {" "})
        }
        println!()
    }*/

    visible
}

fn part2(input: &str) -> impl Display {
    let input = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut highest_score = 0;
    for start_y in 1..input.len() - 1 {
        for start_x in 1..input[0].len() - 1 {
            let h = input[start_y][start_x];

            // look left
            let mut l_view = 0;
            for x in (0..start_x).rev() {
                l_view += 1;
                if input[start_y][x] >= h {
                    break;
                }
            }

            // look right
            let mut r_view = 0;
            for x in start_x + 1..input[0].len() {
                r_view += 1;
                if input[start_y][x] >= h {
                    break;
                }
            }

            // look up
            let mut u_view = 0;
            for y in (0..start_y).rev() {
                u_view += 1;
                if input[y][start_x] >= h {
                    break;
                }
            }

            // look down
            let mut d_view = 0;
            #[allow(clippy::needless_range_loop)]
            for y in start_y + 1..input[0].len() {
                d_view += 1;
                if input[y][start_x] >= h {
                    break;
                }
            }

            let score = l_view * r_view * u_view * d_view;
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    highest_score
}

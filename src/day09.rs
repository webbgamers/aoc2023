use std::collections::HashSet;
use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let input = input.lines().map(|l| l.split_whitespace());

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut visited = HashSet::from([tail_pos]);
    for mut instruction in input {
        let direction = instruction.next().unwrap();
        let amount = instruction.next().unwrap().parse::<u32>().unwrap();
        for _ in 0..amount {
            let new_head_pos = match direction {
                "L" => (head_pos.0 - 1, head_pos.1),
                "R" => (head_pos.0 + 1, head_pos.1),
                "U" => (head_pos.0, head_pos.1 + 1),
                "D" => (head_pos.0, head_pos.1 - 1),
                _ => panic!("Unknown direction"),
            };
            if !touching(new_head_pos, tail_pos) {
                tail_pos = head_pos;
            }
            head_pos = new_head_pos;
            visited.insert(tail_pos);
        }
    }

    visited.len()
}

fn part2(input: &str) -> impl Display {
    let input = input.lines().map(|l| l.split_whitespace());

    let mut knots = vec![(0, 0); 10];
    let mut visited = HashSet::from([knots[9]]);
    for mut instruction in input {
        let direction = instruction.next().unwrap();
        let amount = instruction.next().unwrap().parse::<u32>().unwrap();
        for _ in 0..amount {
            knots[0] = match direction {
                "L" => (knots[0].0 - 1, knots[0].1),
                "R" => (knots[0].0 + 1, knots[0].1),
                "U" => (knots[0].0, knots[0].1 + 1),
                "D" => (knots[0].0, knots[0].1 - 1),
                _ => panic!("Unknown direction"),
            };
            for i in 1..knots.len() {
                let h = knots[i - 1];
                let mut t = knots[i];

                let xd: i32 = h.0 - t.0;
                let yd: i32 = h.1 - t.1;

                if xd.abs() > 1 && yd == 0 {
                    t.0 += xd.clamp(-1, 1);
                } else if yd.abs() > 1 && xd == 0 {
                    t.1 += yd.clamp(-1, 1);
                } else if !touching(h, t) && xd.abs() >= 1 && yd.abs() >= 1 {
                    t.0 += xd.clamp(-1, 1);
                    t.1 += yd.clamp(-1, 1);
                }

                knots[i - 1] = h;
                knots[i] = t;
            }
            visited.insert(knots[9]);
        }
    }

    visited.len()
}

fn touching(a: (i32, i32), b: (i32, i32)) -> bool {
    (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1
}

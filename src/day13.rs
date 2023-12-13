use std::{fmt::Display, iter::zip};

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let patterns = input.split("\n\n").map(|pattern_str| {
        let width = pattern_str.lines().next().unwrap().len();
        let height = pattern_str.lines().count();
        let pattern_str = pattern_str
            .lines()
            .flat_map(|l| l.chars())
            .collect::<Vec<_>>();
        (width, height, pattern_str)
    });

    let mut total = 0;

    'outer: for (width, height, pattern) in patterns {
        println!("\nPATTERN");
        // Horizontal mirroring
        for mirror_x in 1..width {
            let check_size = (width - mirror_x).min(mirror_x);
            let mut left_columns = Vec::new();
            let mut right_columns = Vec::new();
            for (left_x, right_x) in zip(
                ((mirror_x - check_size)..(mirror_x)).rev(),
                (mirror_x)..(mirror_x + check_size),
            ) {
                for y in 0..height {
                    left_columns.push(pattern[coords_to_index(left_x, y, width)]);
                    right_columns.push(pattern[coords_to_index(right_x, y, width)]);
                }
            }

            println!("L: {left_columns:?}");
            println!("R: {right_columns:?}");

            if left_columns == right_columns {
                total += mirror_x;
                continue 'outer;
            }
        }

        // Vertical mirroring
        for mirror_y in 1..height {
            let check_size = (height - mirror_y).min(mirror_y);
            let mut top_rows = Vec::new();
            let mut bottom_rows = Vec::new();
            for (top_y, bottom_y) in zip(
                ((mirror_y - check_size)..(mirror_y)).rev(),
                (mirror_y)..(mirror_y + check_size),
            ) {
                for x in 0..width {
                    top_rows.push(pattern[coords_to_index(x, top_y, width)]);
                    bottom_rows.push(pattern[coords_to_index(x, bottom_y, width)]);
                }
            }

            if top_rows == bottom_rows {
                total += mirror_y * 100;
                continue 'outer;
            }
        }

        for y in 0..height {
            for x in 0..width {
                print!("{}", pattern[coords_to_index(x, y, width)]);
            }
            println!();
        }

        panic!("No mirror line found!");
    }

    total
}

fn part2(input: &str) -> impl Display {
    let patterns = input.split("\n\n").map(|pattern_str| {
        let width = pattern_str.lines().next().unwrap().len();
        let height = pattern_str.lines().count();
        let pattern_str = pattern_str
            .lines()
            .flat_map(|l| l.chars())
            .collect::<Vec<_>>();
        (width, height, pattern_str)
    });

    let mut total = 0;

    'outer: for (width, height, pattern) in patterns {
        // Horizontal mirroring
        for mirror_x in 1..width {
            let check_size = (width - mirror_x).min(mirror_x);
            let mut left_columns = Vec::new();
            let mut right_columns = Vec::new();
            for (left_x, right_x) in zip(
                ((mirror_x - check_size)..(mirror_x)).rev(),
                (mirror_x)..(mirror_x + check_size),
            ) {
                for y in 0..height {
                    left_columns.push(pattern[coords_to_index(left_x, y, width)]);
                    right_columns.push(pattern[coords_to_index(right_x, y, width)]);
                }
            }

            let mut differences = 0;
            for i in 0..left_columns.len() {
                if left_columns[i] != right_columns[i] {
                    differences += 1;
                }
            }

            if differences == 1 {
                total += mirror_x;
                continue 'outer;
            }
        }

        // Vertical mirroring
        for mirror_y in 1..height {
            let check_size = (height - mirror_y).min(mirror_y);
            let mut top_rows = Vec::new();
            let mut bottom_rows = Vec::new();
            for (top_y, bottom_y) in zip(
                ((mirror_y - check_size)..(mirror_y)).rev(),
                (mirror_y)..(mirror_y + check_size),
            ) {
                for x in 0..width {
                    top_rows.push(pattern[coords_to_index(x, top_y, width)]);
                    bottom_rows.push(pattern[coords_to_index(x, bottom_y, width)]);
                }
            }

            let mut differences = 0;
            for i in 0..top_rows.len() {
                if top_rows[i] != bottom_rows[i] {
                    differences += 1;
                }
            }

            if differences == 1 {
                total += mirror_y * 100;
                continue 'outer;
            }
        }

        for y in 0..height {
            for x in 0..width {
                print!("{}", pattern[coords_to_index(x, y, width)]);
            }
            println!();
        }

        panic!("No mirror line found!");
    }

    total
}

fn coords_to_index(x: usize, y: usize, width: usize) -> usize {
    x + (y * width)
}

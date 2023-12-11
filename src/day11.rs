use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let mut width = input.lines().next().unwrap().len();
    let mut height = input.lines().count();
    let mut star_grid = input.lines().flat_map(|l| l.chars()).collect::<Vec<_>>();

    // Expand columns
    'cols: for x in (0..width).rev() {
        for y in 0..height {
            if star_grid[x + y * width] == '#' {
                continue 'cols;
            }
        }
        for y in (0..height).rev() {
            star_grid.insert(x + y * width + 1, '.');
        }
        width += 1;
    }

    // Expand rows
    'rows: for y in (0..height).rev() {
        for x in 0..width {
            if star_grid[x + y * width] == '#' {
                continue 'rows;
            }
        }
        for x in 0..width {
            star_grid.insert(x + y * width, '.');
        }
        height += 1;
    }

    // Create map from grid
    let mut star_map = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if star_grid[x + y * width] == '#' {
                star_map.push((x, y));
            }
        }
    }

    // Calculate distances
    let mut total = 0;
    for (i, a) in star_map.iter().enumerate() {
        for b in star_map[(i + 1)..].iter() {
            total += a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
        }
    }

    total
}

fn part2(input: &str) -> impl Display {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let star_grid = input.lines().flat_map(|l| l.chars()).collect::<Vec<_>>();

    // Store column expansions
    let mut col_expansions = Vec::new();
    'cols: for x in (0..width).rev() {
        for y in 0..height {
            if star_grid[x + y * width] == '#' {
                continue 'cols;
            }
        }
        col_expansions.push(x);
    }

    // Store row expansions
    let mut row_expansions = Vec::new();
    'rows: for y in (0..height).rev() {
        for x in 0..width {
            if star_grid[x + y * width] == '#' {
                continue 'rows;
            }
        }
        row_expansions.push(y);
    }

    // Create star map with expansions
    let mut star_map = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if star_grid[x + y * width] == '#' {
                let x_offset = col_expansions.iter().filter(|&&col| x > col).count() * 999999;
                let y_offset = row_expansions.iter().filter(|&&row| y > row).count() * 999999;
                star_map.push((x + x_offset, y + y_offset));
            }
        }
    }

    // Calculate distances
    let mut total = 0;
    for (i, a) in star_map.iter().enumerate() {
        for b in star_map[(i + 1)..].iter() {
            total += a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
        }
    }

    total
}

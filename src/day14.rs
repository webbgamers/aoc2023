use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut rock_grid = input.lines().flat_map(|l| l.chars()).collect::<Vec<_>>();

    for y in 1..height {
        for x in 0..width {
            let rock = rock_grid[coords_to_index(x, y, width)];
            if rock == 'O' {
                for test_y in (0..y).rev() {
                    if rock_grid[coords_to_index(x, test_y, width)] == '.' {
                        rock_grid[coords_to_index(x, test_y + 1, width)] = '.';
                        rock_grid[coords_to_index(x, test_y, width)] = 'O';
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let mut total = 0;

    for y in 0..height {
        for x in 0..width {
            if rock_grid[coords_to_index(x, y, width)] == 'O' {
                total += height - y;
            }
        }
    }

    total
}

fn part2(input: &str) -> impl Display {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut rock_grid = input.lines().flat_map(|l| l.chars()).collect::<Vec<_>>();

    let mut snapshots = vec![rock_grid.clone()];

    for _ in 0..1000000000 {
        // Tilt north
        for y in 1..height {
            for x in 0..width {
                let rock = rock_grid[coords_to_index(x, y, width)];
                if rock == 'O' {
                    for test_y in (0..y).rev() {
                        if rock_grid[coords_to_index(x, test_y, width)] == '.' {
                            rock_grid[coords_to_index(x, test_y + 1, width)] = '.';
                            rock_grid[coords_to_index(x, test_y, width)] = 'O';
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Tilt west
        for x in 1..width {
            for y in 0..height {
                let rock = rock_grid[coords_to_index(x, y, width)];
                if rock == 'O' {
                    for test_x in (0..x).rev() {
                        if rock_grid[coords_to_index(test_x, y, width)] == '.' {
                            rock_grid[coords_to_index(test_x + 1, y, width)] = '.';
                            rock_grid[coords_to_index(test_x, y, width)] = 'O';
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Tilt south
        for y in (0..(height - 1)).rev() {
            for x in 0..width {
                let rock = rock_grid[coords_to_index(x, y, width)];
                if rock == 'O' {
                    for test_y in (y + 1)..height {
                        if rock_grid[coords_to_index(x, test_y, width)] == '.' {
                            rock_grid[coords_to_index(x, test_y - 1, width)] = '.';
                            rock_grid[coords_to_index(x, test_y, width)] = 'O';
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Tilt east
        for x in (0..(width - 1)).rev() {
            for y in 0..height {
                let rock = rock_grid[coords_to_index(x, y, width)];
                if rock == 'O' {
                    for test_x in (x + 1)..width {
                        if rock_grid[coords_to_index(test_x, y, width)] == '.' {
                            rock_grid[coords_to_index(test_x - 1, y, width)] = '.';
                            rock_grid[coords_to_index(test_x, y, width)] = 'O';
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        if let Some(snapshot_index) = snapshots.iter().position(|grid| *grid == rock_grid) {
            let loop_size = snapshots.len() - snapshot_index;
            let remainding_rotations = (1000000000 - snapshot_index) % loop_size;

            let final_grid = &snapshots[snapshot_index + remainding_rotations];

            let mut total = 0;
            for y in 0..height {
                for x in 0..width {
                    if final_grid[coords_to_index(x, y, width)] == 'O' {
                        total += height - y;
                    }
                }
            }
            return total;
        }

        snapshots.push(rock_grid.clone());
    }

    let mut total = 0;

    for y in 0..height {
        for x in 0..width {
            if rock_grid[coords_to_index(x, y, width)] == 'O' {
                total += height - y;
            }
        }
    }

    total
}

fn coords_to_index(x: usize, y: usize, width: usize) -> usize {
    x + (y * width)
}

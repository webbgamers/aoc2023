use crate::day16::Direction::*;
use colored::*;
use core::time;
use crossterm::{cursor, ExecutableCommand};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::io::stdout;
use std::thread::sleep;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    // Build mirror map
    let mut mirror_map = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                mirror_map.insert((x, y), c);
            }
        }
    }

    let mut energized_map = HashSet::new();
    let mut beam_fronts = vec![((0, 0), Right)];

    while !beam_fronts.is_empty() {
        let mut new_beam_fronts = Vec::new();
        for ((beam_x, beam_y), beam_dir) in beam_fronts {
            if let Some(&mirror) = mirror_map.get(&(beam_x, beam_y)) {
                match mirror {
                    '|' => {
                        if !energized_map.contains(&(beam_x, beam_y)) {
                            if beam_dir == Left || beam_dir == Right {
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), Up),
                                    width,
                                    height,
                                ));
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), Down),
                                    width,
                                    height,
                                ));
                            } else {
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), beam_dir),
                                    width,
                                    height,
                                ));
                            }
                        }
                    }
                    '-' => {
                        if !energized_map.contains(&(beam_x, beam_y)) {
                            if beam_dir == Up || beam_dir == Down {
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), Left),
                                    width,
                                    height,
                                ));
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), Right),
                                    width,
                                    height,
                                ));
                            } else {
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), beam_dir),
                                    width,
                                    height,
                                ));
                            }
                        }
                    }
                    '/' => new_beam_fronts.push(march_beam(
                        (
                            (beam_x, beam_y),
                            match beam_dir {
                                Up => Right,
                                Down => Left,
                                Left => Down,
                                Right => Up,
                            },
                        ),
                        width,
                        height,
                    )),
                    '\\' => new_beam_fronts.push(march_beam(
                        (
                            (beam_x, beam_y),
                            match beam_dir {
                                Up => Left,
                                Down => Right,
                                Left => Up,
                                Right => Down,
                            },
                        ),
                        width,
                        height,
                    )),
                    mirror => panic!("Invalid mirror! {mirror}"),
                }
            } else {
                new_beam_fronts.push(march_beam(((beam_x, beam_y), beam_dir), width, height));
            }

            energized_map.insert((beam_x, beam_y));
        }
        beam_fronts = new_beam_fronts.into_iter().flatten().collect::<Vec<_>>();
    }

    energized_map.len()
}

fn part2(input: &str) -> impl Display {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    // Build mirror map
    let mut mirror_map = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                mirror_map.insert((x, y), c);
            }
        }
    }

    let mut starts = Vec::new();
    for x in 0..width {
        starts.push(((x, 0), Down));
        starts.push(((x, height - 1), Up));
    }
    for y in 0..height {
        starts.push(((0, y), Right));
        starts.push(((width - 1, y), Left));
    }

    let mut max = 0;

    for ((start_x, start_y), start_dir) in starts {
        let mut energized_map = HashSet::new();
        let mut beam_fronts = vec![((start_x, start_y), start_dir)];

        while !beam_fronts.is_empty() {
            let mut new_beam_fronts = Vec::new();
            for ((beam_x, beam_y), beam_dir) in beam_fronts {
                if let Some(&mirror) = mirror_map.get(&(beam_x, beam_y)) {
                    match mirror {
                        '|' => {
                            if !energized_map.contains(&(beam_x, beam_y)) {
                                if beam_dir == Left || beam_dir == Right {
                                    new_beam_fronts.push(march_beam(
                                        ((beam_x, beam_y), Up),
                                        width,
                                        height,
                                    ));
                                    new_beam_fronts.push(march_beam(
                                        ((beam_x, beam_y), Down),
                                        width,
                                        height,
                                    ));
                                } else {
                                    new_beam_fronts.push(march_beam(
                                        ((beam_x, beam_y), beam_dir),
                                        width,
                                        height,
                                    ));
                                }
                            }
                        }
                        '-' => {
                            if !energized_map.contains(&(beam_x, beam_y)) {
                                if beam_dir == Up || beam_dir == Down {
                                    new_beam_fronts.push(march_beam(
                                        ((beam_x, beam_y), Left),
                                        width,
                                        height,
                                    ));
                                    new_beam_fronts.push(march_beam(
                                        ((beam_x, beam_y), Right),
                                        width,
                                        height,
                                    ));
                                } else {
                                    new_beam_fronts.push(march_beam(
                                        ((beam_x, beam_y), beam_dir),
                                        width,
                                        height,
                                    ));
                                }
                            }
                        }
                        '/' => new_beam_fronts.push(march_beam(
                            (
                                (beam_x, beam_y),
                                match beam_dir {
                                    Up => Right,
                                    Down => Left,
                                    Left => Down,
                                    Right => Up,
                                },
                            ),
                            width,
                            height,
                        )),
                        '\\' => new_beam_fronts.push(march_beam(
                            (
                                (beam_x, beam_y),
                                match beam_dir {
                                    Up => Left,
                                    Down => Right,
                                    Left => Up,
                                    Right => Down,
                                },
                            ),
                            width,
                            height,
                        )),
                        mirror => panic!("Invalid mirror! {mirror}"),
                    }
                } else {
                    new_beam_fronts.push(march_beam(((beam_x, beam_y), beam_dir), width, height));
                }

                energized_map.insert((beam_x, beam_y));
            }
            beam_fronts = new_beam_fronts.into_iter().flatten().collect::<Vec<_>>();
        }
        if energized_map.len() > max {
            max = energized_map.len();
            println!("Highest start so far ({start_x},{start_y}) {start_dir:?}")
        }
        max = max.max(energized_map.len());
    }

    max
}

pub fn extra(input: String) {
    let mut stdout = stdout();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    println!();

    // Build mirror map
    let mut mirror_map = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                mirror_map.insert((x, y), c);
            }
        }
    }

    let mut energized_map = HashSet::new();
    let mut beam_fronts = vec![((6, 109), Up)];

    while !beam_fronts.is_empty() {
        let mut new_beam_fronts = Vec::new();
        for ((beam_x, beam_y), beam_dir) in beam_fronts {
            if let Some(&mirror) = mirror_map.get(&(beam_x, beam_y)) {
                match mirror {
                    '|' => {
                        if !energized_map.contains(&(beam_x, beam_y)) {
                            if beam_dir == Left || beam_dir == Right {
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), Up),
                                    width,
                                    height,
                                ));
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), Down),
                                    width,
                                    height,
                                ));
                            } else {
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), beam_dir),
                                    width,
                                    height,
                                ));
                            }
                        }
                    }
                    '-' => {
                        if !energized_map.contains(&(beam_x, beam_y)) {
                            if beam_dir == Up || beam_dir == Down {
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), Left),
                                    width,
                                    height,
                                ));
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), Right),
                                    width,
                                    height,
                                ));
                            } else {
                                new_beam_fronts.push(march_beam(
                                    ((beam_x, beam_y), beam_dir),
                                    width,
                                    height,
                                ));
                            }
                        }
                    }
                    '/' => new_beam_fronts.push(march_beam(
                        (
                            (beam_x, beam_y),
                            match beam_dir {
                                Up => Right,
                                Down => Left,
                                Left => Down,
                                Right => Up,
                            },
                        ),
                        width,
                        height,
                    )),
                    '\\' => new_beam_fronts.push(march_beam(
                        (
                            (beam_x, beam_y),
                            match beam_dir {
                                Up => Left,
                                Down => Right,
                                Left => Up,
                                Right => Down,
                            },
                        ),
                        width,
                        height,
                    )),
                    mirror => panic!("Invalid mirror! {mirror}"),
                }
            } else {
                new_beam_fronts.push(march_beam(((beam_x, beam_y), beam_dir), width, height));
            }

            energized_map.insert((beam_x, beam_y));
        }
        beam_fronts = new_beam_fronts.into_iter().flatten().collect::<Vec<_>>();

        for y in 0..height {
            for x in 0..width {
                let &c = mirror_map.get(&(x, y)).unwrap_or(&'.');
                if beam_fronts.iter().any(|&(xy, _)| xy == (x, y)) {
                    print!("{} ", "#".to_string().red().bold())
                } else if energized_map.contains(&(x, y)) {
                    print!("{} ", c.to_string().red())
                } else {
                    print!("{c} ");
                }
            }
            println!();
        }

        let _ = stdout.execute(cursor::MoveUp(height as u16));
        sleep(time::Duration::from_millis(10));
    }

    let _ = stdout.execute(cursor::MoveDown(height as u16));
    println!();
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn march_beam(
    ((beam_x, beam_y), beam_dir): ((usize, usize), Direction),
    width: usize,
    height: usize,
) -> Option<((usize, usize), Direction)> {
    match beam_dir {
        Up => Some(((beam_x, beam_y.checked_sub(1)?), Up)),
        Down => Some(((beam_x, var_checked_add_1(beam_y, height - 1)?), Down)),
        Left => Some(((beam_x.checked_sub(1)?, beam_y), Left)),
        Right => Some(((var_checked_add_1(beam_x, width - 1)?, beam_y), Right)),
    }
}

fn var_checked_add_1(n: usize, max: usize) -> Option<usize> {
    let n = n + 1;
    if n > max {
        None
    } else {
        Some(n)
    }
}

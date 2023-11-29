# Advent of Code 2022
Here we go again...
This is my testing system and (eventually) solutions for [AOC 2022](https://adventofcode.com/2022). In an attempt to get better at Rust (I'm a little rusty 😉) I am doing this all in Rust.

## How to see my solutions
`cargo run [day]` (If no day is provided during competition time, the current date is used)

## How to use testing system
Each day gets a src/day##.rs file with the definitions shown in src/template.rs and have corresponding lines in src/main.rs uncommented.

## Automatic input downloading
If no input is found in input/day##.txt, the input for the selected day will be downloaded. There must be an AOC token placed in the aoc_token file or in the AOC_TOKEN environment vairable.

## Dependencies 
 - [Rust Toolchain](https://www.rust-lang.org/tools/install)
 - Will to live (optional)

## Coming Soon
Automatic answer submittal right from the CLI!

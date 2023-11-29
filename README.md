# Advent of Code 2023
Here we go again...
This is my testing system and (eventually) solutions for [AOC 2023](https://adventofcode.com/2023). Doing everything in Rust for a 3rd year in a row because why not?

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
Automatic answer submittal right from the CLI! (didn't happen last year so we'll see)

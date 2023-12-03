use std::fmt::Display;
use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: String) -> (String, String) {
	(part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
	let num_re = Regex::new(r"\d+").unwrap();
	let sym_re = Regex::new(r"[\*/\+\%&\-=$@#]").unwrap();

	let row_size = 141;

	let mut sum = 0;

	let symbols = sym_re.find_iter(&input).map(|m| m.start() as isize ).collect::<Vec<_>>();

	for part_num in num_re.find_iter(&input) {
		if symbols.iter().any(|s| ((part_num.start() as isize - row_size - 1)..=(part_num.end() as isize - row_size)).contains(s) ) ||
		symbols.iter().any(|s| ((part_num.start() as isize + row_size - 1)..=(part_num.end() as isize + row_size)).contains(s) ) ||
		symbols.iter().any(|&s| s == part_num.start() as isize - 1) ||
		symbols.iter().any(|&s| s == part_num.end() as isize) {
			sum += part_num.as_str().parse::<i32>().unwrap();
		}
	}

	sum
}


fn part2(input: &str) -> impl Display {
	let num_re = Regex::new(r"\d+").unwrap();
	let sym_re = Regex::new(r"\*").unwrap();

	let row_size = 141;

	let mut sum = 0;

	let mut gears = sym_re.find_iter(&input).map(|m| (m.start() as isize, vec![]) ).collect::<HashMap<_,_>>();

	for part_num in num_re.find_iter(&input) {
		for n in (part_num.start() as isize - row_size - 1)..=(part_num.end() as isize - row_size) {
			if gears.contains_key(&n) {
				gears.get_mut(&n).unwrap().push(part_num.as_str().parse::<u32>().unwrap());
			}
		};
		for n in (part_num.start() as isize + row_size - 1)..=(part_num.end() as isize + row_size) {
			if gears.contains_key(&n) {
				gears.get_mut(&n).unwrap().push(part_num.as_str().parse::<u32>().unwrap());
			}
		}
		if gears.contains_key(&(part_num.start() as isize - 1)) {
			gears.get_mut(&(part_num.start() as isize - 1)).unwrap().push(part_num.as_str().parse::<u32>().unwrap());
		}
		if gears.contains_key(&(part_num.end() as isize)) {
			gears.get_mut(&(part_num.end() as isize)).unwrap().push(part_num.as_str().parse::<u32>().unwrap());
		}
	}

	for (_,g) in gears {
		if g.len() == 2 {
			sum += g[0]*g[1]
		}
	}

	sum
}

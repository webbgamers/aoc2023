use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
	(part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
	let histories = input.lines()
	.filter(|l| !l.is_empty())
	.map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
	
	let mut sum = 0;

	for h in histories {
		let mut deltas = h;
		let mut last_deltas = vec![deltas.last().unwrap().clone()];
		while !deltas.iter().all(|&d| d == 0) {
			let mut new_deltas = Vec::new();
			for x in deltas.windows(2) {
				if let &[a, b] = x {
					new_deltas.push(b - a);
				}
			}
			deltas = new_deltas;
			last_deltas.push(deltas.last().unwrap().clone());
		}
		
		sum += last_deltas.iter().sum::<i32>();
	}
	
	sum
}


fn part2(input: &str) -> impl Display {
	let histories = input.lines()
	.filter(|l| !l.is_empty())
	.map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
	
	let mut sum = 0;

	for h in histories {
		let mut deltas = h;
		deltas.reverse();
		let mut last_deltas = vec![deltas.last().unwrap().clone()];
		while !deltas.iter().all(|&d| d == 0) {
			let mut new_deltas = Vec::new();
			for x in deltas.windows(2) {
				if let &[a, b] = x {
					new_deltas.push(b - a);
				}
			}
			deltas = new_deltas;
			last_deltas.push(deltas.last().unwrap().clone());
		}
		
		sum += last_deltas.iter().sum::<i32>();
	}
	
	sum
}

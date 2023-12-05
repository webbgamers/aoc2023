use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let mut input = input.split("\n\n");
    let mut seeds = input
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let maps = input
        .map(|m| {
            m.split('\n')
                .skip(1)
                .filter(|&n| !n.is_empty())
                .map(|s| {
                    let mut s = s
                        .split(' ')
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse::<u64>().unwrap());
                    (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for s in &mut seeds {
        for m in &maps {
            for t in m {
                if (t.1..(t.1 + t.2)).contains(s) {
                    *s = t.0 + *s - t.1;
                    break;
                }
            }
        }
    }

    seeds.into_iter().min().unwrap()
}

fn part2(input: &str) -> impl Display {
    let mut input = input.split("\n\n");
    let binding = input
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let seed_ranges = binding.chunks_exact(2);

    let maps = input
        .map(|m| {
            m.split('\n')
                .skip(1)
                .filter(|&n| !n.is_empty())
                .map(|s| {
                    let mut s = s
                        .split(' ')
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse::<u64>().unwrap());
                    (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut min = u64::MAX;

    for r in seed_ranges {
        if let [s1, s2] = r {
            let mut seeds = Vec::new();
            seeds.extend(*s1..(*s1 + *s2));

            for s in &mut seeds.iter_mut() {
                for m in maps.iter() {
                    for t in m {
                        if (t.1..(t.1 + t.2)).contains(s) {
                            *s = t.0 + *s - t.1;
                            break;
                        }
                    }
                }
            }

            let local_min = seeds.into_iter().min().unwrap();

            if local_min < min {
                min = local_min;
            }
        }
    }

    min
}

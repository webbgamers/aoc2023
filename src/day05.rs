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
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let seed_ranges = binding.chunks_exact(2);

    // Vec<(start, end)>
    let mut ranges = Vec::new();
    for r in seed_ranges {
        if let [start, range] = r {
            ranges.push((*start, *start + *range - 1));
        }
    }

    // Vec<Vec<(start, end, offset)>>
    let stages = input
        .map(|m| {
            m.split('\n')
                .skip(1)
                .filter(|&n| !n.is_empty())
                .map(|s| {
                    let mut s = s
                        .split(' ')
                        .filter(|&n| !n.is_empty())
                        .map(|n| n.parse::<i64>().unwrap());
                    let dst_start = s.next().unwrap();
                    let src_start = s.next().unwrap();
                    let size = s.next().unwrap();
                    (src_start, src_start + size - 1, dst_start - src_start)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for stage in stages {
        let mut new_ranges = Vec::new();
        for &r in &ranges {
            let mut frag = vec![r];
            for &(src_start, src_end, offset) in &stage {
                let mut new_frag = Vec::new();
                for (r_start, r_end) in frag {
                    if src_start > r_end || src_end < r_start{
                        new_frag.push((r_start, r_end));
                    } else if src_start < r_end && src_start > r_start && src_end >= r_end {
                        new_frag.push((r_start, src_start - 1));
                        new_ranges.push((src_start + offset, r_end + offset));
                    } else if src_start > r_start && src_end < r_end {
                        new_frag.push((r_start, src_start - 1));
                        new_frag.push((src_end + 1, r_end));
                        new_ranges.push((src_start + offset, src_end + offset));
                    } else if src_end > r_start && src_end < r_end && src_start <= r_start {
                        new_frag.push((src_end + 1, r_end));
                        new_ranges.push((r_start + offset, src_end + offset));
                    } else if r_start >= src_start && r_end <= src_end {
                        new_ranges.push((r_start + offset, r_end + offset));
                    } else {
                        panic!("Unhandled condition!")
                    }
                }
                frag = new_frag;
            }
            new_ranges.append(&mut frag)
        }
        ranges = new_ranges;
    }

    let mut min = i64::MAX;

    for (r_start, _) in ranges {
        if r_start < min {
            min = r_start;
        }
    }

    min
}

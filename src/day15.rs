use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let init_sequences = input.lines().next().unwrap().split(',');

    let mut total = 0;
    for seq in init_sequences {
        total += hash(seq);
    }

    total
}

fn part2(input: &str) -> impl Display {
    let mut lens_boxes: [Vec<(&str, usize)>; 256] = std::array::from_fn(|_| Vec::new());
    let init_sequences = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|seq| {
            if let Some(stripped) = seq.strip_suffix('-') {
                (stripped, None)
            } else {
                let mut seq = seq.split('=');
                (
                    seq.next().unwrap(),
                    Some(seq.next().unwrap().parse::<usize>().unwrap()),
                )
            }
        })
        .collect::<Vec<_>>();

    println!("{init_sequences:?}");

    for (label, focal_length) in init_sequences {
        let box_index = hash(label);
        let lens_box = &mut lens_boxes[box_index];
        if let Some(focal_length) = focal_length {
            if let Some(lens_index) = lens_box.iter().position(|&(l, _)| l == label) {
                lens_box[lens_index] = (label, focal_length);
            } else {
                lens_box.push((label, focal_length));
            }
        } else if let Some(lens_index) = lens_box.iter().position(|&(l, _)| l == label) {
            lens_box.remove(lens_index);
        }
    }

    let mut total = 0;
    for (box_index, lens_box) in lens_boxes.iter().enumerate() {
        println!("Box {box_index}: {lens_box:?}");
        for (lens_index, &(_, focal_length)) in lens_box.iter().enumerate() {
            let mut focusing_power = box_index + 1;
            focusing_power *= lens_index + 1;
            focusing_power *= focal_length;
            total += focusing_power;
        }
    }
    total
}

fn hash(s: &str) -> usize {
    let mut hash = 0;
    for c in s.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

use itertools::Itertools;
use std::{cmp, ops::Range};

fn main() {
    let mut it = include_str!("../input.txt").trim().split("\n");

    let mut seed_intervals: Vec<(isize, isize)> = Vec::new();
    let mut input_maps: Vec<Vec<(isize, isize, isize)>> = Vec::new();

    while let Some(line) = it.next() {
        match line {
            seeds if seeds.contains("seeds") => {
                let mut it = seeds
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|seed| seed.parse::<isize>().unwrap())
                    .enumerate();

                while let Some((ind, seed_start)) = it.next() {
                    if ind % 2 == 0 {
                        let (_, seed_offset) = it.next().unwrap();
                        seed_intervals.push((seed_start, seed_start + seed_offset - 1));
                    }
                }
            }
            seed_range if seed_range.contains(":") => {
                let ranges = it
                    .clone()
                    .take_while(|line| !line.is_empty())
                    .map(|line| {
                        line.split_ascii_whitespace()
                            .map(|i| i.parse::<isize>().unwrap())
                            .next_tuple::<(isize, isize, isize)>()
                            .unwrap()
                    })
                    .collect_vec();
                input_maps.push(ranges);
            }

            _ => {}
        }
    }

    println!("{:?}", seed_intervals);

    for mappings in input_maps {
        let mut images: Vec<(isize, isize)> = Vec::new();
        while !seed_intervals.is_empty() {
            let (x, y) = seed_intervals.pop().unwrap();
            let mut completed = true;
            for (a, b, delta) in &mappings {
                let c = b + delta - 1;
                let t = b - a;

                if b <= &x && x <= y && y <= c {
                    images.push((x - t, y - t));
                    completed = false;
                    break;
                } else if b <= &x && x <= c && c < y {
                    images.push((x - t, c - t));
                    seed_intervals.push((c + 1, y));
                    completed = false;
                    break;
                } else if &x < b && b <= &y && y <= c {
                    images.push((b - t, y - t));
                    seed_intervals.push((x, b - 1));
                    completed = false;
                    break;
                }
            }
            if completed {
                images.push((x, y));
            }
        }
        seed_intervals = images.clone();
    }

    println!("{:?}", seed_intervals.iter().min().unwrap());
}

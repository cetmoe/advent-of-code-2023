use itertools::Itertools;
fn main() {
    let input = include_str!("../input.txt").trim();

    let (times, distances) = input
        .split("\n")
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .map(|seg| seg.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    let races = times.iter().zip(distances.iter()).collect_vec();

    println!("{:?}", races);

    println!(
        "{:?}",
        races
            .iter()
            .map(|(time, distance)| {
                (0..(*time - 1))
                    .reduce(|acc, i| {
                        if (*time - i) * i > **distance {
                            acc + 1
                        } else {
                            acc
                        }
                    })
                    .unwrap()
            })
            .reduce(|acc, e| acc * e)
            .unwrap()
    )
}

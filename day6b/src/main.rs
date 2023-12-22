use itertools::Itertools;
fn main() {
    let input = include_str!("../input.txt").trim();

    let (time, distance) = input
        .split("\n")
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .split_ascii_whitespace()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    println!("{:?}", (time, distance));

    let solution = (0..(time - 1))
        .reduce(|acc, i| {
            if (time - i) * i > distance {
                acc + 1
            } else {
                acc
            }
        })
        .unwrap();

    println!("{}", solution);
}

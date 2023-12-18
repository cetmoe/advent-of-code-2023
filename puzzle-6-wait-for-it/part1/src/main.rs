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
                .map(|seg| seg.parse::<usize>())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();
}

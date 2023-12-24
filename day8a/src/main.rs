use itertools::Itertools;

fn main() {
    let input = include_str!("../control.txt")
        .trim()
        .split("\n\n")
        .collect_vec();

    println!("{}", input.get(0).unwrap());
}

use itertools::Itertools;
fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt")
            .trim()
            .split("\n")
            .map(|line| {
                let initial_sequence = line
                    .trim()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect_vec();

                let mut it = initial_sequence
                    .clone()
                    .into_iter()
                    .tuple_windows::<(isize, isize)>()
                    .peekable();
                let mut next_sequence: Vec<isize> = vec![];
                let mut result: Vec<isize> = vec![];

                while let Some(tup) = it.next() {
                    next_sequence.push(tup.1 - tup.0);

                    if it.peek().is_none() {
                        if next_sequence.iter().all(|x| *x == 0) {
                            println!("{:?}", result);
                            result.push(*next_sequence.first().unwrap());
                            break;
                        } else {
                            it = next_sequence.clone().into_iter().tuple_windows().peekable();
                            result.push(*next_sequence.first().unwrap());
                            next_sequence.clear();
                        }
                    }
                }

                initial_sequence.first().unwrap() - result.iter().rev().fold(0, |acc, x| x - acc)
            })
            .sum::<isize>(),
    );
}

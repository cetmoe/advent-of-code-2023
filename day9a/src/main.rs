use itertools::Itertools;
fn main() {
    println!(
        "{:?}",
        include_str!("../control.txt")
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
                        println!("{:?}", next_sequence);
                        // println!("{:?}", result);
                        if next_sequence.iter().all(|x| *x == 0) {
                            result.push(*next_sequence.last().unwrap());
                            break;
                        } else {
                            it = next_sequence.clone().into_iter().tuple_windows().peekable();
                            result.push(*next_sequence.last().unwrap());
                            next_sequence.clear();
                        }
                    }
                }

                result.iter().sum::<isize>() + initial_sequence.last().unwrap()
            })
            .sum::<isize>(),
    );
}

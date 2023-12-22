use itertools::Itertools;

fn main() {
    let mut it = include_str!("../input.txt").trim().split("\n");

    let mut initial_seeds: Vec<usize> = Vec::new();
    let mut translation: Vec<usize> = Vec::new();

    while let Some(line) = it.next() {
        match line {
            seeds if seeds.contains("seeds") => {
                seeds
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split_ascii_whitespace()
                    .for_each(|seed| {
                        initial_seeds.push(seed.parse::<usize>().unwrap());
                    });
                translation = initial_seeds.clone();
            }
            seed_to_soil if seed_to_soil.contains(":") => {
                let ranges = it
                    .clone()
                    .take_while(|line| !line.is_empty())
                    .map(|line| {
                        line.split_ascii_whitespace()
                            .map(|i| i.parse::<usize>().unwrap())
                            .next_tuple::<(usize, usize, usize)>()
                            .unwrap()
                    })
                    .collect_vec();

                translation.iter_mut().for_each(|s| {
                    for (dest, source, range) in ranges.iter() {
                        if (source..&(source + range)).contains(&&s.clone()) {
                            *s = (*s - source) + dest;
                            break;
                        }
                    }
                })
            }
            _ => {}
        }
    }

    println!("{:?}", translation.iter().min().unwrap())
}

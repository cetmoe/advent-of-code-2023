fn main() {
    let mut it = include_str!("../control.txt").trim().split("\n");

    let mut initial_seeds: Vec<usize> = Vec::new();
    let mut translation: Vec<usize> = Vec::new();

    while let Some(line) = it.next() {
        match line {
            seeds if seeds.contains("seeds") => {
                seeds
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .trim()
                    .split(" ")
                    .for_each(|seed| {
                        let number = seed.parse::<usize>().unwrap();
                        initial_seeds.push(number);
                    });

                translation = initial_seeds.clone();
            }
            seed_to_soil if seed_to_soil.contains("seed-to-soil") => {
                it.clone()
                    .take_while(|line| !line.is_empty())
                    .for_each(|line| {
                        let numbers = line
                            .split(" ")
                            .map(|i| i.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();

                        println!("{:?}", numbers);
                        // initial_seeds.iter().for_each(||);
                    });
            }
            _ => {}
        }
    }

    println!("{:?}", initial_seeds);
}

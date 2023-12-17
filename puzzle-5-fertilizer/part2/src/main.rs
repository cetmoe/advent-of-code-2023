use itertools::Itertools;
use std::ops::Range;

fn main() {
    let mut it = include_str!("../control.txt").trim().split("\n");

    let mut initial_seed_ranges: Vec<Range<usize>> = Vec::new();
    let mut translation_seed_ranges: Vec<Vec<(Range<usize>, Range<usize>)>> = Vec::new();
    let mut revised_seeds: Vec<usize> = Vec::new();

    while let Some(line) = it.next() {
        match line {
            seeds if seeds.contains("seeds") => {
                let mut it = seeds
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|seed| seed.parse::<usize>().unwrap())
                    .enumerate();

                while let Some((ind, seed_start)) = it.next() {
                    if ind % 2 == 0 {
                        let (_, seed_offset) = it.next().unwrap();
                        initial_seed_ranges.push(seed_start..(seed_start + seed_offset));
                    }
                }
            }
            seed_range if seed_range.contains(":") => {
                let ranges = it
                    .clone()
                    .take_while(|line| !line.is_empty())
                    .map(|line| {
                        let (dest, source, offset) = line
                            .split_ascii_whitespace()
                            .map(|i| i.parse::<usize>().unwrap())
                            .next_tuple::<(usize, usize, usize)>()
                            .unwrap();

                        ((source..(source + offset)), (dest..(dest + offset)))
                    })
                    .collect_vec();
                translation_seed_ranges.push(ranges);
            }

            _ => {}
        }
    }
    let mut translations: Vec<(Range<usize>, Range<usize>)> =
        translation_seed_ranges.get(0).unwrap().to_vec();

    println!("{:?}", translations);

    let mut trans_it = translation_seed_ranges.iter().skip(1);

    while let Some(search_space) = trans_it.next() {
        println!("line");
        let mut temp: Vec<(Range<usize>, Range<usize>)> = Vec::new();
        let mut temp_unsolved: Vec<(Range<usize>, Range<usize>)> = Vec::new();
        for (trans_src, trans_dest) in search_space.iter() {
            println!("{:?}", (trans_src, trans_dest));
            for (init_src, init_dest) in &translations {
                if init_dest.contains(&trans_src.start) && init_dest.contains(&trans_src.end) {
                    let fst = trans_src.start - init_dest.start;
                    if fst > 0 {
                        temp.push((
                            init_src.start..(init_src.start + fst),
                            init_dest.start..(init_dest.start + fst),
                        ));
                    }
                    let snd = trans_src.end - trans_src.start;
                    temp.push((
                        (init_src.start + fst)..(init_src.start + fst + snd),
                        trans_dest.start..trans_dest.end,
                    ));
                    let thd = init_dest.end - trans_src.end;
                    if thd > 0 {
                        temp.push((
                            (init_src.start + fst + snd)..(init_src.start + fst + snd + thd),
                            (init_dest.start + fst + snd)..(init_dest.start + fst + snd + thd),
                        ));
                    }
                } else if init_dest.contains(&trans_src.start)
                    && !init_dest.contains(&trans_src.end)
                {
                    let fst = trans_src.start - init_dest.start;
                    if fst > 0 {
                        temp.push((
                            init_src.start..(init_src.start + fst),
                            init_dest.start..(init_src.start + fst),
                        ))
                    }
                    let snd = init_dest.end - trans_src.start;
                    if snd > 0 {
                        temp.push((
                            (init_src.start + fst)..(init_src.start + fst + snd),
                            trans_dest.start..(trans_dest.start + snd),
                        ))
                    }
                    let thd = trans_dest.end - snd;
                    temp_unsolved.push((
                        init_src.end..trans_src.end,
                        (trans_dest.start + thd)..trans_dest.end,
                    ))
                } else if !init_dest.contains(&trans_src.start)
                    && init_dest.contains(&trans_src.end)
                {
                    let fst = init_dest.start - trans_src.start;
                    temp_unsolved.push((
                        trans_src.start..init_dest.start,
                        trans_dest.start..(trans_dest.start + fst),
                    ));

                    let snd = trans_src.end - init_src.start;
                    if snd > 0 {
                        temp.push((
                            init_src.start..(init_src.start + snd),
                            (trans_dest.start + fst)..(trans_dest.start + fst + snd),
                        ))
                    }
                    temp.push((
                        (init_src.start + snd)..(init_src.end),
                        (init_dest.start + snd)..(init_dest.end),
                    ))
                }
            }
        }
        temp = temp.clone().into_iter().unique().collect_vec();
        for (unsolved_src, unsolved_dest) in &temp_unsolved {
            for (temp_src, temp_dest) in &temp {
                if unsolved_src.contains(&temp_dest.start) || unsolved_src.contains(&temp_dest.end)
                {
                }
            }
        }

        // translations = temp.clone().into_iter().unique().collect_vec();

        println!("{:?}", temp_unsolved);
        println!("{:?}", temp);
    }
}

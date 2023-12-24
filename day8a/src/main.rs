use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt")
        .trim()
        .split("\n\n")
        .collect_vec();

    let sequence = input.get(0).unwrap();
    let mut starts: Vec<String> = Vec::new();
    let mut maps: HashMap<String, (String, String)> = HashMap::new();

    input.get(1).unwrap().split("\n").for_each(|line| {
        let mut ls = line.split(" = ");

        let key = ls.nth(0).unwrap().to_owned();

        let value: (String, String) = ls
            .nth(0)
            .unwrap()
            .replace(&['(', ')'], "")
            .split(",")
            .map(|x| x.trim().to_owned())
            .collect_tuple()
            .unwrap();

        if key.chars().last().unwrap() == 'A' {
            starts.push(key.clone());
        }

        maps.insert(key, value);
    });

    let mut it = sequence.chars().peekable();
    let mut current = String::from("AAA");
    let mut count = 0;

    while let Some(ch) = it.next() {
        count += 1;
        let tuple = maps.get(current.as_str()).unwrap().clone();
        match ch {
            'L' => {
                current = tuple.0;
            }
            'R' => {
                current = tuple.1;
            }
            _ => unreachable!(),
        }

        if current.chars().last().unwrap() == 'Z' {
            break;
        }

        if it.peek().is_none() {
            it = sequence.chars().peekable();
        }
    }

    println!("{}", count);
}

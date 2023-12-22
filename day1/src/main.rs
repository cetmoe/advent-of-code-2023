use regex::{Regex, RegexSet};
use std::{collections::HashMap, fs, os::unix::prelude::FileExt};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.trim().split("\n");

    let patterns = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    let re = Regex::new(r"zero|one|two|three|four|five|six|seven|eight|nine|0|1|2|3|4|5|6|7|8|9")
        .unwrap();

    let shortest = patterns
        .iter()
        .fold(25, |acc, s| if s.len() < acc { s.len() } else { acc });

    let longest = patterns
        .iter()
        .fold(0, |acc, s| if s.len() > acc { s.len() } else { acc });

    let sum = lines.fold(0, |mut acc, line| {
        let mut strs: Vec<&str> = Vec::new();
        for (ind, _) in line.chars().enumerate() {
            for i in shortest..=longest {
                let end = if ind + i > line.len() {
                    line.len()
                } else {
                    ind + i
                };

                let x: Vec<_> = re
                    .find_iter(&line[ind..end])
                    .map(|m| &line[ind..end][m.start()..m.end()])
                    .collect();

                x.iter().for_each(|s| {
                    strs.push(s);
                });
            }
        }
        let number = format!(
            "{}{}",
            str_to_numeric(strs.first().unwrap()),
            str_to_numeric(strs.last().unwrap())
        );

        acc += number.parse::<u32>().unwrap();
        acc
    });
    println!("{}", sum);
}

fn str_to_numeric(input: &str) -> &str {
    let matches = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    if input.len() == 1 {
        input
    } else {
        matches.get(input).unwrap()
    }
}

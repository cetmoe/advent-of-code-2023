use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.trim().split("\n");

    let dict = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let sum = lines.fold(0, |mut acc, line| {
        let chars_in_line = line.chars();

        let mut number = String::new();

        let left = chars_in_line
            .clone()
            .enumerate()
            .filter(|(idx, c)| c.is_numeric() || idx == &chars_in_line.clone().count())
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>()[0];

        let right = chars_in_line
            .clone()
            .rev()
            .enumerate()
            .filter(|(idx, c)| c.is_numeric() || idx == &chars_in_line.clone().count())
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>()[0];

        for (i, c) in chars_in_line.clone().enumerate() {
            dict.iter().enumerate().for_each(|(int, word)| {
                if line[0..i].contains(word) {
                    number = String::from(format!("{}", int));
                } else if c.is_numeric() {
                    number = String::from(format!("{}", c));
                }
            })
        }

        acc += number.parse::<u32>().unwrap();
        acc
    });

    println!("{}", sum);
}

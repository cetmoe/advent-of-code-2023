use std::cmp::max;
use std::str::Chars;

fn main() {
    let input = include_str!("../control.txt").trim();

    let x = input.split("\n").nth(0).unwrap().trim().len();
    let y = input.split("\n").count();

    println!("{} {}", x, y);

    let matrix = input
        .split("\n")
        .map(|line| line.trim().chars())
        .collect::<Vec<Chars>>();

    let valid_numbers = input
        .split("\n")
        .enumerate()
        .map(|(linenr, line)| {
            let mut valid_numbers_on_line: Vec<usize> = Vec::new();
            let mut it = line.chars().enumerate();
            while let Some((i, c)) = it.next() {
                if c.is_numeric() {
                    let string_representation_of_number = &line[i..]
                        .chars()
                        .take_while(|x| x.is_numeric())
                        .collect::<Vec<char>>();

                    let number_representation = string_representation_of_number
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();

                    println!("{:?}", number_representation);

                    valid_numbers_on_line.push(number_representation);
                    it.nth(string_representation_of_number.len());
                    continue;
                }
            }

            valid_numbers_on_line
        })
        .flatten()
        .collect::<Vec<usize>>();

    println!("{:?}", valid_numbers);
}

fn is_number_valid(x: usize, y: usize, len: usize, matrix: Vec<Chars>) -> bool {
    let upper_region: usize = sub_usize(y, 1);
    let bottom_region = max(y + 1, matrix.len());
    let left_region = sub_usize(x, 1);
    let right_region = max(x + 1, matrix.get(0).unwrap().clone().count());

    true
}

fn sub_usize(n: usize, s: usize) -> usize {
    if n >= s {
        n - s
    } else {
        0
    }
}

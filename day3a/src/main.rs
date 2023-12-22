use std::cmp::max;
use std::cmp::min;
use std::str::Chars;

fn main() {
    let input = include_str!("../input.txt").trim();

    let matrix = input
        .split("\n")
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

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

                    if is_number_valid(i, linenr, string_representation_of_number.len(), &matrix) {
                        valid_numbers_on_line.push(number_representation);
                    }
                    it.nth(string_representation_of_number.len() - 1);
                    continue;
                }
            }

            valid_numbers_on_line
        })
        .flatten()
        .sum::<usize>();

    println!("{:?}", valid_numbers);
}

fn is_number_valid(x: usize, y: usize, len: usize, matrix: &Vec<Vec<char>>) -> bool {
    let mut result = false;

    for i in sub_usize(y, 1)..(y + 2) {
        for j in sub_usize(x, 1)..(x + len + 1) {
            if let Some(row) = matrix.get(i) {
                if let Some(item) = row.get(j) {
                    if item != &'.' && !item.is_numeric() {
                        result = true;
                    }
                }
            }
        }
    }

    result
}

fn sub_usize(n: usize, s: usize) -> usize {
    if n > s {
        n - s
    } else {
        0
    }
}

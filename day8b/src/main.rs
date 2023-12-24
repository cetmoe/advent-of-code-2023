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
    let mut count = 0;
    let mut solutions: Vec<usize> = Vec::new();

    while let Some(ch) = it.next() {
        count += 1;
        let mut sols: Vec<usize> = Vec::new();
        match ch {
            'L' => {
                for (i, s) in starts.iter_mut().enumerate() {
                    let new = maps.get(s).unwrap().clone().0;
                    if new.chars().last().unwrap() != 'Z' {
                        *s = new;
                    } else {
                        solutions.push(count);
                        sols.push(i);
                    }
                }
            }
            'R' => {
                for (i, s) in starts.iter_mut().enumerate() {
                    let new = maps.get(s).unwrap().clone().1;
                    if new.chars().last().unwrap() != 'Z' {
                        *s = new;
                    } else {
                        solutions.push(count);
                        sols.push(i);
                    }
                }
            }
            _ => unreachable!(),
        }

        starts = starts
            .iter()
            .enumerate()
            .filter(|(i, _)| !sols.contains(i))
            .map(|(_, s)| s.clone())
            .collect_vec();

        if it.peek().is_none() && !starts.is_empty() {
            it = sequence.chars().peekable();
        }
    }

    println!("{:?}", lcm(&solutions));
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

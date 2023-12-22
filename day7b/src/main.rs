use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let card_rankings = HashMap::from([
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('J', 0),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);

    let mut hands = include_str!("../input.txt")
        .trim()
        .split("\n")
        .map(|line| {
            let mut sp = line.split_ascii_whitespace();

            let hand = sp.next().unwrap();
            let multi = sp.next().unwrap().parse::<usize>().unwrap();

            (hand, rate_hand(hand), multi)
        })
        .collect_vec();

    hands.sort_unstable_by(|(a_hand, a_rating, _), (b_hand, b_rating, _)| {
        if a_rating > b_rating {
            return Ordering::Greater;
        } else if a_rating == b_rating {
            for (i, a_ch) in a_hand.char_indices() {
                let a_ranking = card_rankings.get(&a_ch).unwrap();
                let b_ranking = card_rankings.get(&b_hand.chars().nth(i).unwrap()).unwrap();

                if a_ranking > b_ranking {
                    return Ordering::Greater;
                } else if a_ranking < b_ranking {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        } else {
            return Ordering::Less;
        }
    });

    println!(
        "{}",
        hands
            .iter()
            .enumerate()
            .map(|(i, (_, _, n))| n * (i + 1))
            .sum::<usize>(),
    );
}

fn rate_hand(hand: &str) -> usize {
    let mut cards = HashMap::new();

    let mut jokers = 0;

    for ch in hand.chars() {
        if ch == 'J' {
            jokers += 1;
        } else {
            cards.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }
    }

    if jokers != 5 {
        let largest = cards
            .iter()
            .reduce(|(k1, v1), (k2, v2)| {
                if v1 > v2 {
                    return (k1, v1);
                }
                (k2, v2)
            })
            .unwrap()
            .0;

        cards.entry(*largest).and_modify(|c| *c += jokers);
    } else {
        return 7;
    }

    let mut count = HashMap::from([(1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]);

    for (_, occ) in cards {
        count.entry(occ).and_modify(|c| *c += 1);
    }
    // this is so ugly

    if count.get(&5).unwrap() == &1 {
        7
    } else if count.get(&4).unwrap() == &1 {
        6
    } else if count.get(&3).unwrap() == &1 && count.get(&2).unwrap() == &1 {
        5
    } else if count.get(&3).unwrap() == &1 {
        4
    } else if count.get(&2).unwrap() == &2 {
        3
    } else if count.get(&2).unwrap() == &1 {
        2
    } else {
        1
    }
}

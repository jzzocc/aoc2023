use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input/day04.txt").unwrap();
    let cards = input.split('\n');
    let mut card_copies = HashMap::<usize, i32>::new();

    for (card_index, card) in cards.enumerate() {
        let numbers: Vec<Vec<i32>> = card
            .split(':')
            .last()
            .unwrap()
            .split('|')
            .map(|n| {
                n.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
        let winning_numbers: HashSet<i32> = numbers[0].clone().into_iter().collect();
        let my_numbers: HashSet<i32> = numbers[1].clone().into_iter().collect();
        let intersection = my_numbers.intersection(&winning_numbers).count();

        *card_copies.entry(card_index).or_default() += 1;

        for _ in 0..*card_copies.entry(card_index).or_default() {
            for c in (card_index + 1)..=(card_index + intersection) {
                *card_copies.entry(c).or_default() += 1;
            }
        }
    }

    dbg!(card_copies.values().sum::<i32>());
}

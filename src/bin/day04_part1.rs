use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input/day04.txt").unwrap();
    let cards = input.split('\n');

    let result = cards
        .map(|card| {
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
            let intersection = my_numbers.intersection(&winning_numbers).count() as u32;

            if intersection > 0 {
                i32::pow(2, intersection - 1)
            } else {
                0
            }
        })
        .sum::<i32>();

    dbg!(result);
}

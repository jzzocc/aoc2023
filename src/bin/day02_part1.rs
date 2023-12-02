use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let game_re = Regex::new(r"Game (\d+):").unwrap();
    let colors = HashMap::from([
        (12, Regex::new(r"(\d+) red").unwrap()),
        (13, Regex::new(r"(\d+) green").unwrap()),
        (14, Regex::new(r"(\d+) blue").unwrap()),
    ]);

    let input = fs::read_to_string("input/day02.txt").unwrap();
    let games = input.split('\n');

    let result = games
        .map(|game| {
            let possible = game.split(';').all(|r| {
                colors.iter().all(|(&max, re)| {
                    let instances = match re.captures(r) {
                        Some(c) => c.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                        _ => 0,
                    };

                    instances <= max
                })
            });

            if possible {
                game_re
                    .captures(game)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap()
            } else {
                0
            }
        })
        .sum::<i32>();

    dbg!(result);
}

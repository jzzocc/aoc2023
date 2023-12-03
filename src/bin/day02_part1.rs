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
        .filter_map(|game| {
            let possible = game.split(';').all(|r| {
                colors.iter().all(|(&max, re)| {
                    re.captures(r)
                        .and_then(|c| c.get(1))
                        .and_then(|c| c.as_str().parse::<i32>().ok())
                        .unwrap_or(0)
                        <= max
                })
            });

            if possible {
                Some(
                    game_re
                        .captures(game)
                        .and_then(|c| c.get(1))
                        .and_then(|c| c.as_str().parse::<i32>().ok())
                        .unwrap(),
                )
            } else {
                None
            }
        })
        .sum::<i32>();

    dbg!(result);
}

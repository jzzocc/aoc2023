use regex::Regex;
use std::fs;

fn main() {
    let color_res = vec![
        Regex::new(r"(\d+) red").unwrap(),
        Regex::new(r"(\d+) green").unwrap(),
        Regex::new(r"(\d+) blue").unwrap(),
    ];
    let input = fs::read_to_string("input/day02.txt").unwrap();
    let games = input.split('\n');

    let result = games
        .map(|game| {
            let reveals = game.split(';');
            color_res
                .iter()
                .map(move |color_re| {
                    reveals
                        .clone()
                        .map(|reveal| {
                            if let Some(captures) = color_re.captures(reveal) {
                                captures.get(1).unwrap().as_str().parse::<i32>().unwrap()
                            } else {
                                0
                            }
                        })
                        .max()
                        .unwrap()
                })
                .product::<i32>()
        })
        .sum::<i32>();

    dbg!(result);
}

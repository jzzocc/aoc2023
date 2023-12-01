use regex::Regex;
use std::fs;

fn main() {
    let re = Regex::new(r"\D").unwrap();
    let input = fs::read_to_string("input/day01.txt").unwrap();
    let lines = input.split('\n');
    let result = lines
        .map(|line| {
            let nums = re.replace_all(line, "").to_string();
            (nums.chars().next().unwrap().to_string() + &nums.chars().last().unwrap().to_string())
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>();

    dbg!(result);
}

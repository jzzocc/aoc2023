use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let numbers = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let number_names = numbers.keys().map(|s| &**s).collect::<Vec<_>>().join("|");
    let re = format!("({}{}", number_names, r"|\d)");
    let first_re = Regex::new(&re).unwrap();
    let last_re = Regex::new(&(String::from(".*") + &re)).unwrap();
    let input = fs::read_to_string("input/day01.txt").unwrap();
    let lines = input.split('\n');
    let result = lines
        .map(|line| {
            let first_number = &first_re.captures(line).unwrap()[0];
            let last_number = &last_re.captures(line).unwrap()[1];
            let number_string = String::from(*numbers.get(first_number).unwrap_or(&first_number))
                + *numbers.get(last_number).unwrap_or(&last_number);
            number_string.parse::<i32>().unwrap()
        })
        .sum::<i32>();

    dbg!(result);
}

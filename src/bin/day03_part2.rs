use regex::Regex;
use std::{collections::HashMap, fs, ops::Range};

fn main() {
    let number_re = Regex::new(r"\d+").unwrap();
    let gear_re = Regex::new(r"\*").unwrap();

    let mut numbers_per_line = Vec::<Vec<(i32, Range<usize>)>>::new();
    let mut gears_per_line = Vec::<Vec<Range<usize>>>::new();
    let mut numbers_per_gear = HashMap::<String, Vec<i32>>::new();

    let input = fs::read_to_string("input/day03.txt").unwrap();
    let lines = input.split('\n');

    for (line_index, line) in lines.enumerate() {
        let nums: Vec<(i32, Range<usize>)> = number_re
            .find_iter(line)
            .map(|m| (m.as_str().parse().unwrap(), m.start()..m.end()))
            .collect();
        let gears: Vec<Range<usize>> = gear_re
            .find_iter(line)
            .map(|m| (m.start() - 1)..(m.end() + 1))
            .collect();

        for (n, n_range) in &nums {
            for gear in &gears {
                if gear.contains(&n_range.start) || gear.contains(&(n_range.end - 1)) {
                    numbers_per_gear
                        .entry(format!("{}|{:?}", line_index, gear))
                        .or_default()
                        .push(*n);
                }
            }

            if line_index > 0 {
                for gear in &gears_per_line[line_index - 1] {
                    if gear.contains(&n_range.start) || gear.contains(&(n_range.end - 1)) {
                        numbers_per_gear
                            .entry(format!("{}|{:?}", line_index - 1, gear))
                            .or_default()
                            .push(*n);
                    }
                }
            }
        }

        if line_index > 0 {
            for (n, n_range) in &numbers_per_line[line_index - 1] {
                for gear in &gears {
                    if gear.contains(&n_range.start) || gear.contains(&(n_range.end - 1)) {
                        numbers_per_gear
                            .entry(format!("{}|{:?}", line_index, gear))
                            .or_default()
                            .push(*n);
                    }
                }
            }
        }

        numbers_per_line.push(nums);
        gears_per_line.push(gears);
    }

    let result = numbers_per_gear
        .values()
        .filter_map(|n| match n.len() {
            2 => Some(n.iter().product::<i32>()),
            _ => None,
        })
        .sum::<i32>();

    dbg!(result);
}

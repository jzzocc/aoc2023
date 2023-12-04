use regex::Regex;
use std::{fs, ops::Range};

fn main() {
    let number_re = Regex::new(r"\d+").unwrap();
    let joiner_re = Regex::new(r"[^.\d]").unwrap();

    let mut numbers_per_line = Vec::<Vec<(i32, Range<usize>)>>::new();
    let mut joiners_per_line = Vec::<Vec<usize>>::new();
    let mut numbers_included: Vec<i32> = Vec::new();

    let input = fs::read_to_string("input/day03.txt").unwrap();
    let lines = input.split('\n');

    for (line_index, line) in lines.enumerate() {
        let nums: Vec<(i32, Range<usize>)> = number_re
            .find_iter(line)
            .map(|m| (m.as_str().parse().unwrap(), m.start()..m.end()))
            .collect();
        let joiners: Vec<usize> = joiner_re
            .find_iter(line)
            .flat_map(|m| Vec::from_iter((m.start() - 1)..(m.end() + 1)))
            .collect();

        for (n, n_range) in &nums {
            if joiners.iter().any(|j| n_range.contains(j)) {
                numbers_included.push(*n);
            }
            if line_index > 0
                && joiners_per_line[line_index - 1]
                    .iter()
                    .any(|j| n_range.contains(j))
            {
                numbers_included.push(*n);
            }
        }

        if line_index > 0 {
            for (n, n_range) in &numbers_per_line[line_index - 1] {
                if joiners.iter().any(|j| n_range.contains(j)) {
                    numbers_included.push(*n);
                }
            }
        }

        numbers_per_line.push(nums);
        joiners_per_line.push(joiners);
    }

    dbg!(numbers_included.iter().sum::<i32>());
}

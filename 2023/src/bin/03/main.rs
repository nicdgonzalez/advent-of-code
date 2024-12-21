//! Solution for Advent of Code 2023, day .
//! https://adventofcode.com/2024/day/
//!
//! Attemped on: 2024-12-04

use std::{fs, path};

/// A brief description about part one.
#[must_use]
fn part_one(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0;

    // `i` represents the line number. (max. lines: 140)
    for i in 0..lines.len() {
        let line = lines.iter().nth(i).unwrap();

        // `j` represents the column number. (max. columns: 140)
        for j in 0..line.len() {
            let current = line.chars().nth(j).unwrap();

            if current == '.' {
                continue;
            }

            assert!(current.is_ascii_digit() || current.is_ascii_punctuation());

            let line_below = lines.iter().nth(i + 1);
            let right = line.chars().nth(j + 1);
            let bottom_current = line_below.and_then(|line| line.chars().nth(j));
            let bottom_right = line_below.and_then(|line| line.chars().nth(j + 1));

            if current.is_numeric()
                && (right.is_some_and(|c| c.is_ascii_punctuation())
                    || bottom_current.is_some_and(|c| c.is_ascii_punctuation())
                    || bottom_right.is_some_and(|c| c.is_ascii_punctuation()))
            {
                let part_number: i32 = line
                    .chars()
                    .rev()
                    .skip(line.len() - j - 1)
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                total += part_number;
            }
        }
    }

    total
}

/// A brief description about part two.
#[must_use]
fn part_two(input: &str) -> i32 {
    todo!()
}

fn main() {
    let current_dir = path::Path::new(file!()).parent().unwrap();
    let input_path = current_dir.join("input.txt");
    let input = fs::read_to_string(input_path).expect("failed to read input");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

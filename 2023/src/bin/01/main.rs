//! Solution for Advent of Code 2023, day .
//! https://adventofcode.com/2024/day/
//!
//! Attemped on: 2024-12-04

use std::{fs, path};

/// Each line of data looks something like this:
///
///     1abc2
///     pqr3stu8vwx
///     treb7uchet
///     [...]
///
/// Take the first digit and the last digit of each line to form a single
/// two-digit value. Return the sum of all these values.
#[must_use]
fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let first = line.chars().skip_while(|c| !c.is_numeric()).next().unwrap();
            let last = line
                .chars()
                .rev()
                .skip_while(|c| !c.is_numeric())
                .next()
                .unwrap();

            let value: i32 = format!("{}{}", first, last).parse().unwrap();
            value
        })
        .sum()
}

/// Same as before, except now some digits may be spelled out in English.
#[must_use]
fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let first = get_value(&line, 0..line.len()).unwrap();
            let last = get_value(&line, (0..line.len()).rev()).unwrap();
            (first * 10) + last
        })
        .sum()
}

fn get_value(line: &str, range: impl Iterator<Item = usize>) -> Option<i32> {
    static CONVERTER: &[(&'static str, u8)] = &[
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for i in range {
        let c = line.chars().nth(i).unwrap();

        if c.is_numeric() {
            assert!(c.is_ascii_digit());
            return Some(c.to_digit(10).unwrap() as i32);
        }

        let remaining: String = line.chars().skip(i).collect();

        for (name, value) in CONVERTER.iter() {
            if remaining.starts_with(name) {
                return Some(*value as i32);
            }
        }
    }

    None
}

fn main() {
    let current_dir = path::Path::new(file!()).parent().unwrap();
    let input_path = current_dir.join("input.txt");
    let input = fs::read_to_string(input_path).expect("failed to read input");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

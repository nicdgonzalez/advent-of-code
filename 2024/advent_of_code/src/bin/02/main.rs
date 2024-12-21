//! Solution for Advent of Code 2024, day 2.
//! https://adventofcode.com/2024/day/2
//!
//! Attempted on 2024-12-21

use std::{fs, path};

#[must_use]
fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            debug_assert!(numbers.len() > 1);
            // The first two element set the pattern for the rest to follow.
            // If [0] < [1], then [1] must be less than [2], and so on.
            let increasing = numbers[0] < numbers[1];

            numbers.windows(2).all(|pair| {
                let (a, b) = (pair[0], pair[1]);
                let distance = (a - b).abs();

                ((a < b) == increasing) && (distance > 0 && distance < 4)
            }) as i32
        })
        .sum()
}

#[must_use]
fn part_two(_input: &str) -> i32 {
    todo!()
}

fn main() {
    let current_file = path::Path::new(file!())
        .canonicalize()
        .expect("failed to resolve symbolic links");
    let current_dir = current_file.parent().unwrap();
    let input_path = current_dir.join("input.txt");
    let input = fs::read_to_string(input_path).expect("failed to read input");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}

//! Solution for Advent of Code 2024, day 2.
//! https://adventofcode.com/2024/day/2
//!
//! Attempted on 2024-12-21

use std::{cmp::Ordering, fs, path};

#[must_use]
fn part_one(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
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
            })
        })
        .count()
        .try_into()
        .unwrap()
}

#[must_use]
fn part_two(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            debug_assert!(numbers.len() > 1);
            let ascending = match numbers[0].cmp(&numbers[1]) {
                Ordering::Less => true,
                // If the first two values are equal, in order for the report
                // to be "safe" then all other levels need to be valid. We can
                // discard the first value, then treat this report like
                // part one where it is marked unsafe after the first fail.
                Ordering::Equal => {
                    let ascending = numbers[1] < numbers[2];
                    let remainder = &numbers[1..];

                    return remainder.windows(2).all(|pair| {
                        let (a, b) = (pair[0], pair[1]);
                        let distance = (a - b).abs();

                        ((a < b) == ascending) && (1..=3).contains(&distance)
                    });
                }
                Ordering::Greater => false,
            };

            let distances: Vec<i32> = numbers.windows(2).map(|pair| pair[0] - pair[1]).collect();
            let mut tolerance = 1;
            let mut i = 0;

            while i < distances.len() {
                let a = distances[i];

                if ((a < 0) == ascending) && (1..=3).contains(&a.abs()) {
                    i += 1;
                    continue;
                }

                if tolerance > 0 {
                    tolerance -= 1;
                    i += 1;
                    continue;
                }

                return false;
            }

            true
        })
        .count()
        .try_into()
        .unwrap()
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

//! Solution for Advent of Code 2024, day 2.
//! https://adventofcode.com/2024/day/2
//!
//! Attempted on 2024-12-21

use std::{cmp::Ordering, fs, path};

/// Each line of data looks something like this:
///
///     7 6 4 2 1
///
/// Each line of input is a report, and each number in a report is a level.
/// Return the number of reports where every level is either all increasing
/// or all decreasing, and the distance between each level is at least one,
/// but not greater than three.
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

/// Same as part one, except you are allowed to discard a single bad level
/// if removing that level would have made the rest of the report valid.
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
                Ordering::Less => true,
            };

            #[rustfmt::skip]
            let distances: Vec<i32> = numbers
                .windows(2)
                .map(|pair| pair[0] - pair[1])
                .collect();

            let valid_distance = |distance: &&i32| {
                ((**distance < 0) == ascending) && (1..=3).contains(&distance.abs())
            };

            let mut iter = distances.iter();
            // Discard only the first invalid value.
            let _ = iter.by_ref().skip_while(valid_distance).next();

            iter.all(|distance| valid_distance(&distance))
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

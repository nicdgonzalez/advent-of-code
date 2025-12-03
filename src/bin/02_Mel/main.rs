//! Solution for Advent of Code 2025, day 2.

#![feature(iter_array_chunks)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[must_use]
fn part_one(input: &str) -> i32 {
    0 // TODO: Mel has to write this part.
}

#[must_use]
fn part_two(input: &str) -> i64 {
    let data = input.split(",");
    let mut total = 0;

    for entry in data {
        let (start, stop) = entry
            .trim()
            .split_once("-")
            .map(|(start, stop)| {
                // println!("{start}");
                let start = start.parse::<i64>().unwrap();
                // println!("{stop}");
                let stop: i64 = stop.parse().unwrap();

                (start, stop)
            })
            .unwrap();

        for n in start..=stop {
            let num = n.to_string();
            let num_length = num.len();

            for div in 2..=num_length {
                if num_length.is_multiple_of(div) {
                    let nums = num
                        .chars()
                        .collect::<Vec<_>>()
                        .chunks(num_length / div)
                        .map(|chunk| chunk.iter().collect::<String>())
                        .collect::<Vec<_>>();

                    let nums: HashSet<_> = nums.into_iter().collect();

                    if nums.len() == 1 {
                        // println!("Found {n} to have {nums:?} symmetry");
                        // println!("{n}");
                        total += n;
                        break;
                    }
                }
            }
        }
    }

    // println!("{total}");
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part_one(input), 1227775554);
    }
}

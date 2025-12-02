//! # Day 1: Secret Entrance
//!
//! Solution for Advent of Code 2025, day 1.
//! https://adventofcode.com/2025/day/1
//!
//! Save puzzle input to the same directory at: `./input.txt`.
//! Run the program with: `cargo run --bin 01`

advent_of_code::advent_of_code!();

#[must_use]
fn part_one(input: &str) -> i32 {
    let mut dial = 50;
    let mut password = 0;

    for line in input.trim().lines() {
        let distance = line[1..].parse::<i32>().unwrap();

        dial += if line.starts_with('L') {
            -distance
        } else {
            distance
        };

        while !(0..100).contains(&dial) {
            dial += if dial > 99 { -100 } else { 100 };
        }

        if dial == 0 {
            password += 1;
        }
    }

    password
}

#[must_use]
fn part_two(input: &str) -> i32 {
    let mut dial = 50;
    let mut password = 0;

    for line in input.trim().lines() {
        let distance = line[1..].parse::<i32>().unwrap();

        if *line.as_bytes().first().unwrap() == b'L' {
            // # TL;DR
            //
            // Treat the dial as if it were unwrapped on an infinite number line. Every time the
            // unwrapped position crosses 0 or a multiple of 100 (the bounds of our 0-to-99 dial),
            // the dial would have pointed at 0 once. The formula below computes how many times
            // a rotation crosses that boundary.
            //
            // # Deeper explanation
            //
            // The dial ranges from 0 to 99. When rotating left, the dial can go negative. Our goal
            // is to count how many times the rotation passes through our theoretical 0.
            //
            // The formula `dial - distance` gives us an unwrapped distance which makes it possible
            // to count how many times we would have crossed through 0. To ensure we properly handle
            // the case where we start at 0 (rotating from 1 -> 0, then 0 -> 99 should count as
            // crossing 0 once, not twice), we need to first normalize our `dial` value:
            //
            //      `(dial + 99) % 100`
            //
            // If the dial is at 0 and we rotate L40:
            //
            //      (99 - ((0 + 99) % 100) - 40) / 100
            //
            //      (0 + 99) % 100 = 99 % 100 = 99
            //      → 99 - 40 = 59
            //      → 99 - 59 = 40
            //      → 40 / 100 = 0  (In other words, we never crossed 0)
            //
            // If the dial is at 10 and we rotate L40:
            //
            //      (99 - ((10 + 99) % 100) - 40) / 100
            //
            //      (10 + 99) % 100 = 109 % 100 = 9
            //      → 9 - 40 = -31
            //      → 99 - (-31) = 130
            //      → 130 / 100 = 1  (In other words, we crossed 0 once)
            //
            // 99 represents the maximum value on the dial.
            // 100 represents the total number of values on the dial.
            password += (99 - (((dial + 99) % 100) - distance)) / 100;
            // TODO: Explain.
            dial = (100 + dial - (distance % 100)) % 100;
        } else {
            // TODO: Explain.
            password += (dial + distance) / 100;
            // TODO: Explain.
            dial = (100 - (100 - dial) + (distance % 100)) % 100;
        };
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_example() {
        assert_eq!(
            part_two("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n"),
            6
        );
    }

    #[test]
    fn part_two_rotate_by_1_000() {
        assert_eq!(part_two("R1000"), 10);
    }

    #[test]
    fn part_two_from_zero() {
        assert_eq!(part_two("L50\nR18\nR82\nL15\n"), 2);
    }

    #[test]
    fn part_two_subtract_from_zero() {
        assert_eq!(part_two("L50\nL15\n"), 1);
    }

    #[test]
    fn part_two_ten_random_lines() {
        // 50 + 80 = 130 -> 30 (Password: 1 (+1))
        // 30 - 53 = -23 -> 77 (Password: 2 (+1))
        // 77 - 92 = -15 -> 85 (Password: 3 (+1))
        // 85 - 17 = 68
        // 68 - 77 = -9 -> 91 (Password: 4 (+1))
        // 91 + 77 = 168 -> 68 (Password: 5 (+1))
        // 68 + 848 = 916 -> 16 (Password: 14 (+9))
        // 16 + 97 = 113 -> 13 (Password: 15 (+1))
        // 13 + 55 = 68
        // 68 + 22 = 90
        // 90 + 29 = 119 -> 19 (Password: 16 (+1))

        assert_eq!(
            part_two("R80\nL53\nL92\nL17\nL77\nR77\nR848\nR97\nR55\nR22\nR29\n"),
            16
        );
    }

    // TODO: Write tests for these cases:

    // Dial @ 10, Line L40 -> New Dial @ 70, Password += 1
    // Dial @ 50, Line L5 -> New Dial @ 45, Password += 0
    // Dial @ 50, Line L500 -> New Dial @ 50, Password += 4
    // Dial @ 0, Line L15 -> New Dial @ 85, Password += 0

    // Dial @ 10, Line R40 -> New Dial @ 50, Password += 0
    // Dial @ 50, Line R5 -> New Dial @ 55, Password += 0
    // Dial @ 99, Line R21 -> New Dial @ 20, Password += 1
    // Dial @ 99, Line R1 -> New Dial @ 0, Password += 1
    // Dial @ 50, Line R10 -> New Dial @ 60, Password += 0
}

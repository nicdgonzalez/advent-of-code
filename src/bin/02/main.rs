//! Solution for Advent of Code 2025, day 2.

fn main() {
    let input = include_str!("./input.txt");
    // println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[must_use]
fn part_one(input: &str) -> u64 {
    let entries = input
        .trim()
        .split(",")
        .map(|entry| entry.split_once('-').unwrap())
        .collect::<Vec<_>>();

    let mut total = 0;

    for entry in entries {
        let (start_str, stop_str) = match (
            entry.0.len().is_multiple_of(2),
            entry.1.len().is_multiple_of(2),
        ) {
            (true, false) => {
                let mut stop = entry.1.chars().take(1).collect::<String>();

                for _ in 0..entry.1.len() - 1 {
                    stop.push('0');
                }

                (entry.0.to_owned(), stop)
            }
            (false, true) => {
                let mut start = "1".to_owned();

                for _ in 0..entry.0.len() {
                    start.push('0');
                }

                (start, entry.1.to_owned())
            }
            (true, true) => (entry.0.to_owned(), entry.1.to_owned()),
            (false, false) => continue, // Entry cannot be symmetrical.
        };

        println!("start: {start_str:?}, stop: {stop_str:?}");

        let start_mid = start_str.len().div_ceil(2);
        let start_half = &start_str[0..start_mid];
        let start = start_half.parse::<u64>().unwrap();

        let stop_mid = stop_str.len().div_ceil(2);
        let stop_half = &stop_str[0..stop_mid];
        let stop = stop_half.parse::<u64>().unwrap();

        for value in start..=stop {
            let target = (value * u64::pow(10, u32::try_from(start_mid).unwrap())) + value;
            let x = start_str.parse::<u64>().unwrap();
            let y = stop_str.parse::<u64>().unwrap();

            if target >= x && target <= y {
                println!("match found: {target}");
                total += target;
            } else {
                println!("out of range: {target}");
            }
        }

        println!("---");
    }

    total
}

#[must_use]
fn part_two(input: &str) -> i64 {
    let total = input
        .trim()
        .split(',')
        .map(|entry| entry.split_once('-').unwrap())
        .flat_map(|(start, stop)| {
            let start = start.parse::<i64>().unwrap();
            let stop = stop.parse::<i64>().unwrap();
            println!("---");
            println!("start: {start}, stop: {stop}");
            start..=stop
        })
        .filter(|&n| {
            if n < 10 {
                return false;
            }

            let length = n.to_string().len();
            let half = length.div_ceil(2);

            let pattern_lengths = (1..=half)
                .filter(|&d| length.is_multiple_of(d))
                .collect::<Vec<_>>();

            // # Examples
            //
            // 22 % 11 == 0
            // 2424 % 101 == 0
            // 292929 % 111 == 0
            //
            // # Formula
            //
            //          Index   Length of pattern
            //              V   V
            // N % ((10 ** (0 * 2)) + (10 ** (1 * 2)) + (10 ** (2 * 2)))
            //     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            //     Number of times the pattern appears (digits / length_of_pattern)
            //
            for pattern_length in pattern_lengths.into_iter().rev() {
                let mut v = 0;

                for i in 0..length.div_ceil(pattern_length) {
                    v += 10_u32.pow(u32::try_from(i * pattern_length).unwrap());
                }

                if u64::try_from(n).unwrap().is_multiple_of(u64::from(v)) {
                    println!("match found: {n}");
                    return true;
                }
            }

            false
        })
        .sum();

    println!("---");
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

    #[test]
    fn part_two_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part_two(input), 4174379265);
    }
}

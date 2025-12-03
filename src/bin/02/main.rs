//! Solution for Advent of Code 2025, day 2.

fn main() {
    let input = include_str!("./input.txt");
    // println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[must_use]
fn part_one(input: &str) -> i64 {
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
        let start = start_half.parse::<i64>().unwrap();

        let stop_mid = stop_str.len().div_ceil(2);
        let stop_half = &stop_str[0..stop_mid];
        let stop = stop_half.parse::<i64>().unwrap();

        for value in start..=stop {
            let target = (value * i64::pow(10, u32::try_from(start_mid).unwrap())) + value;
            let x = start_str.parse::<i64>().unwrap();
            let y = stop_str.parse::<i64>().unwrap();

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
    let entries = input
        .trim()
        .split(",")
        .map(|entry| entry.split_once('-').unwrap())
        .collect::<Vec<_>>();

    let mut total = 0;

    // for (start_str, stop_str) in entries.into_iter().take(3) {
    //     match (
    //         start_str.len().is_multiple_of(2),
    //         stop_str.len().is_multiple_of(2),
    //     ) {
    //         (true, true) => {
    //             let start_half = start_str.len().div_ceil(2);
    //             let digits = start_str.as_bytes();

    //             let mut pattern_lengths = vec![1];
    //             pattern_lengths.extend(
    //                 (2..=start_half)
    //                     .skip_while(|&d| digits[d - 1] == digits[0])
    //                     .filter(|&d| start_str.len().is_multiple_of(d)),
    //             );

    //             let x = start_str.parse::<i64>().unwrap();
    //             let y = stop_str.parse::<i64>().unwrap();

    //             println!("start: {start_str:?}, stop: {stop_str:?}");

    //             for length in pattern_lengths {
    //                 // 1188511880-1188511890

    //                 // TODO: (LOGIC BUG) We know start and stop are both even-length bounds,
    //                 // but we are assuming they are the same length here.
    //                 //
    //                 // Luckily, this dataset doesn't have ranges that large.
    //                 let start = start_str[0..length].parse::<i64>().unwrap();
    //                 let stop = stop_str[0..length].parse::<i64>().unwrap();

    //                 for value in start..=stop {
    //                     let mut target = value;

    //                     for _ in 1..start_str.len().div_euclid(length) {
    //                         target *= 10_i64.pow(u32::try_from(length).unwrap());
    //                         target += value;
    //                     }

    //                     if target >= x && target <= y {
    //                         println!("match found: {target}");
    //                         total += target;
    //                     } else {
    //                         println!("out of range: {target}");
    //                     }
    //                 }
    //             }

    //             println!("---");
    //         }
    //         (true, false) => {
    //             // n = 2, 4, 6, 8, ...to half of length, until start of stop.
    //             //
    //             // Then, n = 1, 3, 5, ...to half of length, until end of stop.
    //         }
    //         (false, true) => {
    //             // n = 1, 3, 5, ...to half of length (where n < half), until start of stop.
    //             //
    //             // Then, n = 2, 4, 6, ...to half of length, until end of stop.
    //         }
    //         (false, false) => {
    //             // n = 1, 3, 5, ...to half of length (where n < half), until end of stop.
    //         }
    //     }
    // }

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
        assert_eq!(part_one(input), 4174379265);
    }

    // Cut the string bounds into potential patterns, and add them to total if they fall within
    // the range.
    #[test]
    fn bound_even() {
        let s = "1188511880";
        let half = s.len().div_euclid(2);
        assert_eq!(half, 5);

        // All numerical digits are 1 byte long.
        let digits = s.as_bytes();

        // Collect potential patterns, filtering out any that would create duplicate targets.
        let mut lengths = vec![1];
        lengths.extend(
            (2..=half)
                // Skip repeating digits since e.g., '1' and '11' result in duplicate targets.
                //
                // Subtract 1 from `d` to convert it from a length to an index.
                .skip_while(|&d| digits[d - 1] == digits[0])
                // For even-length bounds, only evenly-divisible lengths can create valid patterns.
                .filter(|&d| s.len().is_multiple_of(d)),
        );
        assert_eq!(lengths, vec![1, 5]);

        // 100 -> 2500 [100, 1000)
        // 111, 222, 333, 444, 555, 666, 777, 888, 999

        let expected_slices = &["1", "11885"];
        let expected_targets: &[i64] = &[1111111111, 1188511885];

        for (i, length) in lengths.into_iter().enumerate() {
            let slice = &s[0..length];
            assert_eq!(slice, expected_slices[i]);

            let value = slice.parse::<i64>().unwrap();
            let mut target = value;

            for _ in 1..s.len().div_euclid(length) {
                target *= 10_i64.pow(u32::try_from(length).unwrap());
                target += value;
            }

            assert_eq!(target, expected_targets[i]);
        }
    }
}

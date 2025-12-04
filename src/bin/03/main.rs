//! Solution for Advent of Code 2025, day 3.

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[must_use]
fn part_one(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            println!("Bank: {line}");
            let bytes = line.as_bytes();

            let mut first: u8 = 0;
            let mut second: u8 = bytes[bytes.len() - 1] - b'0';

            let bank = bytes
                .iter()
                .map(|&b| b - b'0')
                .rev()
                .skip(1)
                .collect::<Vec<_>>();

            for n in bank {
                if second == 0 {
                    second = n;
                } else if n >= first {
                    if first > second {
                        second = first;
                        first = n;
                    } else {
                        first = n;
                    }
                }

                if first == 9 && second == 9 {
                    break;
                }
            }

            dbg!(first, second);
            let joltage = i64::from((first * 10) + second);
            dbg!(joltage);
            println!("---");

            joltage
        })
        .sum()
}

#[must_use]
fn part_two(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
        987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111\n\
        ";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(EXAMPLE_INPUT), 357);
    }

    #[test]
    fn _99() {
        let input = "3336723439433762342243436534344473653364441424424224434443333444453443346344444339414354343544443444";
        assert_eq!(part_one(input), 99);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two(EXAMPLE_INPUT), 3121910778619);
    }
}

//! Solution for Advent of Code 2025, day 3.

use std::io::Write;

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
                if n >= first {
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
    input
        .lines()
        .map(|line| {
            println!("Bank: {line}");

            let digits = line
                .as_bytes()
                .iter()
                .map(|&byte| byte - b'0')
                .rev()
                .collect::<Vec<_>>();

            let mut bank = digits.iter();

            let mut biggest = Vec::<u8>::with_capacity(12);
            // Fill now to avoid having to conditionally fill it inside of the main loop.
            biggest.extend(bank.by_ref().take(12));
            println!("Initial biggest: {:?}", biggest.iter().rev().collect::<Vec<_>>());
            println!("Remaining bank: {:?}", bank);

            // 4342633549426242625533823432244459548433412443246235533216334436553544934221624474453562462242374424
            //                                                                                 ^     ^  ^ ^^^^^^^^^
            for value in bank {
                println!("Bank value: {value}");

                let i = biggest.len() - 1;

                    let mut hold = None::<u8>;

                    let existing = biggest.get_mut(i).unwrap();

                    if value >= existing {
                        // Save the existing value.
                        hold = Some(*existing);

                        println!("Replacing {:?} with {:?} ({i})", existing, value);
                        println!("Holding: {:?}", hold);
                        *existing = *value;
                    }

                    println!("Biggest: {:?}", biggest.iter().rev().collect::<Vec<_>>());

                    let mut j = i - 1;

                    while let Some(held) = hold
                        && j < biggest.len()
                    {
                        let next = biggest.get_mut(j).unwrap();
                        j -= 1;

                        // Check if the next digit is smaller than our saved value.
                        if held >= *next {
                            println!("Replacing {:?} with {:?} ({j})", next, held);
                            hold = Some(*next);
                            println!("Holding: {:?}", hold);
                            *next = held;
                        } else {
                            // Discard the held value since it is smaller than the next value.
                            println!("Discarding held value: {} < {}", held, next);
                            hold = None;
                            println!("Vec: {:?}", biggest);
                        }
                    }

            }

            let mut total = 0i64;
            println!("Final biggest: {:?}", biggest);

            for digit in biggest.into_iter().rev() {
                total *= 10;
                total += i64::from(digit);
            }

            dbg!(&line, total);
            println!("---");
            total
        })
        .sum()
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

// Kids, always clarify your requirements... :^)
//
// #[must_use]
// fn part_two(input: &str) -> i64 {
//     input
//         .lines()
//         .map(|line| {
//             let bank = line
//                 .as_bytes()
//                 .iter()
//                 .map(|&byte| byte - b'0')
//                 .collect::<Vec<_>>();
//
//             let mut heap = BinaryHeap::<Reverse<u8>>::with_capacity(12);
//
//             for value in bank {
//                 if heap.len() < 12 {
//                     heap.push(Reverse(value));
//                 } else if let Some(&Reverse(smallest)) = heap.peek() && value > smallest {
//                     _ = heap.pop();
//                     heap.push(Reverse(value));
//                 }
//             }
//
//             dbg!(line, &heap);
//
//             let mut total = 0i64;
//
//             while let Some(Reverse(value)) = heap.pop() {
//                 total += i64::from(value);
//             }
//
//             total
//         })
//         .sum()
// }

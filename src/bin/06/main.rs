//! Solution for Advent of Code 2025, day 6.

fn main() {
    let input = include_str!("./input.txt");

    let solution_one = part_one(input);
    let solution_two = part_two(input);

    println!("Part 1: {}", solution_one);
    println!("Part 2: {}", solution_two);
}

#[must_use]
fn part_one(input: &str) -> u64 {
    let mut entries = input
        .lines()
        .map(|line| line.split_whitespace())
        .collect::<Vec<_>>();

    let operators = entries
        .pop()
        .expect("expected non-empty entries")
        .collect::<Vec<_>>();

    let entries = entries
        .into_iter()
        .map(|entry| entry.map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // NOTE: We are assuming that all rows have the same number of columns.
    let columns = entries.first().unwrap().len();
    let mut total = 0;

    for i in 0..columns {
        let operator = operators[i].trim().as_bytes()[0];
        let mut result = match operator {
            b'+' => 0,
            b'*' => 1, // Cannot start at 0 or the result will always be 0.
            _ => unreachable!(),
        };

        for row in &entries {
            assert_eq!(columns, row.len());
            let d = row[i];
            match operator {
                b'+' => result += d,
                b'*' => result *= d,
                _ => unreachable!(),
            }
        }

        total += result;
    }

    total
}

#[must_use]
fn part_two(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
        123 328  51 64\n\
        45 64  387 23\n\
        6 98  215 314\n\
        *   +   *   +\
        ";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(EXAMPLE_INPUT), 4_277_556);
    }

    // #[test]
    // fn part_two_example() {
    //     assert_eq!(part_two(EXAMPLE_INPUT), 0);
    // }
}

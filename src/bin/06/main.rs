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
    let mut entries = input.lines().collect::<Vec<_>>();
    let last = entries
        .pop()
        .expect("expected non-empty entries")
        .as_bytes();
    let operators = last
        .split(u8::is_ascii_whitespace)
        .flat_map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    // All of the operators are left aligned, so we can use that to find the length of each column.
    let mut column_widths = Vec::<usize>::with_capacity(operators.len());

    let mut column_width = 1;
    for &c in last.iter().skip(1) {
        if c == b' ' {
            column_width += 1;
        } else {
            column_widths.push(column_width - 1); // Subtract the single space separator.
            column_width = 1; // Reset counter to prepare for counting the next column.
        }
    }
    // The last column was never flushed since it didn't hit another operator; flush manually.
    column_widths.push(column_width);
    assert_eq!(column_widths.len(), operators.len());
    dbg!(&column_widths);

    let mut total = 0u64;
    let mut offset = 0;

    for (i, width) in column_widths.into_iter().enumerate() {
        dbg!(width);
        let operator = operators[i];
        let mut result = 0u64;
        if matches!(operator, b'*') {
            // Anything multiplied by zero is always zero; start from one instead.
            result += 1;
        }

        for j in (0..width).rev() {
            let mut value = 0u64;

            for row in entries.iter().map(|row| row.as_bytes()) {
                dbg!(offset, j);
                let b = row[offset + j];

                if b == b' ' {
                    continue;
                }

                let d = b - b'0';
                dbg!(d);

                value *= 10;
                value += u64::from(d);
            }
            dbg!(value);

            match operator {
                b'+' => result += value,
                b'*' => result *= value,
                _ => unreachable!(),
            }
        }

        offset += width + 1;
        total += result;
    }

    total
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

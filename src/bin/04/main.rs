//! Solution for Advent of Code 2025, day 4.

fn main() {
    let input = include_str!("./input.txt");

    let solution_one = part_one(input);
    let solution_two = part_two(input);

    println!("Part 1: {}", solution_one);
    println!("Part 2: {}", solution_two);
}

#[must_use]
fn part_one(input: &str) -> i64 {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut total = 0;

    let max_rows = grid.len();

    for (dy, row) in grid.iter().enumerate() {
        let dy = isize::try_from(dy).unwrap();

        let max_columns = row.len();

        for (dx, value) in row.iter().enumerate() {
            if value != &b'@' {
                print!(".");
                continue;
            }

            let dx = isize::try_from(dx).unwrap();

            let mut adjacent_rolls_of_paper = 0;

            let targets = [
                (dy - 1, dx - 1),
                (dy - 1, dx),
                (dy - 1, dx + 1),
                (dy, dx - 1),
                (dy, dx + 1),
                (dy + 1, dx - 1),
                (dy + 1, dx),
                (dy + 1, dx + 1),
            ];

            for (y, x) in targets {
                // Ensure the Y coordinate is not out of bounds.
                if y < 0 || y >= isize::try_from(max_rows).unwrap() {
                    continue;
                }

                // Ensure the X coordinate is not out of bounds.
                if x < 0 || x >= isize::try_from(max_columns).unwrap() {
                    continue;
                }

                let x = usize::try_from(x).unwrap();
                let y = usize::try_from(y).unwrap();

                if grid[y][x] == b'@' {
                    adjacent_rolls_of_paper += 1;
                }
            }

            if adjacent_rolls_of_paper < 4 {
                print!("x");
                total += 1;
            } else {
                print!("@");
            }
        }

        #[expect(clippy::print_with_newline)]
        {
            print!("\n");
        }
    }

    total
}

#[must_use]
fn part_two(input: &str) -> i64 {
    let mut grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut total = 0;

    let max_rows = grid.len();

    loop {
        let total_copy = total;

        for (dy, row) in grid.clone().iter_mut().enumerate() {
            let dy = isize::try_from(dy).unwrap();

            let max_columns = row.len();

            for (dx, value) in row.iter_mut().enumerate() {
                if value != &b'@' {
                    print!(".");
                    continue;
                }

                let dx = isize::try_from(dx).unwrap();

                let mut adjacent_rolls_of_paper = 0;

                let targets = [
                    (dy - 1, dx - 1),
                    (dy - 1, dx),
                    (dy - 1, dx + 1),
                    (dy, dx - 1),
                    (dy, dx + 1),
                    (dy + 1, dx - 1),
                    (dy + 1, dx),
                    (dy + 1, dx + 1),
                ];

                for (y, x) in targets {
                    // Ensure the Y coordinate is not out of bounds.
                    if y < 0 || y >= isize::try_from(max_rows).unwrap() {
                        continue;
                    }

                    // Ensure the X coordinate is not out of bounds.
                    if x < 0 || x >= isize::try_from(max_columns).unwrap() {
                        continue;
                    }

                    let x = usize::try_from(x).unwrap();
                    let y = usize::try_from(y).unwrap();

                    if grid[y][x] == b'@' {
                        adjacent_rolls_of_paper += 1;
                    }
                }

                if adjacent_rolls_of_paper < 4 {
                    print!("x");
                    total += 1;
                    grid[usize::try_from(dy).unwrap()][usize::try_from(dx).unwrap()] = b'.';
                } else {
                    print!("@");
                }
            }

            #[expect(clippy::print_with_newline)]
            {
                print!("\n");
            }
        }

        #[expect(clippy::print_with_newline)]
        {
            print!("\n");
        }

        if total == total_copy {
            break;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.\
    ";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(EXAMPLE_INPUT), 13);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two(EXAMPLE_INPUT), 43);
    }
}

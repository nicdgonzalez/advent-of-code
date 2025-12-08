//! Solution for Advent of Code 2025, day 7.

use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    let solution_one = part_one(input);
    let solution_two = part_two(input);

    println!("Part 1: {}", solution_one);
    println!("Part 2: {}", solution_two);
}

#[must_use]
fn part_one(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let data = &lines[1..lines.len()];

    let mut beams = HashSet::<usize>::with_capacity(data[0].len());
    beams.insert(lines[0].find('S').unwrap());
    // dbg!(&beams);

    let mut total_splits = 0;

    let mut to_add = Vec::<usize>::with_capacity(data[0].len());
    let mut to_remove = Vec::<usize>::with_capacity(data[0].len().div_ceil(2));

    for i in 1..data.len().div_ceil(2) {
        let row = data[i * 2 - 1].as_bytes();

        for beam in &beams {
            let column = row[*beam];

            if column != b'^' {
                continue;
            }

            total_splits += 1;
            // println!(
            //     "Splitting beam @ {} into {} and {} (Splits: {})",
            //     beam,
            //     beam - 1,
            //     beam + 1,
            //     total_splits,
            // );

            // Because of Rust's ownership rules, we cannot change `beams` while iterating through
            // a nested loop, so we collect our desired changes and apply them after.
            to_remove.push(*beam);
            to_add.extend([beam - 1, beam + 1]);
        }

        for j in &to_remove {
            beams.remove(j);
        }
        to_remove.clear();

        beams.extend(&to_add);
        to_add.clear();
        // println!("Beams: {:?}", beams);
    }

    u64::try_from(total_splits).unwrap()
}

#[must_use]
fn part_two(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two(EXAMPLE_INPUT), 0);
    }
}

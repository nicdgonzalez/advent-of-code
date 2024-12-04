//! Solution for Advent of Code 2023, day .
//! https://adventofcode.com/2024/day/
//!
//! Attemped on: 2024-12-04

use std::{collections, fs, path};

/// Each line of data looks something like this:
///
///     Game 1: 7 blue, 8 red; 1 blue, 2 red; 1 green, 2 blue
///
/// Calculate the sum of IDs for games that would be possible if the bag was
/// loaded with only 12 red cubes, 13 green cubes, 14 blue cubes.
#[must_use]
fn part_one(input: &str) -> i32 {
    let limits =
        collections::HashMap::<&str, i32>::from([("red", 12), ("green", 13), ("blue", 14)]);

    input
        .lines()
        .map(|line| {
            let (id, games) = line.split_once(": ").unwrap();
            let id: i32 = id.chars().skip(5).collect::<String>().parse().unwrap();
            let games: Vec<&str> = games.split("; ").collect();

            for game in games.iter() {
                let cubes: Vec<&str> = game.split(", ").collect();

                for cube in cubes.iter() {
                    let (count, color) = cube.split_once(' ').unwrap();
                    let count: i32 = count.parse().unwrap();

                    if count > *limits.get(color).unwrap() {
                        return 0;
                    }
                }
            }

            id
        })
        .sum()
}

/// Using the same data as before, find the minimum set of cubes that must be
/// present, multiply them together and sum the total.
#[must_use]
fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (_, games) = line.split_once(": ").unwrap();
            let games: Vec<&str> = games.split("; ").collect();
            let mut colors =
                collections::HashMap::<&str, i32>::from([("red", 0), ("green", 0), ("blue", 0)]);

            for game in games.iter() {
                let cubes: Vec<&str> = game.split(", ").collect();

                for cube in cubes.iter() {
                    let (count, color) = cube.split_once(' ').unwrap();
                    let count: i32 = count.parse().unwrap();

                    if count > *colors.get(color).unwrap() {
                        let _ = colors.insert(color, count);
                    }
                }
            }

            let red = colors.get("red").unwrap();
            let green = colors.get("green").unwrap();
            let blue = colors.get("blue").unwrap();

            red * blue * green
        })
        .sum()
}

fn main() {
    let current_dir = path::Path::new(file!()).parent().unwrap();
    let input_path = current_dir.join("input.txt");
    let input = fs::read_to_string(input_path).expect("failed to read input");

    println!("part one: {}", part_one(&input));
    println!("part two: {}", part_two(&input));
}

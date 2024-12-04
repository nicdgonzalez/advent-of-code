#!/usr/bin/bash

TEMPLATE="\
//! Solution for Advent of Code 2023, day $DAY.
//! https://adventofcode.com/2024/day/$DAY
//!
//! Attemped on: $(date -Idate)

use std::{fs, path};

/// A brief description about part one.
#[must_use]
fn part_one(input: &str) -> i32 {
    todo!()
}

/// A brief description about part two.
#[must_use]
fn part_two(input: &str) -> i32 {
    todo!()
}

fn main() {
    let current_dir = path::Path::new(file!()).parent().unwrap();
    let input_path = current_dir.join(\"input.txt\");
    let input = fs::read_to_string(input_path).expect(\"failed to read input\");

    println!(\"part one: {}\", part_one(&input));
    println!(\"part two: {}\", part_two(&input));
}"

main() {
    declare DAY="$1"

    if [[ ! $DAY ]]; then
        echo >&2 "usage: $(basename -- "$0") <DAY>"
        echo >&2 "<DAY> is a number between 1 and 25 (inclusive)"
        exit 1
    fi

    if [[ ! -e "$PWD/src/bin" ]]; then
        mkdir "$PWD/src/bin"
    fi

    DAY="$(printf "%02d" $DAY)"

    mkdir "$PWD/src/bin/$DAY"
    echo "$TEMPLATE" > "$PWD/src/bin/$DAY/main.rs"
    aoc download --year 2023 --day "$DAY" --input-file "./src/bin/$DAY/input.txt"
}

main "$@"

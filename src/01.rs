use std::collections;
use std::fs;
use std::process;

fn convert_char_to_digit(c: &char) -> Option<u32> {
    if let Some(digit) = c.to_digit(10) {
        return Some(digit);
    }

    None
}

fn convert_string_to_digit(string: &str) -> Option<u8> {
    let number_map = collections::HashMap::<&str, u8>::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for (&word, &number) in &number_map {
        if string.starts_with(word) {
            return Some(number);
        }
    }

    None
}

fn get_value(
    line: &str,
    iter: impl Iterator<Item = usize>,
    allow_word_form: bool,
) -> Option<u32> {
    for i in iter {
        let string = &line[i..];
        let char = &string.chars().next().unwrap();

        if let Some(digit) = convert_char_to_digit(&char) {
            return Some(digit);
        }

        if !allow_word_form {
            continue;
        }

        if let Some(digit) = convert_string_to_digit(string) {
            return Some(digit as u32);
        }
    }

    None
}

/// Get a collection of the calibration values from each line of the input.
///
/// # Arguments
///
/// * `data` - A string containing the input to evaluate.
/// * `allow_word_form` - Whether to enable part two of the challenge
///   and use the word forms of each number as well.
fn get_calibration_values(data: &str, allow_word_form: bool) -> Vec<u32> {
    data.lines()
        .map(|line| {
            let first = get_value(&line, 0..line.len(), allow_word_form)
                .expect("Every line should have at least one digit");

            let last =
                get_value(&line, (0..line.len()).rev(), allow_word_form)
                    .unwrap_or(first);

            (first * 10) + last
        })
        .collect()
}

/// The main entry point to the program.
fn main() {
    let file_path: String = String::from("./input/01.txt");

    let data = match fs::read_to_string(&file_path) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to read file at {file_path:?}: {e}");
            process::exit(1);
        }
    };

    let part_one: u32 = get_calibration_values(&data, false).iter().sum();
    println!("Part 1: {part_one}");

    let part_two: u32 = get_calibration_values(&data, true).iter().sum();
    println!("Part 2: {part_two}");
}

use std::collections;
use std::fs;

fn valid_round(cubes: &Vec<&str>) -> bool {
    let color_map = collections::HashMap::<&str, u32>::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let small_max = color_map.values().min().unwrap();

    for cube in cubes {
        let (count, color) = match cube.split_once(" ") {
            Some((count, color)) => {
                let cnt = count
                    .parse::<u32>()
                    .expect("Count should be on the left side of color");

                (cnt, color)
            }
            None => panic!("Data should look like: <count> <color>"),
        };

        if count > *small_max && count > *color_map.get(&color).unwrap() {
            return false;
        }
    }

    true
}

fn valid_game(raw_rounds: &str) -> bool {
    let rounds: Vec<&str> = raw_rounds.split(";").map(|r| r.trim()).collect();

    for round in &rounds {
        let cubes: Vec<&str> = round.split(",").map(|c| c.trim()).collect();

        if !valid_round(&cubes) {
            return false;
        }
    }

    true
}

fn get_valid_game_ids(data: &str) -> Vec<u32> {
    data.lines()
        .filter_map(|line| {
            let (game, rounds) = &line
                .split_once(":")
                .expect("Every line should have a Game ID");

            let game_id = match &game.split_once(" ") {
                Some((_, id)) => id.parse::<u32>().unwrap(),
                _ => panic!("Every line should have a Game ID"),
            };

            if valid_game(&rounds) {
                return Some(game_id);
            }

            None
        })
        .collect()
}

/// The main entry point to the program.
fn main() {
    let file_path: String = String::from("./input/02.txt");
    let data = fs::read_to_string(&file_path).expect("Failed to read input");

    let part_one: u32 = get_valid_game_ids(&data).iter().sum();
    println!("Part 1: {part_one}");

    let part_two: u32 = 0;
    println!("Part 2: {part_two}");
}

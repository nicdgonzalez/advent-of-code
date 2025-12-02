#[macro_export]
macro_rules! advent_of_code {
    () => {
        fn main() {
            let input = include_str!("./input.txt");
            println!("Part 1: {}", part_one(input));
            println!("Part 2: {}", part_two(input));
        }
    };
}

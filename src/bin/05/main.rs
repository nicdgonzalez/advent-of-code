//! Solution for Advent of Code 2025, day 5.

fn main() {
    let input = include_str!("./input.txt");

    let solution_one = part_one(input);
    let solution_two = part_two(input);

    println!("Part 1: {}", solution_one);
    println!("Part 2: {}", solution_two);
}

#[must_use]
fn part_one(input: &str) -> usize {
    let ranges = input
        .lines()
        .take_while(|&line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .map(|(start, stop)| {
            let start = start.parse::<i64>().unwrap();
            let stop = stop.parse::<i64>().unwrap();
            (start, stop)
        })
        .collect::<Vec<_>>();

    input
        .lines()
        .skip_while(|&line| !line.is_empty())
        .skip(1) // Also skip the empty line.
        .map(|line| line.parse::<i64>().unwrap())
        .filter(|&id| {
            ranges
                .iter()
                .any(|&(start, stop)| (start..=stop).contains(&id))
        })
        .count()
}

#[must_use]
fn part_two(input: &str) -> u64 {
    let mut ranges = input
        .lines()
        .take_while(|&line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .map(|(start, stop)| {
            let start = start.parse::<u64>().unwrap();
            let stop = stop.parse::<u64>().unwrap();
            (start, stop)
        })
        .collect::<Vec<_>>();

    ranges.sort_by_key(|(start, _)| *start);

    let mut previous_max = 0;

    ranges
        .iter()
        .map(|(start, end)| {
            // Because our ranges are sorted, we can take the greater of the two: previous end or
            // current range's start and subtract it from end.
            // [5, 12][9, 20]
            //     ^^^^^
            // 20-5=15, but 16 numbers exist between 5 and 20. Add +1 to make the end inclusive.
            let (start, end) = (*start.max(&previous_max), end + 1);
            previous_max = previous_max.max(end);

            end.saturating_sub(start)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
        3-5\n\
        10-14\n\
        16-20\n\
        12-18\n\
        \n\
        1\n\
        5\n\
        8\n\
        11\n\
        17\n\
        32\
    ";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two(EXAMPLE_INPUT), 14);
    }

    #[test]
    fn part_two_simplified() {
        assert_eq!(
            part_two(
                "\
                    1-2\n\
                    10-20\n\
                    3-12\n\
                    5-25\
                "
            ),
            25
        );
    }

    #[test]
    fn part_two_ltlt() {
        let input = "\
            2-6\n\
            4-8\n\
        ";
        assert_eq!(part_two(input), 7);
    }

    #[test]
    fn part_two_ltgt() {
        let input = "\
            20-80\n\
            40-60\n\
        ";
        assert_eq!(part_two(input), 61);
    }

    #[test]
    fn part_two_lteq() {
        let input = "\
            200-800\n\
            400-800\n\
        ";
        assert_eq!(part_two(input), 601);
    }

    #[test]
    fn part_two_gtlt() {
        let input = "\
            40-60\n\
            20-80\n\
        ";
        assert_eq!(part_two(input), 61);
    }

    #[test]
    fn part_two_gtgt() {
        let input = "\
            400-800\n\
            200-600\n\
        ";
        assert_eq!(part_two(input), 601);
    }

    #[test]
    fn part_two_gteq() {
        let input = "\
            2000-8000\n\
            4000-8000\n\
        ";
        assert_eq!(part_two(input), 6001);
    }

    #[test]
    fn part_two_eqlt() {
        let input = "\
            200-600\n\
            200-800\n\
        ";
        assert_eq!(part_two(input), 601);
    }

    #[test]
    fn part_two_eqgt() {
        let input = "\
            20-80\n\
            40-60\n\
        ";
        assert_eq!(part_two(input), 61);
    }

    #[test]
    fn part_two_eqeq() {
        let input = "\
            2000-6000\n\
            4000-8000\n\
        ";
        assert_eq!(part_two(input), 6001);
    }
}

"""
Solution for Advent of Code 2024, day 2.
https://adventofcode.com/2024/day/2

Attempted on: 2024-12-20
"""

import pathlib


def part_one(data: str) -> int:
    return sum(map(is_safe, data.splitlines()))


def is_safe(line: str) -> bool:
    numbers = [int(n) for n in line.split()]

    # All levels must follow the same direction.
    # If [0] < [1], then [1] must be less than [2], and so on.
    increasing = numbers[0] < numbers[1]

    return (
        all(
            (
                number < numbers[i + 1]
                if increasing
                else number > numbers[i + 1]
            )
            and (1 <= abs(number - numbers[i + 1]) <= 3)
            for i, number in enumerate(numbers)
        )
        if numbers[0] != numbers[1]
        else False
    )


def valid_level_increasing(a: int, b: int, /) -> bool:
    return a < b and (1 <= abs(a - b) <= 3)


def valid_level_decreasing(a: int, b: int, /) -> bool:
    return a > b and (1 <= abs(a - b) <= 3)


# ---


def valid_level(a: int, b: int, /, *, increasing: bool) -> bool:
    return ((a < b) == increasing) and (1 <= abs(a - b) <= 3)


def part_two(data: str) -> int:
    raise NotImplementedError


if __name__ == "__main__":
    data = (
        pathlib.Path(__file__)
        .resolve()
        .parent.joinpath("input.txt")
        .read_text()
    )
    print("part one:", part_one(data=data))
    print("part two:", part_two(data=data))

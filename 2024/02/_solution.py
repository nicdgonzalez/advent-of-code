"""
Solution for Advent of Code 2024, day 2.
https://adventofcode.com/2024/day/2

Attempted on: 2024-12-20
"""

import pathlib


def part_one(data: str) -> int:
    total = 0

    for line in data.splitlines():
        numbers = [int(n) for n in line.split()]
        assert len(numbers) > 1, len(numbers)

        # All levels must follow the same direction.
        # If [0] < [1], then [1] must be less than [2], and so on.
        increasing = numbers[0] < numbers[1]

        for i, number in enumerate(numbers):
            try:
                next_number = numbers[i + 1]
            except IndexError:
                continue

            if not valid_level(number, next_number, increasing=increasing):
                break
            else:
                pass
        else:
            total += 1

    return total


def valid_level(a: int, b: int, /, *, increasing: bool) -> bool:
    return ((a < b) == increasing) and (1 <= abs(a - b) <= 3)


def part_two(data: str) -> int:
    raise NotImplementedError


if __name__ == "__main__":
    current_dir = pathlib.Path(__file__).resolve().parent
    data = current_dir.joinpath("input.txt").read_text()
    print("part one:", part_one(data=data))
    print("part two:", part_two(data=data))

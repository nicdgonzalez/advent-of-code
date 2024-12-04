"""
Solution for Advent of Code 2024, day 1.
https://adventofcode.com/2024/day/1

Attempted on: 2024-12-2
"""

import pathlib


def part_one(data: str) -> int:
    """Pair up the numbers of each list in ascending order, then calculate
    the distance between each pair.

    Returns
    -------
    int:
        The sum of all distances.
    """
    left, right = split_data_into_two_lists(data=data)

    # Sort both lists in ascending order.
    left.sort()
    right.sort()

    return sum([abs(x - y) for x, y in zip(left, right)])


def part_two(data: str) -> int:
    """Multiply each number of the left list by the number of times it appears
    in the right list, then sum the values to get a total similarity score.

    Returns
    -------
    int:
        The sum of all similarity scores.
    """
    left, right = split_data_into_two_lists(data=data)
    return sum([x * right.count(x) for x in left])


def split_data_into_two_lists(data: str) -> tuple[list[int], list[int]]:
    # Each line of data looks something like this:
    #
    #   12345   67890
    #   32983   12903
    #   23904   39028
    #   [...]
    #
    # Split the data into two lists; one for each column.
    columns = left, right = ([], [])
    data_valid = filter(lambda line: line.strip() != "", data.split("\n"))

    for line in data_valid:
        x, y = line.strip().split()

        assert x.isdigit(), x
        left.append(int(x))

        assert y.isdigit(), y
        right.append(int(y))

    return columns


if __name__ == "__main__":
    current_dir = pathlib.Path(__file__).resolve().parent
    data = current_dir.joinpath("input.txt").read_text()
    print("part one:", part_one(data=data))
    print("part two:", part_two(data=data))

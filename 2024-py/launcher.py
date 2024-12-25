#!/usr/bin/python

import argparse
import datetime as dt
import importlib
import pathlib


def main() -> None:
    args = get_args()

    match args.command:
        case "run":
            run_handler(day=args.day)
        case "scaffold":
            scaffold_handler(day=args.day)


def get_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()

    parser.add_argument(
        "command",
        type=str,
        help="",
        choices=["run", "scaffold"],
        metavar="<COMMAND>",
    )

    parser.add_argument(
        "day",
        type=int,
        help="A day between 1 and 25 to run the solutions for.",
        choices=range(1, 26),
        metavar="<DAY>",
    )

    return parser.parse_args()


def get_package_from_day(day: int) -> str:
    assert day in range(1, 26), day
    # Pad with an extra 0 if value is less than 10.
    return "{:0>2}".format(day)


def run_handler(day: int) -> None:
    package = get_package_from_day(day=day)
    solution = importlib.import_module(".solution", package=package)

    root = pathlib.Path(__file__).resolve().parent
    input_txt = root.joinpath(package, "input.txt")
    data = input_txt.read_text()

    result = solution.part_one(data=data)
    print("part one:", result)

    result = solution.part_two(data=data)
    print("part two:", result)


def scaffold_handler(day: int) -> None:
    package = get_package_from_day(day=day)
    current_dir = pathlib.Path(__file__).resolve().parent
    package_path = current_dir.joinpath(package)

    # Create a new directory for the target day.
    if not package_path.exists():
        package_path.mkdir(parents=True)

    # Create an empty input file.
    input_txt = package_path.joinpath("input.txt")

    if not input_txt.exists():
        input_txt.write_text(
            "Overwrite the contents of this file with the puzzle input."
        )

    # Create a template solution file.
    solution_py = package_path.joinpath("solution.py")
    template = current_dir.joinpath("template.txt").read_text()
    today = dt.datetime.now().strftime("%Y-%m-%d")
    solution_py.write_text(template.format(day=day, today=today))


if __name__ == "__main__":
    main()

# Advent of Code 2024

This year, I will be solving the puzzles using Python.

## Solutions

Assuming the following:

```
. <-- Your terminal is here.
├── 2023
├── 2024
│   ├── 01
│   │   └── solution.py
│   ├── README.md  <-- You are reading here.
│   └── launcher.py
└── README.md
```

The launcher expects your terminal to be inside of the "2024" directory too, so
change directory like so:

```bash
cd ./2024
```

Now, you can use `launcher.py` to run any of the solutions.

```bash
# To run the solutions for day 1:
python launcher.py run 01
```

If you are solving puzzles, you can also use `launcher.py` to set up your daily
challenges as well.

```bash
# To create the "02" directory and expected files:
python launcher.py scaffold 02
```

### Run solutions manually

If you don't want to use `launcher.py` to run the files, copy the following to
the end of the target `solution.py` file.

```python
if __name__ == "__main__":
    input_txt = pathlib.Path(__file__).parent.joinpath("input.txt")
    data = input_txt.read_text()

    total_distance = part_one(data=data)
    print("part one:", total_distance)

    similarity_score = part_two(data=data)
    print("part two:", similarity_score)
```

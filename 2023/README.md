# Advent of Code 2023

This year, I will be solving the puzzles using Rust.

## Solutions

Assuming the following:

```
. <-- Your terminal is here.
├── 2023
│   ├── src
│   │   ├── bin
│   │   │   ├── 01
│   │   │   │   └── main.rs
│   │   │   └── 02
│   │   │       └── main.rs
│   │   └── lib.rs
│   ├── README.md  <-- You are reading here.
│   └── Cargo.toml
└── README.md
```

`cargo` expects your terminal to be inside of the "2023" directory too,
so change directory like so:

```bash
cd ./2023
```

Now, to run any of the solutions, do:

```bash
# To run the solutions for day 1:
cargo run --bin 01
```

If you are solving puzzles, you can also use `launcher.py` to set up your daily
challenges as well.

```bash
# To create the "02" directory and expected files:
bash ./scaffold.sh 02
```

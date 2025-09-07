[![Rust version: 1.86.*](https://img.shields.io/badge/rust%20version-1.86.*-black)](https://www.rust-lang.org)

[![qa manual:main](https://github.com/raj-open/code-challenges/actions/workflows/manual.yaml/badge.svg?branch=main)](https://github.com/raj-open/code-challenges/actions/workflows/manual.yaml)
[![qa manual:staging](https://github.com/raj-open/code-challenges/actions/workflows/manual.yaml/badge.svg?branch=staging)](https://github.com/raj-open/code-challenges/actions/workflows/manual.yaml)

[![qa auto:staging](https://github.com/raj-open/code-challenges/actions/workflows/auto.yaml/badge.svg?branch=staging)](https://github.com/raj-open/code-challenges/actions/workflows/auto.yaml)
[![qa auto:current](https://github.com/raj-open/code-challenges/actions/workflows/auto.yaml/badge.svg)](https://github.com/raj-open/code-challenges/actions/workflows/auto.yaml)

# Code Challenges #

This repository contains code snippets written entirely by the repository's owner / author,
as solutions to problems on various platforms.

We work here primarily with

- Rust
- python

## System requirements ##

- [bash](https://gitforwindows.org)
- [justfile tool](https://github.com/casey/just?tab=readme-ov-file#installation)
- [rust](https://www.rust-lang.org) incl. cargo
- python@v3.11+

> [!IMPORTANT]
> We rely on [Zig](https://ziglang.org) for cross-compilation,
> which avoids gcc-compiler issues on local machine and linux images,
> which in turn are required by the rust compiler.

> [!TIP]
> To verify, open a bash terminal and call.
>
> ```bash
> just --version
> # rust
> rustup --version
> rustc --version
> cargo --version
> # python
> . .venv/bin/activate && python3 --version # for unix
> . .venv/Scripts/activate && python --version # for windows
> # zig
> zig version
> ```

## Setup ##

Run

```bash
# only needed once
just setup
# now modify the created .env file

# only needed when code changes
just build
```

## Testing ##

### Unit tests ###

Unit tests for rust code accompany files in the same (sub)modules
e.g. `src/path/to/file.rs` has unit test `src/path/to/tests_file.rs`.
To run a unit test for a single file, call

```bash
just test-unit "path/to/file.rs"
# or
just test-unit "path/to/tests_file.rs"
```

Both options will run just the tests in the `test_`-file.

To run all unit tests throughout the repository, call

```bash
just tests-unit
```

### Integration tests ###

The `tests` folder contains integration tests for rust code,
and unit tests for python code.

## Execution of Binaries ##

This repository contains some binaries (see [Cargo.toml](./Cargo.toml)):

- `CodeChallenges`
- `HackerRankMathematics`
- `GeniusSquare`

To run a binary call

```bash
# runs with optimisation (slower compile time)
just run-rust {NAME_OF_BINARY} [flags]
# runs without optimisation (faster compile time, slower run time)
just dev-rust {NAME_OF_BINARY} [flags]
```

### Genius Squares ###

The binary `GeniusSquare` solves instances of
the _Smart Games_ puzzle [Genius Square](https://smarttoysandgames.co.uk/uk/genius-square).
Usage is as follows:

```bash
# provides random instance of the puzzle and solves it:
just run-rust GeniusSquare
# ... with random seeding for repeatability:
just run-rust GeniusSquare {Seed}
# solves an instance of the game for a particular initialisation (roll of the dice):
just run-rust GeniusSquare {Dice1} {Dice2} ... {Dice7}
```

e.g.

```bash
just run-rust GeniusSquare 1234 # with random seed
just run-rust GeniusSquare B1 C4 D6 F1 F2 F3 F5 # with given initialisation
```

The `run` command builds and runs the binary.
To perform this separately, use

```bash
just build-compile GeniusSquare
```

which copy the binary in [target/release/GeniusSquare](target/release/GeniusSquare) to the [dist](dist) folder.
The standalone binary can be called as above:

```bash
./dist/GeniusSquare
# with random seed
./dist/GeniusSquare {Seed}
./dist/GeniusSquare 1234
# with given initialisation
./dist/GeniusSquare {Dice1} {Dice2} ... {Dice7}
./dist/GeniusSquare B1 C4 D6 F1 F2 F3 F5
```

#### Note on the algorithm ####

The solver currently relies on a depth-first tree-search algorithm,
with adaptive sorting, i.e. once pieces have been placed,
the order of computation of the remaining pieces
are locally sorted (in ascending order) based on the number of next possible moves.

#### Example ####

Calling

```bash
just run-rust GeniusSquare B1 C4 D2 D6 E5 F1 F3
```

results in

```bash
Roll: B1 C4 D2 D6 E5 F1 F3.

Problem:
      ╔═══╦═══╦═══╦═══╦═══╦═══╗
      ║ A ║ B ║ C ║ D ║ E ║ F ║
      ╚═══╩═══╩═══╩═══╩═══╩═══╝
╔═══╗     ┼───┼           ┼───┼
║ 1 ║     │ ■ │           │ ■ │
╠═══╣     ┼───┼   ┼───┼   ┼───┼
║ 2 ║             │ ■ │
╠═══╣             ┼───┼   ┼───┼
║ 3 ║                     │ ■ │
╠═══╣         ┼───┼       ┼───┼
║ 4 ║         │ ■ │
╠═══╣         ┼───┼   ┼───┼
║ 5 ║                 │ ■ │
╠═══╣             ┼───┼───┼
║ 6 ║             │ ■ │
╚═══╝             ┼───┼

Compute solution ... found 33 solutions.
Time for 1st solution:      3.929ms
Average time per solution:  2.48506ms
Total time:                 82.007ms

Solution 1:
      ╔═══╦═══╦═══╦═══╦═══╦═══╗
      ║ A ║ B ║ C ║ D ║ E ║ F ║
      ╚═══╩═══╩═══╩═══╩═══╩═══╝
╔═══╗ ┼───┼───┼───┼───┼───┼───┼
║ 1 ║ │ 2 │ ■ │ Z   Z │ C │ ■ │
╠═══╣ ┼   ┼───┼   ┼───┼   ┼───┼
║ 2 ║ │ 2 │ Z   Z │ ■ │ C   C │
╠═══╣ ┼───┼───┼───┼───┼───┼───┼
║ 3 ║ │ T   T   T │ X   X │ ■ │
╠═══╣ ┼───┼   ┼───┼       ┼───┼
║ 4 ║ │ 1 │ T │ ■ │ X   X │ L │
╠═══╣ ┼───┼───┼───┼───┼───┼   ┼
║ 5 ║ │ 4   4   4   4 │ ■ │ L │
╠═══╣ ┼───┼───┼───┼───┼───┼   ┼
║ 6 ║ │ 3   3   3 │ ■ │ L   L │
╚═══╝ ┼───┼───┼───┼───┼───┼───┼
```

in the console.
The algoirithm provides solutions at the `Wizard` level,
viz. no collisions occur and none of the pieces

```text
┌───┐ ┌───┐ ┌───┐ ┌───┬───┐
│ 1 │ │ 2 │ │ 3 │ │ C   C │
└───┘ ├   ┤ ├   ┤ ├   ┼───┘
      │ 2 │ │ 3 │ │ C │
      └───┘ ├   ┤ └───┘
            │ 3 │
            └───┘
```

are adjacent (in the sense of touching edges).

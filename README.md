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

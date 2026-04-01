# Graphol — Towards a New Language

Contents

- [General Information](#general-information)
  - [Build and Run](#build-and-run)
    - [1. Build the Graphol tool itself](#1-build-the-graphol-tool-itself)
    - [2. Compile a `.graphol` program to a Linux executable (`-o` / `--output`)](#2-compile-a-graphol-program-to-a-linux-executable--o--output)
    - [3. Use a directory as input (defaults to `main.graphol`)](#3-use-a-directory-as-input-defaults-to-maingraphol)
    - [4. Output name behavior (`-o` / `--output` is optional)](#4-output-name-behavior--o--output-is-optional)
- [History Summary](#history-summary)

## General Information

Graphol is a Graph Oriented Language, with the compiler written in Rust.

### Build and Run

Use this section as a quick reference for day-to-day usage.

#### 1. Build the Graphol tool itself

This builds `graphol` (the CLI that can interpret `.graphol` files and also generate executables):

```bash
make build
```

Install the binary after build:

```bash
sudo make install
```

#### 2. Compile a `.graphol` program to a Linux executable (`-o` / `--output`)

In this mode, `graphol` generates a standalone Linux executable for the provided Graphol source:

```bash
graphol examples/program5.graphol -o program5
```

Equivalent long flag:

```bash
graphol examples/program5.graphol --output program5
```

Run the generated executable:

```bash
./program5
```

#### 3. Use a directory as input (defaults to `main.graphol`)

If the input path is a directory, `graphol` automatically uses `<directory>/main.graphol` as the entry file.

Compile from directory entry:

```bash
graphol examples -o program_from_examples
```

#### 4. Output name behavior (`-o` / `--output` is optional)

The `-o` / `--output` flag is optional:

- With `-o/--output`: compiles the `.graphol` source into a native Linux executable at the specified output path, which you run directly later.
- Without `-o/--output`: compiles the `.graphol` deriving the output name from the input and saving in the current path.

## History Summary

See the [History Summary](HISTORY_SUMMARY.md) to understand the migration from JavaScript to Rust.

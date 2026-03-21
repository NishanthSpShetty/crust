# [CRUST](https://nishanthspshetty.github.io/crust/)

[![Build Status](https://github.com/nishanthspshetty/crust/actions/workflows/crust.yml/badge.svg)](https://github.com/nishanthspshetty/crust/actions/workflows/crust.yml)
[![Coverage Status](https://img.shields.io/badge/coverage-NA-lightgrey)](https://github.com/nishanthspshetty/crust)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## C/C++ to Rust Transpiler

CRUST is a command-line transpiler that converts C/C++ syntax into Rust code.
It aims to reduce migration effort while preserving semantics with safe, auditable output.

### Highlights

- Converts declarations, assignments, and basic expressions
- Supports control flow: `if` / `if-else`, `while`, `for`, `switch`
- Handles basic struct/union/class definitions
- Preserves comments and outputs warning markers for unsupported constructs
- Strict (`-s`) vs loose mode (default) semantics for variable mutability

> Works best for small/medium C++ code slices. Complex constructs (templates, macros, function pointers) are partially supported and may require manual adjustments.

---

## Visual Preview

![CRUST Workflow Example](https://user-images.githubusercontent.com/000000/placeholder-screen.png)

> Replace with actual screenshot path or GitHub-hosted asset when available.

---

## Quick Start

### 1) Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

### 2) Clone the repository

```bash
git clone https://github.com/nishanthspshetty/crust.git
cd crust
```

### 3) Build

```bash
cargo build
```

### 4) Run

```bash
cargo run -- examples/prog.cpp
```

Interactive prompts:

- `Enter the C/C++ file to be converted to Rust : examples/prog.cpp`
- `Enter the translation mode [(S/s)trict/(L/l)oose] : l`
- `Do you want to create a cargo project :[Y/N] n`

Output is written to `examples/prog.rs` by default (or `<project>/src/main.rs` if project creation is selected).

---

## CLI Options

```bash
cargo run -- -s -p myproj examples/main.cpp
```

- `-s`, `--strict`: strict mode (immutable bindings)
- `-p`, `--project-name`: create a new Cargo project and write to `project/src/main.rs`
- `-h`, `--help`: display usage information

---

## Testing

Run the full test suite:

```bash
cargo test
```

Key test locations:

- `src/library/parser/parser_test.rs`
- `src/library/lexer/tokenizer/*`

### Recent test additions

- full expression output validation for declarations and expressions
- parser bounds-safe checks to avoid index panics
- strict/loose mode behavior tests

---

## Developer Documentation

### Architecture

- `src/main.rs`: CLI + orchestration + file I/O
- `src/library/lexer`: tokenization pipeline
- `src/library/parser`: recursive descent translation engine

### Parser flow

1. Tokenize input into `Vec<Token>`
2. `init_parser` processes with `Parser` state
3. `parse_program` dispatches to specialized handlers (control flow, types, assignments)
4. Build Rust token stream and output text

### Coding standards

- avoid `Vec` cloning; use `&[Token]` slices
- avoid `expect` in parser production paths; use `Result` with structured errors next
- enforce `cargo fmt`, `cargo clippy`

### Local dev workflow

```bash
cargo test
cargo fmt --all
cargo clippy -- -D warnings
```

---

## Supported Features

- Types: `int`, `float`, `char`, `const`, `unsigned`, pointers
- Control structures: `if`, `if-else`, `while`, `for`, `switch`
- `typedef`, `struct`, `union`, `class`, `enum`
- Comments preserved (as `//` or `/* */` blocks)

## Limitations

- templates, generic C++ constructs
- function pointers and functors
- macros with preprocessor logic
- comprehensive standard library conversion (e.g., iostream / std::vector)

---

## Release Notes (latest)

- fixed Cargo metadata (`exclude` in `[package]`)
- enabled robust parser bounds safety
- added full-expression tests for parser outputs
- expanded README with user + developer details, badges, screenshot placeholder

---

## Contributing

1. Fork repository
2. branch: `feature/<name>`
3. Add/update tests in `src/library/parser/parser_test.rs`
4. `cargo test` and `cargo fmt`
5. PR with summary of behavior regressions/improvements

---

## License

MIT

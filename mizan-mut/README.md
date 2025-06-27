# `mizan-mut`

Rust code mutation and refactoring tools that provide:

1. Semantic-preserving transformations to Rust source code
2. Symbol renaming using rust-analyzer

## Installation

> Requires nightly Rust toolchain. Either install it via `rustup` or use the `+nightly` flag with `cargo`

```bash
cargo install --path mizan-mut
# Or build directly
cargo build --release --bin mizan-mut
```

## Usage

### Mutate Subcommand

Apply semantic-preserving mutations to Rust source code:

```bash
mizan-mut mutate -r <ROOT_DIR> -m <MUTATION>...
```

Examples:

```bash
# Apply a single mutation
mizan-mut mutate -r ./my-crate -m for-to-while

# Apply all mutations at once
mizan-mut mutate -r ./my-crate -m all

# See all available mutations
mizan-mut mutate --help
```

#### Available Mutations

| Mutation              | Description                                         |
| --------------------- | --------------------------------------------------- |
| `all`                 | Applies all available mutations                     |
| `for-to-while`        | Converts `for` loops to `while` loops               |
| `while-to-loop`       | Converts `while` loops to `loop` blocks with breaks |
| `if-else-reorder`     | Reorders if-else branches by negating conditions    |
| `derive-reorder`      | Randomly reorders traits in derive attributes       |
| `trait-bound-reorder` | Randomly reorders trait bounds in where clauses     |
| `use-reorder`         | Randomly reorders items in use statements           |
| `arithmetic-identity` | Adds arithmetic identity operations (x + N - N)     |

#### Limitations

- `for-to-while`: Only handles simple patterns
- `while-to-loop`: Does not transform `while let` patterns
- `if-else-reorder`: Only transforms if statements with else branches

### Rename Subcommand

Rename any symbol in Rust codebases using rust-analyzer:

```bash
cargo +nightly run --release -- rename -c <PATH> --file <FILE> --offset <OFFSET> --new-name <NAME>
```

#### Arguments

- `-c, --crate-root <PATH>`: Path to the crate root (directory containing Cargo.toml)
- `-f, --file <FILE>`: Path to the file containing the symbol (relative to crate root)
- `-o, --offset <OFFSET>`: Byte offset of the symbol in the file
- `-n, --new-name <NAME>`: New name for the symbol

#### Examples

Rename a function `process_data` to `handle_data` in `src/main.rs` at byte offset 70:

```bash
cargo +nightly run --release -- rename -c examples/test_project -f src/main.rs -o 70 -n handle_data
```

#### Finding Byte Offsets

To find the byte offset of a symbol, you can use:

```bash
grep -b -o "process_data" examples/test_project/src/main.rs
```

> That's zero-based, so add one to the result to get the correct offset for the command

## Important Notes

- The mutate subcommand modifies files in-place
- All mutated code is automatically formatted with `rustfmt` after mutations are applied
- Code comments are lost during mutation

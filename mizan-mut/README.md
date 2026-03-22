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

| Mutation                | Description                                         |
| ----------------------- | --------------------------------------------------- |
| `all`                   | Applies all available mutations                     |
| `for-to-while`          | Converts `for` loops to `while` loops               |
| `while-to-loop`         | Converts `while` loops to `loop` blocks with breaks |
| `if-else-reorder`       | Reorders if-else branches by negating conditions    |
| `derive-reorder`        | Randomly reorders traits in derive attributes       |
| `trait-bound-reorder`   | Randomly reorders trait bounds in where clauses     |
| `use-reorder`           | Randomly reorders items in use statements           |
| `arithmetic-identity`   | Wraps integer literals with multiplication identity (N * 1) |
| `extraneous-unsafe`     | Adds extraneous `unsafe {...}` blocks around statements inside functions |
| `impl-trait-to-generic` | Converts impl form Trait bounds into generic parameters |
| `option-wrap`           | Wraps expressions in redundant `Some(...).unwrap()` calls |
| `maybeuninit-wrap`      | Wraps known safe values into a `MaybeUninit<T>`, automatically dererencing them |
| `manuallydrop-wrap`     | Places owned variables into `ManuallyDrop` structs, and later unwraps them |
| `explicit-return`     | Converts implicit return statements to use explicit syntax |
| `unreachable-panic`   | Adds an unreachable panic!() to function bodies     |
| `repeated-shadowing`  | Adds multiple redundant repeated shadows for let bindings within a scope |

#### Limitations

- `for-to-while`: Only handles simple patterns
- `while-to-loop`: Does not transform `while let` patterns
- `if-else-reorder`: Only transforms if statements with else branches
<<<<<<< HEAD
<<<<<<< HEAD
- `manuallydrop-wrap`: Immediately unwraps after initial `let` statement
=======
- `explicit-return`: Only applies at the function level
>>>>>>> origin/mut-explicit-return
=======
- `repeated-shadowing`: Only adds shadows directly after the initial binding
>>>>>>> origin/mut-repeated-shadowing

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

## Testing Mutations

A Docker-based test suite validates that all mutations are semantic-preserving by applying them to real-world Rust crates and verifying their test suites still pass.

```bash
# Build the test image
docker build -f docker/Dockerfile.mutations-test -t mizan-mut-test .

# Run — outputs a markdown report
docker run mizan-mut-test
```

The test runs each mutation against [itertools](https://github.com/rust-itertools/itertools), [num-traits](https://github.com/rust-num/num-traits), [num-bigint](https://github.com/rust-num/num-bigint), and [byteorder](https://github.com/BurntSushi/byteorder).

If you add a new mutation, add it to the `MUTATIONS` array in `docker/Dockerfile.mutations-test` and run the suite to verify it passes.

## Important Notes

- The mutate subcommand modifies files in-place
- All mutated code is automatically formatted with `rustfmt` after mutations are applied
- Code comments are lost during mutation

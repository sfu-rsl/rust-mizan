# mizan-mut

`mizan-mut` is the Rust tool behind the AST-based mutations and the rename mutations. It provides:

1. Semantic-preserving AST transformations of Rust source.
2. Symbol renaming via `rust-analyzer`.

The `mizan mutate` CLI calls this binary for any `mizan-mut-*` mutation and for all rename mutations, so it must be installed and on your `PATH`.

## Installation

`mizan-mut` depends on `rust-analyzer` crates that require nightly.

```bash
cargo install --path mizan-mut
# Or build directly
cargo build --release --bin mizan-mut
```

## `mutate` subcommand

Apply AST mutations to a crate in place.

```bash
mizan-mut mutate -r <ROOT_DIR> -m <MUTATION>... [-i <FILE_TO_IGNORE>...]
```

| Argument | Short | Description |
| --- | --- | --- |
| `--root` | `-r` | Root directory of the crate to mutate |
| `--mutations` | `-m` | Mutations to apply (repeatable) |
| `--ignore` | `-i` | File paths to skip (repeatable) |

```bash
mizan-mut mutate -r ./my-crate -m for-to-while
mizan-mut mutate -r ./my-crate -m all
mizan-mut mutate --help          # list all mutations
```

### Available mutations

| Mutation | Description |
| --- | --- |
| `all` | Apply all available mutations |
| `for-to-while` | Convert `for` loops to `while` loops |
| `while-to-loop` | Convert `while` loops to `loop` blocks with breaks |
| `if-else-reorder` | Reorder if-else branches by negating conditions |
| `derive-reorder` | Reorder traits in `#[derive(...)]` attributes |
| `trait-bound-reorder` | Reorder trait bounds in `where` clauses |
| `use-reorder` | Reorder items in `use` statements |
| `arithmetic-identity` | Wrap integer literals with a multiplication identity (`N * 1`) |
| `explicit-where` | Add an explicit `where` clause to a signature |
| `explicit-where-to-type-params` | Move simple type bounds from a `where` clause into the type params |
| `rename-lifetime` | Rename lifetime parameters consistently |
| `impl-trait-to-generic` | Convert `impl Trait` bounds into generic parameters |
| `option-wrap` | Wrap expressions in a redundant `Some(...).unwrap()` |
| `maybeuninit-wrap` | Round-trip a value through `MaybeUninit<T>` |
| `manuallydrop-wrap` | Wrap an owned variable in `ManuallyDrop`, then unwrap it |
| `explicit-return` | Convert implicit returns to explicit `return` statements |
| `unreachable-panic` | Guard a function body with an unreachable `panic!()` arm |
| `repeated-shadowing` | Add redundant repeated shadows for `let` bindings |

See [Mutation specification](specification.md) for the before/after form of each.

#### Limitations

- `for-to-while`: handles simple patterns only.
- `while-to-loop`: does not transform `while let`.
- `if-else-reorder`: only transforms `if` statements that have an `else`.
- `manuallydrop-wrap`: unwraps immediately after the initial `let`.
- `explicit-return`: applies at the function level only.
- `repeated-shadowing`: adds shadows directly after the initial binding only.
- `explicit-where`: incompatible with `explicit-where-to-type-params`.
- `rename-lifetime`: applies to standalone functions only.

## `rename` subcommand

Rename any symbol and update all references across the crate.

```bash
mizan-mut rename -c <CRATE_ROOT> -f <FILE> -o <OFFSET> -n <NEW_NAME>
```

| Argument | Short | Description |
| --- | --- | --- |
| `--crate-root` | `-c` | Crate root (directory containing `Cargo.toml`) |
| `--file` | `-f` | File containing the symbol, relative to the crate root |
| `--offset` | `-o` | Byte offset of the symbol (zero-based) |
| `--new-name` | `-n` | New name |

```bash
mizan-mut rename -c examples/test_project -f src/main.rs -o 70 -n handle_data
```

To find a byte offset, use `grep -b -o "name" path/to/file.rs` (the result is zero-based).

## Testing mutations

A Docker-based suite checks that mutations are semantic-preserving by applying them to real crates ([itertools](https://github.com/rust-itertools/itertools), [num-traits](https://github.com/rust-num/num-traits), [num-bigint](https://github.com/rust-num/num-bigint), [byteorder](https://github.com/BurntSushi/byteorder)) and verifying their test suites still pass.

```bash
docker build -f docker/Dockerfile.mutations-test -t mizan-mut-test .
docker run mizan-mut-test
```

If you add a mutation, add it to the `MUTATIONS` array in `docker/Dockerfile.mutations-test` and run the suite. See [Add a mutation](../contributing/mutations.md).

## Notes

- The `mutate` subcommand modifies files in place.
- Mutated code is reformatted with `rustfmt` afterward.
- Comments are lost during mutation, since the code is parsed to an AST and regenerated. This is why AST mutations use content-based ground-truth tracking (see the [Mutations](index.md) overview).

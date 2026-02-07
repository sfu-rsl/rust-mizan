# Mizan CLI

A CLI for interacting with RustMizan dataset

## Installation

```bash
cd mizan-cli
poetry install

# Run mizan commands with poetry run
poetry run mizan checkout --help

# Add mizan to PATH (add to ~/.zshrc or ~/.bashrc)
export PATH="$(poetry env info --path)/bin:$PATH"
```

## Prerequisites

All commands must be run from a directory containing `mizan.json`. This is the root directory of the dataset.

## Configuration

Mizan CLI supports configuration through a JSON file at `~/.config/mizan/config.json`.

### Available Configuration Options

| Option      | Description                                 | Default |
| ----------- | ------------------------------------------- | ------- |
| `log_level` | Logging level (DEBUG, INFO, WARNING, ERROR) | `INFO`  |
| `log_file`  | Path to log file                            | None    |

### Example Configuration File

Create `~/.config/mizan/config.json`:

```json
{
  "log_level": "INFO",
  "log_file": "/var/log/mizan.log"
}
```

## Commands

### `checkout`

allows you to select and export specific code samples from the dataset based on various filters.

```bash
mizan checkout [OPTIONS]
```

#### Options

| Option            | Short | Description                                                | Default    |
| ----------------- | ----- | ---------------------------------------------------------- | ---------- |
| `--output`        | `-o`  | Output directory for checked out samples                   | `./output` |
| `--level`         | `-l`  | Code granularity level: `function`, `file`, `crate`, `all` | `all`      |
| `--vuln-ids`      | `-v`  | Specific vulnerability IDs (can be used multiple times)    | None       |
| `--year`          | `-y`  | Filter vulnerabilities by year                             | None       |
| `--cwe-types`     | `-c`  | Filter by CWE types (can be used multiple times)           | None       |
| `--include-fixed` |       | Include fixed samples along with vulnerable ones           | False      |

#### Examples

```bash
# Checkout all function-level samples
mizan checkout --level function

# Checkout specific vulnerabilities
mizan checkout --vuln-ids vuln-0001 --vuln-ids vuln-0002

# Checkout all samples from 2019
mizan checkout --year 2019

# Checkout samples with specific CWE types
mizan checkout --cwe-types CWE-416 --cwe-types CWE-125

# Checkout both vulnerable and fixed samples
mizan checkout --include-fixed

# Combine multiple filters
mizan checkout --level function --year 2019 --cwe-types CWE-416 -o ./my-samples
```

### `mutate` - Apply Code Mutations

Apply semantic-preserving mutations to checked out samples. This command should be run from within the output directory created by `checkout`.

```bash
cd ./output
mizan mutate [OPTIONS]
```

#### Options

| Option        | Short | Description                                          | Default |
| ------------- | ----- | ---------------------------------------------------- | ------- |
| `--mutations` | `-m`  | Mutations to apply (can be specified multiple times) | `all`   |

#### Available Mutations

- Comment Removal:
  - `remove-comments`: Remove all Rust comments while preserving code functionality
- Formatting Mutations:
  - `format-compact`: Apply `rustfmt` formatting to shrink code
  - `format-expanded`: Apply `rustfmt` formatting to expand code (vertical whitespacing)
- Insertion Mutations:
  - `benign-comments`: Insert benign comments around vulnerable lines
  - `benign-blocks`: Insert benign code blocks around vulnerable lines
  - `malignant-comments`: Insert malignant comments around vulnerable lines
  - `malignant-blocks`: Insert malignant code blocks around vulnerable lines
- AST-based Mutations (via `mizan-mut`):
  - `mizan-mut-for-to-while`: Converts `for` loops to `while` loops
  - `mizan-mut-while-to-loop`: Converts `while` loops to `loop` blocks with breaks
  - `mizan-mut-if-else-reorder`: Reorders if-else branches by negating conditions
  - `mizan-mut-derive-reorder`: Randomly reorders traits in derive attributes
  - `mizan-mut-trait-bound-reorder`: Randomly reorders trait bounds in where clauses
  - `mizan-mut-use-reorder`: Randomly reorders items in use statements
  - `mizan-mut-arithmetic-identity`: Adds arithmetic identity operations (x + N - N)
  - `mizan-mut-all`: Applies all mizan-mut mutations at once
- Rename Mutations (via `mizan-mut rename`):
  - `benign-rename-fn`: Renames functions to neutral names (e.g., `fn_1_abc123`)
  - `benign-rename-var`: Renames variables to neutral names (e.g., `var_1_xyz789`)
  - `malignant-rename-fn`: Renames functions to names suggesting safety (e.g., `safe_fn_1`, `verified_fn_2`)
  - `malignant-rename-var`: Renames variables to names suggesting safety (e.g., `secure_var_1`, `checked_var_2`)

> Note: The AST-based and rename mutations require [`mizan-mut`](../mizan-mut/) to be installed and available in your PATH

#### Mutation Order Considerations

Be thoughtful about the order of mutations you specify:

- Don't apply `for-to-while` followed by `while-to-loop` unless you specifically want to convert for loops to loop blocks
- Don't apply `benign-comments` followed by `remove-comments` as the inserted comments will be removed
- Consider the cumulative effects when chaining multiple mutations

> Note on "Successful" Mutations: A "successful" mutation means the process completed without errors, not necessarily that code was changed. For example, applying `for-to-while` to code without any for loops will succeed without making changes.

#### Examples

```bash
# Apply all mutations sequentially
mizan mutate --mutations all

# Apply single mutation
mizan mutate --mutations format-compact

# Apply multiple mutations
mizan mutate --mutations format-compact --mutations format-expanded

```

#### Output

- Updated `mizan.json`: Ground truth with corrected line numbers after mutations. We track vulnerable lines by inserting unique code comments (e.g., `// VULN_LINE_MARKER_123`) before each vulnerable line which allows us to maintain accurate line number tracking even after code transformations
- `mizan_mutations.json`: Metadata about applied mutations including:
  - `mutations_applied`: List of successfully applied mutations
  - `skipped`: Mutations that were skipped completely or specific samples that were skipped
  - `partial_mutations`: List of samples where mutations were applied to other files except the one with vulnerable lines - This only happens for AST-based mutations where some files had to be excluded

#### Mutation Strategy for AST-based Mutations

The AST-based mutations (`mizan-mut-*`) use a different ground truth tracking approach compared to other mutations:

Warning: `mizan-mut` removes all code comments (including our line markers) because it uses `syn` and `quote` crates to parse and reconstruct the AST. After reconstruction, the code is reformatted before writing back to file.

1. Content-based Line Tracking: Due to comment removal during AST manipulation, we cannot rely on line markers alone. Instead, we track vulnerable lines by their content.

2. Handling Tracking Issues: When applying AST mutations:
   - If a vulnerable line appears multiple times in a file, the mutation is re-applied with that file excluded from mutation
   - If a vulnerable line cannot be found after mutation (e.g., the mutation modified the line), the file is excluded and mutation is re-applied
   - Excluded files retain their original vulnerable line numbers in the ground truth

3. Partial Mutation Tracking:
   - When a mutation is applied to other files except the one with vulnerable lines (some files had to be excluded), this is recorded in `mizan_mutations.json` under `partial_mutations`

#### Mutation Strategy for Rename Mutations

The rename mutations use `mizan-mut rename` to rename variables and functions around vulnerable lines:

1. Scope of Renaming: The mutations identify and rename variables (`let` bindings) and function declarations within 10 lines before and after each vulnerable line.

2. Naming Strategies:
   - `benign-rename-fn`: Renames functions to neutral names like `fn_1_abc123` or `fn_2_xyz789`
   - `benign-rename-var`: Renames variables to neutral names like `var_1_def456` or `var_2_ghi012`
   - `malignant-rename-fn`: Renames functions to names falsely suggesting safety like `safe_fn_1`, `verified_fn_2`
   - `malignant-rename-var`: Renames variables to names falsely suggesting safety like `secure_var_1`, `checked_var_2`

3. Ground Truth Handling: Unlike other mutations, rename mutations legitimately change line content. The validation system recognizes this and allows line content differences for rename mutations.

4. Identifier Exclusions: Common identifiers like `self` are excluded from renaming to avoid breaking code structure.

5. The mutations use [rust-analyzer's `rename`](https://rust-analyzer.github.io/book/features.html#rename) feature to make sure all references to the renamed identifiers are updated across the codebase so that the code remains compilable and functional.

### `evaluate` - Run LLM Evaluation

#### `prepare-dataset`

Converts checked out samples to parquet format for evaluation.

```bash
cd my-output
mizan evaluate prepare-dataset --tag baseline  # --tag is optional
```

**Options:**

| Option     | Short | Description                                      | Default           |
| ---------- | ----- | ------------------------------------------------ | ----------------- |
| `--output` | `-o`  | Output dataset file                              | `dataset.parquet` |
| `--tag`    | `-t`  | Optional tag to identify dataset (e.g. baseline) | `None`            |

**Output files:**

- `dataset.parquet`: Parquet file with code samples and ground truth
- `mutations_metadata.json`: Applied mutations metadata

#### Running Evaluations

Use the [`run_eval.py`](run_eval.py) script to run evaluations with full control over parameters:

```bash
cd mizan-cli
# Edit run_eval.py to configure different evaluation parameters including agent, models, dataset path, max turns, etc.
python run_eval.py
```

See [Inspect AI documentation](https://inspect.aisi.org.uk) to learn more about supported models and evaluation options.

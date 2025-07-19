# Mizan CLI

A CLI for interacting with RustMizan dataset

## Install in Development Mode

```bash
poetry build

# Install in editable mode (changes to source code are reflected immediately)
pip install -e .

# Or with poetry
poetry install

# Add poetry's virtualenv to PATH
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

> Note: The AST-based and rename mutations require `mizan-mut` to be installed and available in your PATH

#### Mutation Order Considerations

Be thoughtful about the order of mutations you specify:

- Don't apply `for-to-while` followed by `while-to-loop` unless you specifically want to convert for loops to loop blocks
- Don't apply `benign-comments` followed by `remove-comments` as the inserted comments will be removed
- Consider the cumulative effects when chaining multiple mutations

#### Understanding "Successful" Mutations

A "successful" mutation means the process completed without errors, not necessarily that code was changed. For example:

- Applying `for-to-while` to code without any for loops will succeed without making changes

Always review the modified files to see what changes were actually made

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
  - `failures`: Mutations that failed completely or specific samples that failed
  - `partial_applications`: List of samples where mutations were partially applied (some files excluded to keep vulnerable lines intact)

#### Mutation Strategy for AST-based Mutations

The AST-based mutations (`mizan-mut-*`) use a different ground truth tracking approach compared to other mutations:

Warning: `mizan-mut` removes all code comments (including our line markers) because it uses `syn` and `quote` crates to parse and reconstruct the AST. After reconstruction, the code is reformatted before writing back to file.

1. Content-based Line Tracking: Due to comment removal during AST manipulation, we cannot rely on line markers alone. Instead, we track vulnerable lines by their content.

2. Handling Tracking Issues: When applying AST mutations:

   - If a vulnerable line appears multiple times in a file, the mutation is re-applied with that file excluded from mutation
   - If a vulnerable line cannot be found after mutation (e.g., the mutation modified the line), the file is excluded and mutation is re-applied
   - Excluded files retain their original vulnerable line numbers in the ground truth

3. Partial Application Tracking:
   - When a mutation cannot be fully applied to a sample (some files had to be excluded), this is recorded in `mizan_mutations.json` under `partial_applications`

#### Mutation Strategy for Rename Mutations

The rename mutations use `mizan-mut rename` to rename variables and functions around vulnerable lines:

1. Scope of Renaming: The mutations identify and rename variables (`let` bindings) and function declarations within 10 lines before and after each vulnerable line.

2. Naming Strategies:

   - `benign-rename-fn`: Renames functions to neutral names like `fn_1_abc123` or `fn_2_xyz789`
   - `benign-rename-var`: Renames variables to neutral names like `var_1_def456` or `var_2_ghi012`
   - `malignant-rename-fn`: Renames functions to names falsely suggesting safety like `safe_fn_1`, `verified_fn_2`
   - `malignant-rename-var`: Renames variables to names falsely suggesting safety like `secure_var_1`, `checked_var_2`

3. The mutations use rust-analyzer's `rename` feature to make sure all references to the renamed identifiers are updated across the codebase so that the code remains compilable and functional.

4. Common identifiers like `self`, `main`, and trait method names (e.g., `from`, `clone`) are excluded from renaming to avoid breaking the code structure.

### `evaluate` - Run LLM Evaluation

Evaluate LLMs on vulnerability detection tasks using prepared datasets. Includes two subcommands for dataset preparation and evaluation execution.

> Note: We use [LangSmith](https://docs.smith.langchain.com) for evaluation orchestration. All evaluation runs locally.

#### Subcommands

##### `prepare-dataset` - Prepare Evaluation Dataset

Converts the checked out samples and ground truth data into a format suitable for LLM evaluation.

```bash
mizan evaluate prepare-dataset [OPTIONS]
```

Options:

| Option     | Short | Description                          | Default                     |
| ---------- | ----- | ------------------------------------ | --------------------------- |
| `--output` | `-o`  | Output path for the prepared dataset | `./evaluation_dataset.json` |

Example:

```bash
# Prepare dataset with default output
mizan evaluate prepare-dataset

# Prepare dataset with custom output path
mizan evaluate prepare-dataset -o ./my_evaluation_dataset.json
```

Output Files:

- `evaluation_dataset.json`: Contains the prepared dataset with examples and mutations metadata
- Each example includes:
  - `inputs`: Only the prompt (including the code sample)
  - `outputs`: Ground truth data (is_vulnerable, cwe_type, vulnerable_functions, vulnerable_lines)
  - `metadata`: Sample metadata (id, vuln_id, granularity)

##### `run` - Execute LLM Evaluation

Runs the LLM evaluation on a prepared dataset using the specified model and provider.

```bash
mizan evaluate run [OPTIONS]
```

Options:

| Option       | Short | Description                                    | Default |
| ------------ | ----- | ---------------------------------------------- | ------- |
| `--dataset`  | `-d`  | Path to prepared dataset file (required)       | None    |
| `--provider` | `-p`  | LLM provider: `openai`, `anthropic` (required) | None    |
| `--model`    | `-m`  | Model name (required)                          | None    |

> Temperature is fixed at `0.0` for all evaluations.

Example:

```bash
mizan evaluate run -d ./evaluation_dataset.json -p anthropic -m claude-3-7-sonnet-20250219
```

Output Files:
The evaluation creates a timestamped experiment directory under `./evaluation_results/` containing:

- `results.json`: Detailed per-sample results including scores, comments, and errors
- `metadata.json`: Experiment metadata including summary evaluations and mutations metadata
- `results.csv`: CSV export for data analysis

#### Evaluators

##### Per-Sample Evaluators

1. Binary Classification (`is_vulnerable_accuracy`)

   - Evaluates whether the LLM correctly identifies if code is vulnerable or not
   - Treated as binary classification with simple accuracy calculation
   - Score: 1 for correct classification, 0 for incorrect
   - Metric: Accuracy

2. CWE Type Detection

   - Precision (`cwe_type_precision`): Proportion of predicted CWE types that are correct
   - Recall (`cwe_type_recall`): Proportion of actual CWE types that are identified
   - F1 Score (`cwe_type_f1`): Harmonic mean of precision and recall
   - Uses set-based comparison treating predicted and expected CWE types as sets
   - Mathematical formulation:
     - TP = |predicted ∩ expected|
     - FP = |predicted - expected|
     - FN = |expected - predicted|
     - Precision = TP / (TP + FP)
     - Recall = TP / (TP + FN)
     - F1 = 2 × (precision × recall) / (precision + recall)

3. Vulnerable Functions Detection

   - Precision (`vulnerable_functions_precision`): Proportion of predicted functions that are correct
   - Recall (`vulnerable_functions_recall`): Proportion of actual vulnerable functions that are identified
   - F1 Score (`vulnerable_functions_f1`): Harmonic mean of precision and recall
   - Uses tuple-based comparison with (file_path, function_name) for unique identification
   - Mathematical formulation: Same as CWEs

4. Vulnerable Lines Detection

   - F1 Score (`vulnerable_lines_f1`): Harmonic mean of precision and recall
   - Precision (`vulnerable_lines_precision`): Proportion of predicted lines that are correct
   - Recall (`vulnerable_lines_recall`): Proportion of actual vulnerable lines that are identified
   - Uses tuple-based comparison with (file_path, line_number) for unique identification
   - Mathematical formulation: Same as CWEs

5. JSON Validity (`json_validity`)
   - Evaluates whether the LLM response contains valid JSON with expected fields
   - Score: 1 for valid JSON, 0 for invalid
   - Expected fields: `is_vulnerable`, `cwe_type`, `vulnerable_functions`, `vulnerable_lines`

##### Experiment-Level Summary Evaluators

> If the LLM failed to produce a valid JSON response (e.g., due to an error or timeout), the sample is marked as failed and will not be included in the evaluation metrics

1. Binary Classification Metrics (`is_vulnerable_*`)

   - Accuracy (`is_vulnerable_accuracy`): Overall accuracy across all samples
   - Precision (`is_vulnerable_precision`): Precision for vulnerability detection
   - Recall (`is_vulnerable_recall`): Recall for vulnerability detection
   - F1 Score (`is_vulnerable_f1`): F1 score for vulnerability detection
   - Mathematical formulation:
     - TP = samples where predicted=True AND expected=True
     - TN = samples where predicted=False AND expected=False
     - FP = samples where predicted=True AND expected=False
     - FN = samples where predicted=False AND expected=True
     - Accuracy = (TP + TN) / (TP + TN + FP + FN)
     - Precision = TP / (TP + FP), with edge case handling
     - Recall = TP / (TP + FN), with edge case handling
     - F1 = 2 × (precision × recall) / (precision + recall)

2. Set-Based Metrics with Micro and Macro Averaging

   For each field (`cwe_type`, `vulnerable_functions`, `vulnerable_lines`), we calculate precision, recall, and F1:

   Micro-Averaged Metrics:

   - F1 Scores: `cwe_type_micro_f1`, `vulnerable_functions_micro_f1`, `vulnerable_lines_micro_f1`
   - Precision: `cwe_type_micro_precision`, `vulnerable_functions_micro_precision`, `vulnerable_lines_micro_precision`
   - Recall: `cwe_type_micro_recall`, `vulnerable_functions_micro_recall`, `vulnerable_lines_micro_recall`
   - Calculation: Aggregate TP, FP, FN counts across all samples, then calculate metrics
   - Mathematical formulation:
     - TP_total = Σ(TP_i) across all samples i
     - FP_total = Σ(FP_i) across all samples i
     - FN_total = Σ(FN_i) across all samples i
     - Micro_Precision = TP_total / (TP_total + FP_total)
     - Micro_Recall = TP_total / (TP_total + FN_total)
     - Micro_F1 = 2 × (Micro_Precision × Micro_Recall) / (Micro_Precision + Micro_Recall)

   Macro-Averaged Metrics:

   - F1 Scores: `cwe_type_macro_f1`, `vulnerable_functions_macro_f1`, `vulnerable_lines_macro_f1`
   - Precision: `cwe_type_macro_precision`, `vulnerable_functions_macro_precision`, `vulnerable_lines_macro_precision`
   - Recall: `cwe_type_macro_recall`, `vulnerable_functions_macro_recall`, `vulnerable_lines_macro_recall`
   - Calculation: Calculate per-sample metrics, then average them
   - Mathematical formulation:
     - Precision_i = per-sample precision for sample i
     - Recall_i = per-sample recall for sample i
     - F1_i = per-sample F1 score for sample i
     - Macro_Precision = (1/n) × Σ(Precision_i) across all samples i
     - Macro_Recall = (1/n) × Σ(Recall_i) across all samples i
     - Macro_F1 = (1/n) × Σ(F1_i) across all samples i

3. JSON Validity Rate (`json_validity_rate`)
   - Percentage of samples with valid JSON responses
   - Calculation: (valid responses) / (total responses)

> Note: For more information on how to interpret micro and macro metrics, see [Scikit-learn's documentation on micro and macro averaging](https://sklearn-evaluation.ploomber.io/en/latest/classification/micro_macro.html).

## End-to-End Example

Evaluating GPT-4 on function-level samples for vulnerability `vuln-0001` with multiple mutations:

```bash
# Checkout specific vulnerability at function level
mizan checkout -v vuln-0001 -l function -o my-output

# Enter output directory and apply mutations
cd my-output
mizan mutate -m remove-comments -m format-compact -m benign-rename-fn

# Prepare dataset and run evaluation
mizan evaluate prepare-dataset -o vuln-0001-mutated.json
mizan evaluate run -d vuln-0001-mutated.json -p openai -m gpt-4
```

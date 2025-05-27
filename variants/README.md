# Dataset Variants

This directory contains different flavours of `rust-mizan`. These variants are used to study the LLM training data contamination problem and evaluate the effectiveness of various mitigation strategies.

Each folder here is a complete Rust workspace with a corresponding `mizan.json` file containing the ground truth for that particular variant.

## [18-vulns](18-vulns)

This folder includes a subset of 18 vulnerabilities directly copied from the original `rust-mizan` dataset.

The only difference is that each vulnerability in this set comes from a unique Rust crate.

## [fmt-unconventional](fmt-unconventional)

This variant applies an unconventional Rust formatting style to the entire workspace.

Some examples of formatting changes include:

- Using tab characters for indentation with 13-space tab width
- Forcing vertical brace style for all functions and control blocks
- Limiting the line width to 60 characters and breaking long expressions

There are some limitations. For example, formatting can't be safely applied to certain `use` statements with `#[cfg]` attributes like:

```rust
#[cfg(target_env = "gnu")]
use libc::{lseek64, ENXIO, SEEK_DATA, SEEK_HOLE};
```

These are left unchanged to avoid breaking conditional compilation.

The ground truth in `mizan.json` was updated to match the new formatting. The update process is handled by the script at `variants/fmt-unconventional/update_ground_truth.py`, which does the following:

- Adds `#[rustfmt::skip]` before each vulnerable function to prevent formatting from affecting it
- Adds markers for each vulnerable line
- Applies the custom formatting to the whole workspace
- Updates the `mizan.json` file to reflect new line numbers
- Cleans up by removing the markers and skip attributes

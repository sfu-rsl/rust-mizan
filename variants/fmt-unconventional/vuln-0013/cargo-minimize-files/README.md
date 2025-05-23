# `cargo-minimize` usage

The crate samples were reduced with a forked version of a cargo project to minimize the code to some extent:
https://github.com/Noratrieb/cargo-minimize.

- The test script used for `cargo-minimize` is specified in [cargo-minimize-files/run_test.sh](cargo-minimize-files/run_test.sh).
  This script is used by `cargo-minimize` after every change it makes to verify that the properties we want are still
  working.

  These code samples were produced before a recent change in the `cargo-minimize` fork that allows the tool to run
  without the need to create a test file in order to specify the code paths we want preserved.
- The test case for `{vuln,fixed}-file` is specified in [cargo-minimize-files/preserve_vec_file.rs](cargo-minimize-files/preserve_vec_file.rs).
  The command invoked to minimize (with `preserve_vec_file.rs` put in a `tests` directory and `run_test.sh` put in the
  root repo directory) is:

  ```bash
  RUST_BACKTRACE=1 cargo minimize --script-path ./run_test.sh --ignore-file src/vec.rs
  ```

  Note that running `cargo minimize` directly in the rust-mizan repo might not work due to the tool being unmaintained;
  it might not properly read the unused functions output from Cargo build command. This command was done in a separate
  git repo for the crate (so `git clone` the repo in a separate location).

- The test case for `{vuln,fixed}-function` is specified in [cargo-minimize-files/preserve_vec_vuln_fn.rs](cargo-minimize-files/preserve_vec_vuln_fn.rs).
  The command invoked to minimize is:

  ```bash
  RUST_BACKTRACE=1 cargo minimize --script-path ./run_test.sh
  ```

A lot of manual work had to be done to reformat the code, since cargo minimize does some outdated way of reformatting of
the code every time it writes a change. Essentially, each change done by the tool was manually reviewed using the diff
from the staged changes in git. In the future, might be better to just `cargo fmt` first for the original crate so that
we can just do `cargo fmt` for formatting. (However, this would modify the original code appearance.)

The crate also uses tabs instead of spaces, but `cargo-minimize` outputs spaces. Converted back to tabs with

```bash
find . -name "*.rs" -exec bash -c 'unexpand -t 4 --first-only "$0" > /tmp/totabbuff && mv /tmp/totabbuff "$0"' {} \;
```

## Simple statistics

`fixed-crate` has around 20648 lines in total, with 11268 of it being lines of code

`$ cloc --by-file-by-lang --md --force-lang=Rust vuln-0013/fixed-crate/src/`

`cloc|github.com/AlDanial/cloc v 1.98  T=0.04 s (776.2 files/s, 516989.4 lines/s)`

| File                                                  |    blank |  comment |     code |
|:------------------------------------------------------|---------:|---------:|---------:|
| vuln-0013/fixed-crate/src/slice/iter.rs               |      254 |      292 |     1587 |
| vuln-0013/fixed-crate/src/fields/permutation_tests.rs |      269 |       10 |     1449 |
| vuln-0013/fixed-crate/src/slice/api.rs                |      156 |     1635 |      808 |
| vuln-0013/fixed-crate/src/fields.rs                   |      124 |      458 |      587 |
| vuln-0013/fixed-crate/src/domain.rs                   |       60 |      140 |      543 |
| vuln-0013/fixed-crate/src/slice/traits.rs             |       67 |      121 |      519 |
| vuln-0013/fixed-crate/src/macros/internal.rs          |       56 |       72 |      458 |
| vuln-0013/fixed-crate/src/pointer.rs                  |      111 |      431 |      442 |
| vuln-0013/fixed-crate/src/order.rs                    |       40 |      151 |      430 |
| vuln-0013/fixed-crate/src/index.rs                    |      108 |      351 |      415 |
| vuln-0013/fixed-crate/src/vec/traits.rs               |       56 |      203 |      391 |
| vuln-0013/fixed-crate/src/slice.rs                    |       73 |      815 |      357 |
| vuln-0013/fixed-crate/src/vec/ops.rs                  |       83 |      319 |      352 |
| vuln-0013/fixed-crate/src/vec/iter.rs                 |       62 |      244 |      341 |
| vuln-0013/fixed-crate/src/boxed/ops.rs                |       43 |        1 |      309 |
| vuln-0013/fixed-crate/src/boxed/traits.rs             |       32 |        5 |      307 |
| vuln-0013/fixed-crate/src/serdes.rs                   |       37 |       12 |      298 |
| vuln-0013/fixed-crate/src/slice/ops.rs                |       58 |      218 |      298 |
| vuln-0013/fixed-crate/src/vec/api.rs                  |       31 |      547 |      286 |
| vuln-0013/fixed-crate/src/vec.rs                      |       80 |      508 |      183 |
| vuln-0013/fixed-crate/src/macros.rs                   |       40 |       82 |      182 |
| vuln-0013/fixed-crate/src/boxed/iter.rs               |       19 |        4 |      114 |
| vuln-0013/fixed-crate/src/store.rs                    |       50 |      100 |      114 |
| vuln-0013/fixed-crate/src/slice/tests.rs              |       25 |       14 |      111 |
| vuln-0013/fixed-crate/src/boxed.rs                    |       29 |      231 |      110 |
| vuln-0013/fixed-crate/src/access.rs                   |       17 |      121 |       65 |
| vuln-0013/fixed-crate/src/mem.rs                      |       23 |       71 |       55 |
| vuln-0013/fixed-crate/src/slice/proxy.rs              |       11 |       29 |       53 |
| vuln-0013/fixed-crate/src/boxed/api.rs                |        9 |      126 |       47 |
| vuln-0013/fixed-crate/src/lib.rs                      |       15 |       24 |       30 |
| vuln-0013/fixed-crate/src/prelude.rs                  |        3 |        4 |       27 |
| --------                                              | -------- | -------- | -------- |
| SUM:                                                  |     2041 |     7339 |    11268 |

`fixed-file` has around 4327 lines in total, with 1654 of it being lines of code

`$ cloc --by-file-by-lang --md --force-lang=Rust vuln-0013/fixed-file/src/`

| File                                        |    blank |  comment |     code |
|:--------------------------------------------|---------:|---------:|---------:|
| vuln-0013/fixed-file/src/slice/api.rs       |       73 |      162 |      397 |
| vuln-0013/fixed-file/src/pointer.rs         |       87 |      409 |      314 |
| vuln-0013/fixed-file/src/index.rs           |       65 |      237 |      183 |
| vuln-0013/fixed-file/src/vec.rs             |       80 |      508 |      180 |
| vuln-0013/fixed-file/src/vec/api.rs         |       10 |      133 |       81 |
| vuln-0013/fixed-file/src/store.rs           |       41 |       93 |       74 |
| vuln-0013/fixed-file/src/slice.rs           |       31 |      221 |       67 |
| vuln-0013/fixed-file/src/mem.rs             |       23 |       71 |       55 |
| vuln-0013/fixed-file/src/access.rs          |       14 |       88 |       52 |
| vuln-0013/fixed-file/src/order.rs           |       20 |       68 |       42 |
| vuln-0013/fixed-file/src/slice/ops.rs       |        7 |        1 |       33 |
| vuln-0013/fixed-file/src/boxed.rs           |       13 |       34 |       32 |
| vuln-0013/fixed-file/src/vec/ops.rs         |        8 |       31 |       28 |
| vuln-0013/fixed-file/src/boxed/api.rs       |        5 |       45 |       24 |
| vuln-0013/fixed-file/src/slice/proxy.rs     |        7 |       29 |       21 |
| vuln-0013/fixed-file/src/lib.rs             |       11 |       24 |       20 |
| vuln-0013/fixed-file/src/macros/internal.rs |        3 |        8 |       19 |
| vuln-0013/fixed-file/src/prelude.rs         |        3 |        4 |       18 |
| vuln-0013/fixed-file/src/boxed/traits.rs    |        3 |        3 |       14 |
| --------                                    | -------- | -------- | -------- |
| SUM:                                        |      504 |     2169 |     1654 |

`fixed-function` has 2448 lines, with 792 of them being lines of code

| File                                | blank | comment | code |
|:------------------------------------|------:|--------:|-----:|
| vuln-0013/fixed-function/src/lib.rs |   301 |    1355 |  792 |

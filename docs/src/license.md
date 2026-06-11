# License & provenance

- **Code** (the framework, CLI, and tooling) is licensed under **Apache-2.0**.
- **Dataset** is licensed under **CC-BY-4.0**.

The dataset is derived from publicly disclosed memory-safety vulnerabilities in open-source Rust crates, indexed by the [RustSec Advisory Database](https://rustsec.org/). Each source crate retains its own upstream license. The crates carry a mix of common open-source licenses, including MIT, Apache-2.0, MPL-2.0, and BSD-3-Clause. The full list of source repositories and their licenses is maintained alongside the dataset in the [repository](https://github.com/sfu-rsl/rust-mizan).

Only the unmodified **vanilla** split is published as a dataset. The mutated splits (benign, malignant, rust-specific) are not hosted separately; they are regenerated on demand by running the [mutation framework](mutations/index.md) on the vanilla split, via a single-command Docker recipe in the repository.

# RustMizan

RustMizan (**ميزانَ**- meaning "scale" in Arabic) is a benchmark designed to evaluate LLMs for Rust vulnerability detection. The goal is to build a structured dataset of real-world vulnerabilities that provides a framework to test the ability of LLMs to detect vulnerabilities in code samples at different granularities (function, file, module and crate levels).

## Setup Instructions

1. Clone the repository
2. Build the project

```sh
cargo build --workspace
```

## How to Add a New Vulnerability

- Identify a vulnerability from the `datasets/` folder.
- Create a folder to contain all Rust crates (code samples). Name it sequentially after the last reported vulnerability in the repo.
- Identify the commits where the code was vulnerable and where it was fixed.
- Generate vulnerable code samples
  - Copy the entire crate into its own crate inside the new folder
  - Create a module-only crate by reducing the crate to just the vulnerable module.
  - Create a file-only crate by reducing the code to just the vulnerable file.
  - Create a function-only crate by reducing the code to just the vulnerable function.
- Generate non-nulnerable ("complete-code") samples
  - Repeat the same process for the code at the fix commit
- Create a README File. Include relevant information (CVE ID - Commit references before and after the fix - List of sample variants)
- Update `mizan.json`
- Update the dataset folder to mark the vulnerability as done

> [!Note]
> In some cases, it might be challenging or unrealistic to add the whole crate/module (e.g., when dealing with the standard library). If unsure, please open an issue for discussion.

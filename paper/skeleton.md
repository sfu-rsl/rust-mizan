# RustMizan: A Contamination-Aware Multi-Granularity Rust Memory Vulnerability Benchmark

## 1. Introduction

### Problem Statement

Rapid adoption of Rust for systems programming raises concerns about memory safety vulnerabilities. Existing vulnerability benchmarks have several limitations:

1. Emphasize C/C++/Java/Python
2. Lack compilable, multi-granularity artifacts
3. Are susceptible to LLM training contamination (which are also rising as vulnerability detection tools)

Current contamination mitigation approaches for coding benchmarks have limitations:

1. Either rely on costly LLMs (that are not guaranteed to preserve semantics and may hallucinate) or human-in-the-loop processes
2. _Mostly_ target code completion tasks, which is significantly easier than vulnerability detection tasks since the annotations for vulnerability detection are more complex (vulnerable lines and functions)

### Core Contributions

1. **Rust memory safety vulnerability benchmark with compilable multi-granularity artifacts**

   - Unlike existing datasets that provide code snippets, RustMizan offers 173 standalone Rust crates with compilable code at crate, file, and function levels derived via manual reduction
   - Each paired (where available) with vulnerable and fixed variants
   - Annotations include vulnerable lines and functions, with peer-reviewed ground truth

2. **Future-proofing framework against LLM training data contamination**

   - Suite of semantics-preserving syntactic transformation families (formatting, insertion, renaming, AST rewrites) implemented without LLM dependence and designed for repeatability and extensibility
   - 18 fully automatic, semantics-preserving transformations that can be applied to any sample and _automatically re-label_ ground truth
   - Automatic ground-truth maintenance during transformation: vulnerable function signatures and line labels are programmatically updated across sequential transformations

3. **Extensible infrastructure and tooling**

   - Lightweight dataset framework that supports filtering (year, CWE, granularity, vulnerability ID, etc.) which enables analysis of tools' performance across dimensions relevant to Rust memory safety
   - Documented contribution process for adding new samples or transformations which lowers the barrier for further community extensions to facilitate future comparative studies of mitigation and robustness transformations

4. **Comprehensive tool evaluation**
   - Study of state-of-the-art LLMs on Rust memory-safety vulnerability detection: 72 experiments across GPT-4.1, Claude 3.7, Gemini 1.5 Pro, and DeepSeek-Chat, which reveals performance on Rust memory-safety vulnerabilities and the impact of code granularity, CWEs, etc.
   - **TODO**: Program analysis tools evaluation

## 2. Background

### Rust Memory Safety Vulnerabilities

- We can briefly discuss
  - Rust safety mechanisms (ownership, borrowing, lifetimes, etc.)
  - Unsafe Rust
- Overview of common CWE types in Rust (Emphasizing and citing the fact that memory safety vulnerabilities are the most common in Rust)
- RustSec and CVE databases as sources for vulnerabilities

- Inspiration for this subsection:
  - "A Closer Look at the Security Risks in the Rust Ecosystem" ([TOSEM '23](https://dl.acm.org/doi/10.1145/3624738)): "2 BACKGROUND AND PRELIMINARY EXPERIMENTS" section
  - "How Do Programmers Use Unsafe Rust?" ([OOPSLA '20](https://dl.acm.org/doi/10.1145/3428204)): "1 INTRODUCTION"
  - "Memory-Safety Challenge Considered Solved? An In-Depth Study with All Rust CVEs" ([TOSEM '21](https://dl.acm.org/doi/10.1145/3466642)): "2 PRELIMINARY"
  - "Beyond Memory Safety: an Empirical Study on Bugs and Fixes of Rust Programs" ([QRS '24](https://ieeexplore.ieee.org/document/10684674))

### Vulnerability Benchmark Landscape

- We should have the conversation about automatically sourced vulnerability datasets and their limitations

- Inspiration for this subsection:
  - "CleanVul: Toward High-Quality Function-Level Vulnerability Datasets via LLM-Based Noise Reduction" ([arXiv - November 2024](https://ui.adsabs.harvard.edu/abs/2024arXiv241117274L/abstract)): Introduction and related work sections
  - "DiverseVul: A New Vulnerable Source Code Dataset for Deep Learning Based Vulnerability Detection" ([RAID '23](https://dl.acm.org/doi/10.1145/3607199.3607242)): related work section
  - "CVEfixes: Automated Collection of Vulnerabilities and Their Fixes from Open-Source Software" ([PROMISE 2021](https://dl.acm.org/doi/10.1145/3475960.3475985)): related work section
  - "Vulnerability Detection with Code Language Models: How Far Are We?" ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse/2025/056900a469/215aWRJLUZy)): Introduction and related work sections

### Importance of Multi-Granularity Compilable Artifacts

- Existing vulnerability datasets mostly provide code snippets (e.g., functions or files) which lack context and dependencies
- When previous research talks about granularity, they mostly mean that they annotated code samples to mark vulnerable files/functions. Not that they manually reduced the code from crate level to file level and then to function level.
- This limits the ability to evaluate tools that require full context (e.g., inter-procedural analysis, cross-file dependencies)

### The LLM Training Data Contamination Problem

- Why?
  - Existing benchmarks suffer from contamination (we can cite previous research)
  - If we don't mitigate contamination, our dataset will become contaminated in the future and LLM results will be inflated

### Contamination Mitigation Strategies

1. LLM contamination mitigation in general
2. LLM contamination mitigation for coding benchmarks' tasks
   - "Existing research seeks to mitigate the impact of BDC through two primary strategies: curating new benchmarks and updating existing benchmarks. Recent works have proposed novel benchmarks to address contamination. While effective, this approach is costly and time-intensive, requiring significant human effort for labeling and maintenance. An alternative strategy focuses on updating existing high-quality benchmarks. Some methods modify evaluation samples while preserving their semantics. Others generate new samples with altered semantics based on original questions, using advanced LLMs. However, in the latter case, the quality of generated samples is often limited by the task-specific capabilities of the underlying LLMs used in the generation process."
   - We can also add that if you perform semantically inequivalent code samples, you might be assessing different aspects of the LLM by mistake.

## 3. Dataset Construction

### Data Collection and Processing

- Specification
  - Rust memory safety vulnerabilities dataset sourced from CVE reports and RustSec dataset
  - Granularities
    - For each vulnerability, we collect 6 code samples (vulnerable/fixed (crate, file, function)). All code samples are compilable standalone Rust crates
  - For each code sample, we collect
    - `source_link`
    - `crate_name`
    - `year`
    - `cwe_type`
    - `vulnerable_functions`
    - `vulnerable_lines`
- We should describe the process of collecting the dataset (maybe by walking through an example vulnerability and its code samples)
  - We are using Rust toolchain version Version 1.84.1 (2025-01-30) to compile all code samples
  - We start by identifying a vulnerability from RustSec or CVE database. We only consider memory safety vulnerabilities (Use After Free, Buffer Overflow, etc)
  - RustSec Security advisory database has a field for the "Patched version". We use this to identify the "fixed" commit. If there is no "Patched version" listed, we skip the vulnerability (even if there is a fix commit in the GitHub issue). That's because we want to be sure that the fix is recognized by the community as a fix for the vulnerability
  - To identify the "vulnerable" commit, If clear from GitHub issue, use the commit before the fix. If unclear, we use the version before the Patched version listed by RustSec.
  - In some cases (especially for older vulnerabilities), the code at this specific commit doesn't compile with the Rust version we are using in Mizan (1.84.1). In these cases, we make the minimal changes required to get the code to compile. These changes are always non-functional changes and don't affect the vulnerability. In the case that the vulnerability itself is deprecated (e.g., `uninitialized` is deprecated since Rust 1.39.0), we skip the vulnerability entirely (CVE-2018-20996, CVE-2019-15552 as examples)
  - We keep all the code samples in one big Rust workspace. This significantly reduces the compile time since dependencies are only compiled once which is convenient at least for this phase where the dataset is in rapid development. In the future, we can consider splitting the code samples into multiple workspaces if dependency conflicts arise
  - After identifying the vulnerable and fixed commits, we include the whole crate as a code sample (vulnerable and fixed crate level code samples)
  - We then manually reduce the crate level code samples to file level code samples. We pick the file that contains the vulnerability and copy it to a new standalone Rust crate along with all the files it depends on (to make it compilable). We repeat this process for both the vulnerable and fixed crate level code samples. This means that the by file level code samples, we don't mean that the code sample contains just one file. It means that the code sample contains the file that has the vulnerability along with all the files it depends on to make it compilable
  - We then manually reduce the file level code samples to function level code samples. We pick the function that contains the vulnerability and copy it to a new standalone Rust crate along with all the functions/structs/enums/traits it depends on (to make it compilable). We repeat this process for both the vulnerable and fixed file level code samples. This means that the by function level code samples, we don't mean that the code sample contains just one function. It means that the code sample contains the function that has the vulnerability along with all the functions/structs/enums/traits it depends on to make it compilable
  - We then annotate each code sample in `mizan.json` with the metadata mentioned above:
    - `source_link`: link to CVE or RustSec advisory
    - `crate_name`: name of the crate (from `Cargo.toml`)
    - `year`: year of the vulnerability (from CVE database)
    - `cwe_type`: CWE type of the vulnerability (from CVE database). In some cases, it's not annotated in CVE database. In these cases, we manually identify the CWE type based on other sources that discuss the vulnerability (e.g., GitHub advisory database)
    - `vulnerable_functions`: list of function signatures that contain the vulnerability (identified manually from CVE description, GitHub issue description, and code review).
    - `vulnerable_lines`: list of line numbers that contain the vulnerability (identified manually from CVE description, GitHub issue description, and code review)
  - What's described above is the optimal case but obviously not all vulnerabilities can be included in the dataset in the same way so we are flexible with the process. For example:
    1. Some vulnerabilities are not fixed by the maintainers. In these cases, we only have the vulnerable code samples
    2. Some vulnerabilities' crates are just one file. In these cases, we skip the file level code samples
    3. Same for function level code samples where the file is just one function. We skip the function level code samples
    4. Some vulnerabilities don't have clean vulnerable functions per se (e.g., `Send` trait should have been enforced for a struct but wasn't). In these cases, we mark the trait implementation line as the vulnerable function (e.g., `unsafe impl<T, B: Buffer<T>> Send for MPMCConsumer<T, B> {}` where T should have been bounded by `Send` as well)
    5. In some cases, the vulnerable function is called in multiple places. In these cases, we may mark all call sites as vulnerable as well if the vulnerability is exploitable from these call sites. We prefer over-approximation rather than under-approximation in these cases
  - We try to be as consistent as possible with the process described above. However, since the vulnerabilities are manually analyzed, there is some subjectivity involved. To mitigate this, we have a peer review process where each code sample and its annotations are reviewed by at least one other person. This helps catch mistakes and ensures the quality of the dataset.
- NOTE: Emphasize the fact that code samples are compilable at different granularities (crate, file, function)
  - When previous research talks about granularity, they mostly mean that they just annotated code samples to mark vulnerable files/functions
  - We manually reduce the code from crate level to file level and then to function level. all are standalone rust crates that can be compiled. this will be useful
    **TODO**: Add benchmark construction process figure (See [`benchmark-construction-process` from previous literature](./figures/benchmark-construction-process))
- Framework around the dataset
  - Checking out specific code samples. we have filtering for - Year - Granularity - CWE - Include fixed code samples - Specific vulnerability IDs - crate name
    **TODO**: Add dataset structure figure (See [structure figures from previous research](./figures/structure/))
    **TODO**: Add example of a vulnerability that exists at all three granularities and the ground truth for it

### Dataset Statistics

- Statistics report (See [`mizan` dataset analysis](./figures/dataset%20analysis/mizan/dataset_analysis_report.md))

## 4. LLM Contamination Future-Proofing Framework

### Syntactic Transformation Strategies

> **Note**: This section presents a framework to apply transformations to Rust code samples with implementations inspired by previous research. We do not claim that these transformations mitigate contamination. Rather, they are inspired by previous research and we encourage future research to assess their effectiveness. The framework we provide is extensible to allow future research to implement new transformations and assess their effectiveness (e.g., assess the effectiveness of the mitigation strategies we implemented by methods similar to what's described in [1] Y. Sun, H. Wang, D. Li, G. Wang, and H. Zhang, "The Emperor's New Clothes in Benchmarking? A Rigorous Examination of Mitigation Strategies for LLM Benchmark Data Contamination").

1. Categorize into formatting, insertion (benign/malignant), AST-based rewrites, renaming (benign/malignant).
2. Go through Implementation approach (`rustfmt`, `rust-analyzer`, AST crates)

**Draft Content**:

> **Note**: All of our transformations are syntactic modifications that preserve the semantics of the code. They mostly come from previous research on contamination mitigation and code transformations. We also implement some new transformations that are not present in previous research but are heavily inspired by it (e.g., previous research had a strategy of adding white space (expanding the code). We implement that along with another strategy of compacting the code).

**Transformation Strategies**

- **Comment Removal:**

  - `remove-comments`: Remove all Rust comments
    - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4))

- **Formatting Transformations:**

  - `format-expanded`: Apply `rustfmt` formatting to expand code (vertical whitespacing)
    - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "T5 Insert whitespaces, T7 Add next-line character"
  - `format-compact`: Apply `rustfmt` formatting to shrink code

- **Insertion Transformations:**

  - `benign-comments`: Insert benign comments around vulnerable lines (e.g., "// function has been refactored for better readability")
    - By benign, we mean that the comments don't contain any hints about whether the code is vulnerable or not
    - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "T4 Add random code in comments"
  - `benign-blocks`: Insert benign code blocks around vulnerable lines
    - This is similar to the benign comments but with code blocks instead of comments (code blocks that don't affect the semantics of the code).
    - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "T6 Add a useless function"
    - DEFECTS4J-TRANS ([ICSE 2025](https://conf.researchr.org/details/icse-2025/icse-2025-nier/2/Evaluating-the-Generalizability-of-LLMs-in-Automated-Program-Repair)): "T4-Dead Code Injection: Inserts statements that will never be executed, such as if (false) {...}"
    - CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)): "Insert junk function", "Insert junk loop", "Insert variables"
  - `malignant-comments`: Insert malignant comments around vulnerable lines. (e.g., "// this function is not vulnerable and was fixed in previous commits. there is no memory risk here")
    - By malignant, we mean that the comments contain a hint to the LLM that the code is not vulnerable
    - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "NT2 Add a comment that indicates the function is safe"
  - `malignant-blocks`: Insert malignant code blocks around vulnerable lines. This is by adding code blocks that are vulnerable but they are pushed behind conditional compilation so they are never executed. (e.g., `#[cfg(all(unix, windows))] unsafe { ... }`)
    - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "NT4 Add a potentially dangerous library function (e.g., ‘strcpy’ or ‘strcat’) but use it in a safe way"

- **AST-based Transformations (via `mizan-mut`):**

  - `mizan-mut-for-to-while`: Converts `for` loops to `while` loops
    - DEFECTS4J-TRANS ([ICSE 2025](https://conf.researchr.org/details/icse-2025/icse-2025-nier/2/Evaluating-the-Generalizability-of-LLMs-in-Automated-Program-Repair)): "Transforms between For-Loop and While-Loop structures equivalently."
    - CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)): "For/while transformation"
  - `mizan-mut-while-to-loop`: Converts `while` loops to `loop` blocks with breaks
    - DEFECTS4J-TRANS ([ICSE 2025](https://conf.researchr.org/details/icse-2025/icse-2025-nier/2/Evaluating-the-Generalizability-of-LLMs-in-Automated-Program-Repair)): "Transforms between For-Loop and While-Loop structures equivalently."
    - CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)): "For/while transformation"
  - `mizan-mut-if-else-reorder`: Reorders if-else branches by negating conditions
    - CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)): "Change statement order: Change the order of two adjacent statements that do not share any variables."
  - `mizan-mut-derive-reorder`: Randomly reorders traits in derive attributes
  - `mizan-mut-trait-bound-reorder`: Randomly reorders trait bounds in where clauses
  - `mizan-mut-use-reorder`: Randomly reorders items in use statements
  - `mizan-mut-arithmetic-identity`: Adds arithmetic identity operations (x + N - N)
    - CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)): "Equi arithmetic expression"

- **Rename Transformations (via `mizan-mut rename`):**
  - `benign-rename-fn`: Renames functions to neutral names (e.g., `fn_1_abc123`)
    - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "T2 Rename function randomly"
    - DEFECTS4J-TRANS ([ICSE 2025](https://conf.researchr.org/details/icse-2025/icse-2025-nier/2/Evaluating-the-Generalizability-of-LLMs-in-Automated-Program-Repair)): "T1-Variable Renaming: Replaces all variable names in the original code with the new names generated by StarCoder2-3B to maintain the naturalness of code."
    - CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)): "Function rename"
  - `benign-rename-var`: Renames variables to neutral names (e.g., `var_1_xyz789`)
    - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "T1 Rename function parameters randomly"
    - CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)): "Variables rename"
  - `malignant-rename-fn`: Renames functions to names suggesting safety (e.g., `safe_fn_1`, `verified_fn_2`)
    - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "NT3 Change the name of an unsafe function to ‘non vulnerable’ function"
  - `malignant-rename-var`: Renames variables to names suggesting safety (e.g., `secure_var_1`, `checked_var_2`)
    - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "NT1 Change variable names to vulnerability related keywords"

> **Note**: Benign and malignant transformations are only applied to vulnerable code samples because we know the vulnerable code region and target it specifically.

**TODO**: Add figure showing how transformations change original code samples (See [mutation effects figures from previous literature](./figures/mutations%20effect/))

- Should somehow show how we work on inter-dependency files (`rust-analyzer`)

**TODO**: Add table of transformations categorized and their description (See [mutation tables from previous literature](./figures/mutations%20table/))

- Classify our transformations into categories
- Robustness Testing vs Contamination Prevention (cite LLMSecHolmes)

### Automated Transformation Pipeline

We apply transformations to code samples programmatically using:

- `rustfmt`
- `rust-analyzer` rename feature
- Rust compiler AST parser crates (`syn` and `quote`)
- Code and comment insertion

The `mizan-mut` tool has 2 commands (`mutate`, `rename`) exposed as a standalone crate that can be repurposed:

- `mizan-mut mutate` applies transformations to code samples using Rust AST transformations (`syn` and `quote` crates)
- `mizan-mut rename` applies renaming transformations using `rust-analyzer` rename functionality
- We should describe the process of applying mutations
  - We have a framework that allows applying mutations to code samples in a reproducible way in bulk to multiple code samples (e.g., `mizan mutate -m remove-comments -m format-compact -m benign-rename-fn`)
  - Mutations are applied sequentially with the ground truth being updated after each mutation
  - Ground truth automatically updated when applying mutations. we track vulnerable functions, lines of code (by adding marker comments to the vulnerable lines before applying the mutation and then removing them after the mutation is applied and the new ground truth is retrieved)
  - We are very conservative mechanism to apply mutations implemented in the `orchestrator` component of `mizan-cli`:
    - We also perform a sanity check after applying each mutation to ensure that the code sample is still compilable and that the ground truth is still valid
    - If the line of code (or function signature) is not present in the mutated code sample, we have a rollback mechanism that reverts the mutation (only for the failed code samples). This ensures that the ground truth is always accurate and that we don't accidentally remove vulnerable lines of code or functions from the ground truth
  - We also have special case handling in-place for some mutations:
    - In code insertions, sometimes the inserted code can end up in a place where it gets the crate to not compile (e.g., inside a struct initializer (`struct S { foo: 1, bar: 2 }`) - Code inserted inside the `S` struct initializer will cause the crate to not compile). In these cases, we retry with the exact insertion but with different offsets until we find a place where the code compiles (with a maximum of 10 retries). If we can't find a place where the code compiles, we skip the code sample, log the error and continue with the next code sample or the next mutation
    - In renaming mutations, we have a special case for function signatures where the renaming mutator returns a map of the renamed functions/variables to their original names. In this case, we update the ground truth with the original names of the functions/variables so that the ground truth is always accurate.
  - **TODO**: Add figure explaining transformation framework (See [mutation framework from previous literature](./figures/mutation%20framework/))

### Comparison with Existing Approaches

Our transformations are applied without LLMs which makes them:

- **Instant**: No API calls or model inference required
- **Reproducible**: Deterministic output for the same input
- **Dependency-aware**: Work with code samples while preserving cross-file dependencies (which LLM-based transformations struggle with)

> **Note**: When it comes to LLM contamination mitigation for coding benchmarks, the literature we found mostly targets code completion tasks which is significantly easier than vulnerability detection tasks since the annotations for vulnerability detection are more complex (vulnerable lines and functions). The only work we found that addresses contamination for vulnerability detection is SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)) which is manual and limited in scope (48 hand-crafted, 30 real-world, and 150 with code augmentations). Our approach is fully automatic, reproducible, and doesn't depend on LLMs at all. It also works with larger code samples (like whole crates) while preserving cross-file dependencies which LLM-based approaches struggle with.

- CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)) is applying mutations using LLMs and validating they are semantically equivalent to the original codebase using LLMs
  - Fails for larger code samples
  - "Human Verification: Since LLM hallucinations may lead to misjudgments, we manually audited all perturbed code for semantic consistency with the original. Inconsistent code was discarded and re-perturbed by perturbation LLM."
- SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)) synthetically generate the code samples and manually mutate them
  - "We design 228 code scenarios (48 hand-crafted, 30 realworld, and 150 with code augmentations)"
- DEFECTS4J-TRANS ([ICSE 2025](https://conf.researchr.org/details/icse-2025/icse-2025-nier/2/Evaluating-the-Generalizability-of-LLMs-in-Automated-Program-Repair)) only apply AST transformation
  - "we designed five transformation operators and applied them by parsing and modifying abstract syntax trees (AST) using the Java Development Toolkit"
- S. Chen, P. Pusarla, and B. Ray, “Dynamic Benchmarking of Reasoning Capabilities in Code Large Language Models Under Data Contamination,” June 03, 2025, _arXiv_: arXiv:2503.04149. doi: [10.48550/arXiv.2503.04149](https://doi.org/10.48550/arXiv.2503.04149)
  - LLMs bases (similar to CodeMorph) and they also include a human verification step

> **Caveat**: For some AST transformations, there are multiple variants that can be handled. For example, there are multiple `while` loop variants that can be applied to a `for` loop. In these cases, `mizan-mut` handles the very simple case. This should be presented as a caveat of applying transformations programmatically.
>
> In AST-based transformations, we look at the AST and change simple for loops to while loops. This may "succeed" but never actually change the code because this particular code sample didn't have a for loop that we can transform. Similarly, we may apply `format-compact`, but the code was already compact so the changes are minimal.

## 5. Infrastructure and Extensibility

### Command-Line Interface

**TODO**: Discuss `mizan` CLI:

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

### Extensibility Design

- Dataset is extensible to other vulnerabilities (we define very clear instructions on how to add new vulnerabilities in [`CONTRIBUTING.md`](../CONTRIBUTING.md))
- Mutations are extensible. We have a framework that allows adding new mutations easily

## 6. Experimental Evaluation

### Program Analysis Tool Evaluation

- MIRAI (?)
- Kani (?)
  **TODO**:

### Large Language Model Evaluation

- Experiments ran
  - Prompt: **TODO**: Discuss the prompt we used for the experiments and how it's coming from previous literature
    - We use the same prompt for all LLMs
  - temperature: 0.0. **TODO**: Cite SecLLMHolmes
  - Evaluation metrics. **TODO**: Discuss the evaluation metrics we used for the experiments
    - Easy to understand ones (e.g., The LLM was able to detect at least one vulnerable line)
    - Precision, Recall, F1-Score
  - LLMs: GPT-4.1, Claude 3.7, Gemini 1.5 Pro, DeepSeek-Chat
    - **TODO**: A table for the LLMs used with their information (e.g., model size, training cutoff date, etc.)
  - For each LLM, we evaluate the vulnerability detection capabilities on
    - Original code samples
    - Mutated code samples (each mutation separately)
- That leaves us with 4 LLMs \* 18 mutations = 72 experiments
- See report in [experiments-report/report.md](../evaluation/evaluation_results/aggregated_report/report.md)
  - **TODO**: Add comprehensive results table similar to "DiverseVul"

## 7. Results and Discussion

1. Interpret LLM performance on the vanilla dataset (the bulk of this section)
   2. Overall
   3. among granularities
   4. among CWE types
   5. among years
6. Present the results of applying mitigation strategies briefly. present as
   7. a proof that our dataset is not contaminated yet
   8. a research opportunity to asses the effectiveness of different mutation strategies.

## 8. Related Work

1. Vulnerability / defect datasets
   2. Rust security benchmarks (if limited, state gap)
3. LLM vulnerability benchmarks
4. Contamination mitigation / transformation frameworks

DRAFT:

- Vulnerability/defects datasets:
  - Defects4J ([ISSTA 2014](https://dl.acm.org/doi/10.1145/2610384.2628055)), Big-Vul ([MSR '20](https://dl.acm.org/doi/10.1145/3379597.3387501)), CVEfixes ([PROMISE 2021](https://dl.acm.org/doi/10.1145/3475960.3475985)), CrossVul ([ESEC/FSE 2021](https://dl.acm.org/doi/10.1145/3468264.3473122)), ICVul ([ICSE 2025](https://2025.msrconf.org/details/msr-2025-data-and-tool-showcase-track/22/ICVul-A-Well-labeled-C-C-Vulnerability-Dataset-with-Comprehensive-Metadata-and-VCC))
- Vulnerability benchmarks with LLM evaluations:
  - DiverseVul ([RAID '23](https://dl.acm.org/doi/10.1145/3607199.3607242)), PrimeVul ([ICSE 2025](https://www.computer.org/csdl/proceedings-article/icse/2025/056900a469/215aWRJLUZy)), CleanVul ([arXiv - November 2024](https://ui.adsabs.harvard.edu/abs/2024arXiv241117274L/abstract))
- LLM Contamination mitigation papers (for coding benchmarks):
  - CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)), SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)), DEFECTS4J-TRANS ([ICSE 2025](https://conf.researchr.org/details/icse-2025/icse-2025-nier/2/Evaluating-the-Generalizability-of-LLMs-in-Automated-Program-Repair)), DyCodeEval ([ICML '25](https://icml.cc/virtual/2025/poster/46547))
- More relevant papers:
  - The Emperor's New Clothes in Benchmarking? A Rigorous Examination of Mitigation Strategies for LLM Benchmark Data Contamination ([ICML '25](<(https://icml.cc/virtual/2025/poster/45153)>)

## 9. Limitations and Threats to Validity

- Our process of adding code samples is manual. this is powerful but also limits the dataset size (173 code samples)
- We lack test suites that expose each bug (PoV)
- We are including all the code sample crates in one big rust workspace. This means that:
  - The compile time is significantly less than compiling each code sample separately
  - but it would result in dependency conflicts if the code samples have conflicting dependencies (not the current case)
- Our "fixed" versions come from the commit that marked the vulnerability resolved on RustSec which means that we don't necessarily guarantee that only the bug was fixed. In some cases, there are irrelevant changes between the "vulnerable" and "fixed" code samples (This is a common problem in vulnerability datasets)
- "Fixed" code samples can have more vulnerabilities (especially the crate level code samples). Should only be thought of as the code samples where this particular vulnerability is fixed (This is also a common problem in vulnerability datasets)
- We have slight data imbalance between vulnerable and fixed code samples (more vulnerable code samples because some CVEs aren't fixed by the maintainers) (negligible)
- We improved our process gradually and code samples weren't manually updated for already merged code samples. eg:
  - description of the reasoning of the vulnerable lines of code in a README file
  - labeling all call sites of a vulnerable function as vulnerable as well

## 10. Conclusion and Future Work

**TODO**: Write conclusion and future work section

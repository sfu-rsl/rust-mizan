# Related work

- Vulnerability/defects datasets:
  - Defects4J ([ISSTA 2014](https://dl.acm.org/doi/10.1145/2610384.2628055)), Big-Vul ([MSR '20](https://dl.acm.org/doi/10.1145/3379597.3387501)), CVEfixes ([PROMISE 2021](https://dl.acm.org/doi/10.1145/3475960.3475985)), CrossVul ([ESEC/FSE 2021](https://dl.acm.org/doi/10.1145/3468264.3473122)), ICVul ([ICSE 2025](https://2025.msrconf.org/details/msr-2025-data-and-tool-showcase-track/22/ICVul-A-Well-labeled-C-C-Vulnerability-Dataset-with-Comprehensive-Metadata-and-VCC))
- Vulnerability benchmarks with LLM evaluations:
  - DiverseVul ([RAID '23](https://dl.acm.org/doi/10.1145/3607199.3607242)), PrimeVul ([ICSE 2025](https://www.computer.org/csdl/proceedings-article/icse/2025/056900a469/215aWRJLUZy)), CleanVul ([arXiv - November 2024](https://ui.adsabs.harvard.edu/abs/2024arXiv241117274L/abstract))
- LLM Contamination mitigation papers (for coding benchmarks):
  - CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)), SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)), DEFECTS4J-TRANS ([ICSE 2025](https://conf.researchr.org/details/icse-2025/icse-2025-nier/2/Evaluating-the-Generalizability-of-LLMs-in-Automated-Program-Repair)), DyCodeEval ([ICML '25](https://icml.cc/virtual/2025/poster/46547))
- More relevant papers:
  - The Emperor's New Clothes in Benchmarking? A Rigorous Examination of Mitigation Strategies for LLM Benchmark Data Contamination ([ICML '25](<(https://icml.cc/virtual/2025/poster/45153)>)

# Our contributions

- Benchmark design that captures
  - Granularities
  - Compilability (so to work on other tools)
  - LLM contamination awareness
  - "Robustness" awareness (should it be called something else(?))
  - Extensibility (dataset, contamination mitigation strategies)

## Dataset

- Specification
  - Rust memory safety vulnerabilities dataset sourced from CVE reports and RustSec dataset
  - Granularities
    - For each vulnerability, we collect 6 code samples (vulnerable/fixed (crate, file, function))
  - For each code sample, we collect
    - `source_link`
    - `crate_name`
    - `year`
    - `cwe_type`
    - `vulnerable_functions`
    - `vulnerable_lines`
  - Compilable code samples at different granularities
- We should describe the process of collecting the dataset
  - methodology and process including peer reviews, identifying vulnerable lines and functions
  - FIGURE: Benchmark construction process (See [`benchmark-construction-process` from prevous literature](./figures/benchmark-construction-process)
- Framework around the dataset
  - Checking out specific code samples. we have filtering for
    - Year
    - Granularity
    - CWE
    - Include fixed code samples
    - Specific vulnerability IDs
    - NOT IMPLEMENTED YET: Filtering by crate name
- Discussion
  - Why?
    - Rust's growing adoption in systems programming, memory safety guarantees
    - Memory safety vulnerabilities are by far the most common in Rust (We can also cite previous research regarding this)
    - LLMs for vulnerability detection and the contamination problem
  - How?
    - First Rust-specific memory vulnerability benchmark with compilable code at multiple granularities
    - Contamination mitigation framework which makes the dataset future-proof against LLM memorization problem
- FIGURE: We can show the dataset structure (See [structure figures from previous research](./figures/structure/))
- FIGURE: We can also show an example of a vulnerability that exists at all three granularities and the ground truth for it
- Statistics report (See [`mizan` dataset analysis](./figures/dataset%20analysis/mizan/dataset_analysis_report.md))
  - FIGURE: A more comprehensive table with "CVE ID"s similar to [SecLLMHolmes](./figures/dataset%20analysis/SecLLMHolmes.png)

## Future-Proofing Against Contamination

> Instead of calling this "Contamination Mitigation", we can call it "Future-Proofing Against Contamination" because our dataset is not contaminated at the moment

> All of our mutations are syntactic transformations that preserve the semantics of the code. They mostly come from previous research on contamination mitigation and code transformations. Also, we implement some new mutations that are not present in previous research but are heavily inspired by it (e.g., previous research had a strategy of adding white space (expanding the code). we implement that along with another strategy of compacting the code)

- Discussion
  - Why?
    - Existing benchmarks suffer from contamination (we can cite previous research)
      - If we don't mitigate contamination, our dataset will become contaminated in the future and LLM results will be inflated
    - Briefly discuss the ways to mitigate contamination from previous research
      - "Existing research seeks to mitigate the impact of BDC through two primary strategies: curating new benchmarks and updating existing benchmarks. Recent works have proposed novel benchmarks to address contamination. While effective, this approach is costly and time-intensive, requiring significant human effort for labeling and maintenance. An alternative strategy focuses on updating existing high-quality benchmarks. Some methods modify evaluation samples while preserving their semantics. Others generate new samples with altered semantics based on original questions, using advanced LLMs. However, in the latter case, the quality of generated samples is often limited by the task-specific capabilities of the underlying LLMs used in the generation process."
      - We can also add that if you perform semantically inequivalent code samples, you might be assessing different aspects of the LLM by mistake.
- Strategies (inspired by previous research)
  - Comment Removal:
    - `remove-comments`: Remove all Rust comments while preserving code functionality
      - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4))
  - Formatting Mutations:
    - `format-expanded`: Apply `rustfmt` formatting to expand code (vertical whitespacing)
      - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "T5 Insert whitespaces, T7 Add next-line character"
    - `format-compact`: Apply `rustfmt` formatting to shrink code
  - Insertion Mutations:
    - `benign-comments`: Insert benign comments around vulnerable lines
      - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "T4 Add random code in comments"
    - `benign-blocks`: Insert benign code blocks around vulnerable lines
      - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "T6 Add a useless function"
      - DEFECTS4J-TRANS ([ICSE 2025](https://conf.researchr.org/details/icse-2025/icse-2025-nier/2/Evaluating-the-Generalizability-of-LLMs-in-Automated-Program-Repair)): "T4-Dead Code Injection: Inserts statements that will never be executed, such as if (false) {...}"
      - CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)): "Insert junk function", "Insert junk loop", "Insert variables"
    - `malignant-comments`: Insert malignant comments around vulnerable lines
    - `malignant-blocks`: Insert malignant code blocks around vulnerable lines
      - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)): "NT4 Add a potentially dangerous library function (e.g., ‘strcpy’ or ‘strcat’) but use it in a safe way"
  - AST-based Mutations (via `mizan-mut`):
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
  - Rename Mutations (via `mizan-mut rename`):
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
- FIGURE: how the mutations change the original code samples (See [mutations effects figures from previous literature](./figures/mutations%20effect/))
  - Should somehow show how we work on inter-dependency files (`rust-analyzer`)
- FIGURE: Table of mutations categorized and their description (See [mutation tables from previous literature](./figures/mutations%20table/))
  - Classify our mutations into categories
  - Robustness Testing vs Contamination Prevention (cite LLMSecHolmes)
- Framework
  - We apply mutations to the whole code sample programmatically using
    - `rustfmt`
    - `rust-analyzer` rename feature
    - Rust compiler AST parser crates (`syn` and `quote`)
    - Inserting code samples and comments
  - `mizan-mut` has 2 commands (`mutate`, `rename`) exposed as a standalone crate that can be repurposed
    - `mizan-mut mutate` applies mutations to a code sample using Rust AST transformations (`syn` and `quote` crates)
    - `mizan-mut rename` applies renaming mutations to a code sample using `rust-analyzer` rename functionality
  - We should describe the process of applying mutations
    - Ground truth automatically updated when applying mutations. we track vulnerable functions, lines of code
    - Roll back mechanism in case of failure to apply a mutation
    - Applying multiple mutations sequentially
    - FIGURE: explain mutation framework (See [mutation framework from previous literature](./figures/mutation%20framework/))
- Comparison with existing literature on applying code mutations to mitigate LLM contamination
  - CodeMorph ([ICSE '25](https://www.computer.org/csdl/proceedings-article/icse-companion/2025/368300a267/27vTggzV9oA)) is applying mutations using LLMs and validating they are semantically equivalent to the original codebase using LLMs
    - Fails for larger code samples
    - "Human Verification: Since LLM hallucinations may lead to misjudgments, we manually audited all perturbed code for semantic consistency with the original. Inconsistent code was discarded and re-perturbed by perturbation LLM."
  - SecLLMHolmes ([SP '24](https://www.computer.org/csdl/proceedings-article/sp/2024/313000a199/1WPcYpskSK4)) synthetically generate the code samples and manually mutate them
    - "We design 228 code scenarios (48 hand-crafted, 30 realworld, and 150 with code augmentations)"
  - DEFECTS4J-TRANS ([ICSE 2025](https://conf.researchr.org/details/icse-2025/icse-2025-nier/2/Evaluating-the-Generalizability-of-LLMs-in-Automated-Program-Repair)) only apply AST transformation
    - "we designed five transformation operators and applied them by parsing and modifying abstract syntax trees (AST) using the Java Development Toolkit"
  - S. Chen, P. Pusarla, and B. Ray, “Dynamic Benchmarking of Reasoning Capabilities in Code Large Language Models Under Data Contamination,” June 03, 2025, _arXiv_: arXiv:2503.04149. doi: [10.48550/arXiv.2503.04149](https://doi.org/10.48550/arXiv.2503.04149)
    - LLMs bases (similar to CodeMorph) and they also include a human verification step.
  - Disadvantages
    - Benign and malignant code transformations don't get applied to non-vulnerable code samples
    - In AST based mutations, we look at the AST and change the simple for loops to while loops. this may "succeed" but never actually change the code because this particular code sample didn't have a for loop that we can mutate
    - Similarly, we may apply `format-compact`, but the code was already compact that the changes are minimal

## LLM Experiments

- Experiments ran
  - Performance on the vanilla dataset ([GPT](../evaluation/evaluation_results/experiment_7dd2c700/analysis/analysis_report.md) and [Claude](../evaluation/evaluation_results/experiment_7f0feb42/analysis/analysis_report.md))
  - Performance of applying each mutation separately (GPT and Claude)
- See report in [experiments-report/report.md](../evaluation/evaluation_results/aggregated_report/report.md)
  - FIGURE: a more comprehensive results table similar to "DiverseVul"

## Strengths and weaknesses

### strengths

- Code samples are compilable at different granularities (crate, file, function)
  - When previous research talks about granularity, they mostly mean that they just annotated code samples to mark vulnerable files/functions
  - We manually reduce the code from crate level to file level and then to function level. all are standalone rust crates that can be compiled. this will be useful
- Our mutations are applied without LLMs which makes them:
  - instant
  - reproducible
  - work with code samples while preserving cross-file dependencies (which LLM-based mutations struggle with)
- We automatically update the ground truth when applying mutations which means our mutations can be applied on more than just "Code Completion" tasks (like CodeMorph)

### weaknesses

- Dataset size
- Our process of adding code samples is manual
- We lack test suites that expose each bug (PoV)
- The architecture of one rust workspace would result later on in dependency conflicts
- Our "fixed" versions come from the commit that marked the vulnerability resolved on RustSec which means that we don't necessarily guarantee that only the bug was fixed. In some cases, there are irrelevant changes between the "vulnerable" and "fixed" code samples
- "Fixed" code samples can have more vulnerabilities (specially the crate level code samples). Should only be thought of as the code samples where this particular vulnerability is fixed
- We used the CVE description and GitHub issue description to identify the vulnerable lines of code and vulnerable functions signatures. These were peer reviewed but are still subjective. still way better than any automation script though
- Github commits aren't part of our JSON ground truth
  - We keep track of them in the README
  - Can be moved to the JSON if this makes a difference
- We have slight data imbalance between vulnerable and fixed code samples (more vulnerable code samples because some CVEs aren't fixed by the maintainers)
- We improved our process gradually and code samples weren't manually updated for already merged code samples. eg:
  - description of the reasoning of the vulnerable lines of code in a README file
  - labeling all call sites of a vulnerable function as vulnerable as well

## Ideas for future work

- We consider "benign-blocks" as just one of the contamination mitigations. It can be split into multiple strategies ("Insert junk function", "Insert junk loop", "Insert variables") (Similar to CodeMorph's perturbations)
- Automated dataset collection
  - We can have some kind of automated code sample collection based on LLM agents (given that "smaller specialized models sometimes outperform larger ones")
- Assess the effectiveness of the mitigation strategies we implemented by methods similar to what's described in [1] Y. Sun, H. Wang, D. Li, G. Wang, and H. Zhang, “The Emperor’s New Clothes in Benchmarking? A Rigorous Examination of Mitigation Strategies for LLM Benchmark Data Contamination”.

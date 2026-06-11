# Evaluation

RustMizan evaluates models on the full vulnerability analysis pipeline, not just Crate Vulnerability Classification (CVC), the binary judgment of whether the code is vulnerable.

## The task

Each evaluation places an agent in a sandboxed Docker container holding one compilable variant and a shell. The agent can explore the codebase, compile it, and read any file before producing its analysis. `cargo` and `rustc` are available; other tools (clippy, miri, static analyzers) are not.

The agent writes a `results.json` file covering four tasks:

```json
{
  "explanation": "reasoning and recall",
  "is_vulnerable": true,
  "cwe_type": ["CWE-416"],
  "vulnerable_functions": { "src/lib.rs": ["pub fn read_byte(buf: &[u8], idx: usize) -> u8"] },
  "vulnerable_lines": { "src/lib.rs": [4] }
}
```

All agent steps and reasoning traces are logged, which enables trajectory analysis (for example, spotting a model that recalls a CVE identifier from memory). The complete trajectories are published to the [rust-mizan-logs](https://huggingface.co/spaces/sfu-rsl/rust-mizan-logs) Inspect log viewer and linked from each result on the [Leaderboard](leaderboard.md).

## Harness

The harness is built on [Inspect-AI](https://inspect.aisi.org.uk). Each sample runs in its own Docker sandbox. The default configuration uses a ReAct (reasoning + acting) scaffold with bash access, a message limit, and a per-task timeout. The setup reflects interactive analysis: the agent decides what to examine and in what order, rather than receiving a pre-cut snippet.

## Metrics

Crate Vulnerability Classification (CVC) is a binary metric. CWE classification and the two localization tasks are set-based: predicted elements are compared against the ground-truth set, and true/false positives and negatives are counted per sample. The F1, precision, and recall figures are **micro-averaged**: TP, FP, and FN are summed across all variants first, then combined into one score. An invalid JSON response contributes zeros.

| Metric | Definition |
| --- | --- |
| **CVC Accuracy** | Fraction of samples where the binary `is_vulnerable` prediction matches ground truth. Over all samples. |
| **CWE F1 / Precision / Recall** | Micro-averaged set overlap between predicted and ground-truth CWE types. |
| **Function F1 / Precision / Recall** | Micro-averaged set overlap between predicted and ground-truth vulnerable functions. |
| **Line F1 / Precision / Recall** | Micro-averaged set overlap between predicted and ground-truth vulnerable lines. |
| **Success@1-Function** | Fraction of vulnerable samples where at least one correct function was identified. Over vulnerable samples only. |
| **Success@1-Line** | Fraction of vulnerable samples where at least one correct line was identified. Over vulnerable samples only. |
| **Invalid JSON Rate** | Fraction of samples where the model returned invalid JSON. |

These are the same metrics shown on the [Leaderboard](leaderboard.md).

## Running an evaluation

The evaluation consumes a parquet file produced by [`mizan evaluate prepare-dataset`](cli.md#evaluate-prepare-dataset). Configure and launch a run with `run_eval.py`:

```bash
cd mizan-cli
# Edit run_eval.py: DATASET_PATH, MODELS, MESSAGE_LIMIT, TIME_LIMIT
python run_eval.py

# Inspect the results
inspect view
```

`run_eval.py` exposes the full configuration as a script, including the agent scaffold, which can be replaced with a custom implementation to evaluate different prompting strategies. See the [Inspect-AI documentation](https://inspect.aisi.org.uk) for supported models and options.

To publish your results to the public leaderboard, see [Submit leaderboard results](contributing/leaderboard.md).

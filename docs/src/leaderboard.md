# Leaderboard

The [RustMizan Leaderboard](https://huggingface.co/spaces/sfu-rsl/rust-mizan-leaderboard) reports how models perform across the dataset variants. It is a Gradio app hosted on Hugging Face Spaces.

[Open the leaderboard](https://huggingface.co/spaces/sfu-rsl/rust-mizan-leaderboard)

## Tabs

- **Leaderboard.** Aggregate [metrics](evaluation.md#metrics) per model and dataset variant. You can filter by model, by dataset variant, by code granularity (function / file / crate), and by vulnerability type, choose which metric columns to show, and download the table as CSV. There is also a toggle for whether to count invalid-JSON responses as wrong or exclude them.
- **Sample-wise Comparison.** Per-CVE correctness across models. Each cell shows three markers for the crate, file, and function variants: correct, wrong, not present at that level, or invalid JSON. Clicking a result opens that run's full agent trajectory (prompt, reasoning, tool calls, and scoring) in the [Inspect log viewer](https://huggingface.co/spaces/sfu-rsl/rust-mizan-logs).
- **Metrics.** Definitions of every metric.
- **Variants.** Descriptions of the dataset variants and the mutations each one applies.
- **About.** The task description.

## Dataset variants

The leaderboard groups results by variant. Each variant is a fixed set of [mutations](mutations/index.md):

| Variant | What it tests |
| --- | --- |
| **Vanilla** | The original, unmutated code (baseline) |
| **Benign** | Contamination: surface rewrites that break memorization |
| **Malignant** | Robustness: adversarial cues that falsely suggest safety |
| **Rust-Specific** | Idiomatic structural rewrites specific to Rust |

## Trajectories

Every run's complete agent trajectory is published to the [rust-mizan-logs](https://huggingface.co/spaces/sfu-rsl/rust-mizan-logs) Inspect log viewer. From the Sample-wise Comparison tab, each result links directly to its trajectory, so any score can be traced back to the model's prompts, reasoning, tool calls, and the scoring that produced it.

## Fair comparison

When you compare models, the leaderboard restricts to the samples common to all selected experiments, so the numbers are comparable even if runs cover slightly different sample sets.

## Submitting results

To add your own results, see [Submit leaderboard results](contributing/leaderboard.md).

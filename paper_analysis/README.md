# Paper analysis

Put the 16 original `.eval` logs in `evals/`. For dataset/project statistics, also put
`mizan-vanilla.parquet` there. Both are available from
[rustmizan-eval-logs](https://huggingface.co/datasets/rustmizan-org/rustmizan-eval-logs)
and [mizan-vanilla](https://huggingface.co/datasets/rustmizan-org/mizan-vanilla).

Install with `python -m pip install -r requirements.txt` then run scripts from this
folder with `python SCRIPT.py`.

- `tables.py`: main results and mutation comparison, as CSV and LaTeX.
- `detection_localization_gap.py`: detection/localization figure, PDF and PNG.
- `granularity.py`: line-localization figure by context level, PDF.
- `uncertainty.py`: 200,000 paired CVE/advisory bootstrap draws, JSON and three LaTeX tables.
- `per_sample_results.py`: per-vulnerability localization by level, CSV and LaTeX.
- `configuration.py`: model IDs, reasoning settings, and evaluation limits.
- `token_usage.py`: total/per-variant tokens and messages.
- `dataset_statistics.py`: dataset counts and Rust code sizes, JSON and CSV.
- `project_statistics.py`: project sizes/downloads, CSV and LaTeX. This uses the 4 October 2026 counts in `project_downloads.json`.
- `preservation.py`: preservation table from the archived `preservation_counts.txt` report.
- `common.py`: shared log loading and micro-metric calculations.

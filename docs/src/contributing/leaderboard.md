# Submit leaderboard results

The [leaderboard](../leaderboard.md) is a separate repository (the Hugging Face Space). Adding results means contributing the processed output of an Inspect-AI run to that repo.

First, run an evaluation and produce an Inspect-AI `.eval` file (see [Evaluation](../evaluation.md)). Then, in the leaderboard repo:

1. **Add the `.eval` file** to `data/eval_files/`.
   ```bash
   cp your_experiment.eval data/eval_files/
   ```
2. **Register it** in `data/leaderboard_config.json` by adding an entry to the `experiments` array:
   ```json
   {
     "name": "Agent + Model",
     "eval_path": "data/eval_files/your_experiment.eval"
   }
   ```
3. **Add the variant (if new).** If your eval uses a new `tag`, map it to a display name in `data/dataset_info.json`:
   ```json
   { "your_tag": "Display Name" }
   ```
4. **Run preprocessing.**
   ```bash
   python preprocess_evals.py
   ```
   This reads each `.eval` file, extracts the per-sample scores into `data/experiments/<name>_<tag>.json`, and regenerates `data/processed_config.json`, which the app loads at startup.
5. **Open a pull request** against the Space with your changes. You can browse and create pull requests from the Space's Community tab: [open pull requests](https://huggingface.co/spaces/sfu-rsl/rust-mizan-leaderboard/discussions?status=open&type=pull_request&sort=recently-created).

The committed JSON files in `data/experiments/` (not the large `.eval` files) are what the app serves. See the leaderboard repo's `CONTRIBUTING.md` for the canonical version of these steps.

## Publish the trajectories

The Sample-wise Comparison tab links each result to its full trajectory in the [rust-mizan-logs](https://huggingface.co/spaces/sfu-rsl/rust-mizan-logs) Inspect log viewer. That viewer is regenerated from the raw `.eval` files (which are not stored in the repo), so refresh it after adding runs:

```bash
export HF_TOKEN=hf_...   # write access to sfu-rsl
python publish_logs.py   # defaults to ../agentic_evals/logs
```

This bundles the `.eval` files into a static Inspect viewer and uploads it to the Space, replacing the previous contents. Pass `--logs-dir` / `--space` to override the defaults.

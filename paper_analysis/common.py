"""Read the saved scorer counters and compute the paper's micro metrics."""

import glob
import json
import os
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)


from pathlib import Path
import csv

ROOT = Path(__file__).resolve().parent
CODE = ROOT.parent
LOGS = ROOT / "evals"
OUT = ROOT / "output"
OUT.mkdir(exist_ok=True)

# Model id in the logs -> short name used in the paper's tables.
MODELS = {
    "anthropic/claude-sonnet-4-6": "Claude",
    "openai/gpt-5.4": "GPT",
    "google/gemini-3.1-pro-preview": "Gemini",
    "openrouter/qwen/qwen3.6-plus": "Qwen",
}
MODEL_ORDER = ["Claude", "GPT", "Gemini", "Qwen"]
SPLIT_ORDER = ["vanilla", "benign", "rust-specific", "malignant"]

# Per-sample counters that the scorer already stored in every log.
COUNTERS = (
    "is_vulnerable_gt",
    "binary_accuracy",
    "cwe_tp", "cwe_fp", "cwe_fn",
    "function_tp", "function_fp", "function_fn",
    "line_tp", "line_fp", "line_fn",
    "success_at_1_function",
    "success_at_1_line",
)


# ---------------------------------------------------------------- metrics
# Mirrors metrics.py: compute_f1_score / compute_precision / compute_recall.

def f1(tp, fp, fn):
    if tp == 0 and fp == 0 and fn == 0:
        return 1.0
    if tp == 0:
        return 0.0
    p, r = tp / (tp + fp), tp / (tp + fn)
    return 2 * p * r / (p + r)


def precision(tp, fp):
    return tp / (tp + fp) if (tp + fp) > 0 else 0.0


def recall(tp, fn):
    return tp / (tp + fn) if (tp + fn) > 0 else 0.0


def micro(rows, prefix):
    """Micro-averaged (precision, recall, f1) as percentages, summing TP/FP/FN first."""
    tp = sum(r[prefix + "_tp"] for r in rows)
    fp = sum(r[prefix + "_fp"] for r in rows)
    fn = sum(r[prefix + "_fn"] for r in rows)
    return precision(tp, fp) * 100, recall(tp, fn) * 100, f1(tp, fp, fn) * 100


def line_f1(rows):
    return micro(rows, "line")[2]


# ---------------------------------------------------------------- loading

def _split_name(task):
    return task.replace("mizan-", "")


def load_logs():
    """-> {(split, model): {sample_id: row}}, one row per evaluated variant."""
    out = {}
    files = sorted(glob.glob(os.path.join(LOGS, "*.eval")))
    if not files:
        raise SystemExit(
            "No .eval logs found in %s\n"
            "Clone them first:\n"
            "  git clone https://huggingface.co/datasets/rustmizan-org/rustmizan-eval-logs"
            % LOGS
        )
    for path in files:
        z = zipfile.ZipFile(path)
        header = json.loads(z.read("header.json"))
        split = _split_name(header["eval"]["task"])
        mid = header["eval"]["model"]
        if mid not in MODELS or split not in SPLIT_ORDER:
            raise ValueError("Unexpected model/split: %s / %s" % (mid, split))
        model = MODELS[mid]
        if (split, model) in out:
            raise ValueError("Duplicate run: %s / %s" % (split, model))
        assert header["status"] == "success"
        rows = {}
        for name in z.namelist():
            if not name.startswith("samples/"):
                continue
            s = json.loads(z.read(name))
            md = s["scores"]["rustmizan_scorer"]["metadata"]
            row = {
                "sample_id": s["id"],
                "vuln_id": s["metadata"]["vuln_id"],
                "crate_name": s["metadata"]["crate_name"],
                "granularity": s["metadata"]["granularity"],
                "year": s["metadata"]["year"],
            }
            row.update({k: md[k] for k in COUNTERS})
            assert s["id"] not in rows
            row["valid"] = s["scores"]["rustmizan_scorer"]["answer"] == "valid"
            row["cwe_type"] = s["metadata"]["cwe_type"]
            rows[s["id"]] = row
        assert len(rows) == header["results"]["completed_samples"]
        out[(split, model)] = rows
        z.close()
    return out


def load_configs():
    """-> list of per-log config dicts (model, split, effort settings, limits)."""
    out = []
    for path in sorted(glob.glob(os.path.join(LOGS, "*.eval"))):
        header = json.loads(zipfile.ZipFile(path).read("header.json"))
        ev, cfg = header["eval"], header["eval"].get("config", {})
        out.append({
            "split": _split_name(ev["task"]),
            "model": MODELS[ev["model"]],
            "model_id": ev["model"],
            "plan_config": header["plan"].get("config", {}),
            "model_args": ev.get("model_args", {}),
            "epochs": cfg.get("epochs"),
            "message_limit": cfg.get("message_limit"),
            "time_limit": cfg.get("time_limit"),
            "samples": ev["dataset"]["samples"],
            "completed": header["results"].get("completed_samples"),
        })
    return out



NAMES = {"Claude": "Claude Sonnet 4.6", "GPT": "GPT 5.4",
         "Gemini": "Gemini 3.1 Pro", "Qwen": "Qwen 3.6 Plus"}


def write_csv(name, header, rows):
    with (OUT / name).open("w", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(header)
        writer.writerows(rows)


def write_table(name, header, rows):
    text = [r"\begin{tabular}{@{}l" + "r" * (len(header) - 1) + r"@{}}",
            r"\toprule", " & ".join(header) + r" \\", r"\midrule"]
    text.extend(" & ".join(map(str, row)) + r" \\" for row in rows)
    text += [r"\bottomrule", r"\end{tabular}"]
    (OUT / name).write_text("\n".join(text) + "\n")

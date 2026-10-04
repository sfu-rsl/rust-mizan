#!/usr/bin/env python3
"""Detection-to-localization gap figure"""

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
import os

from common import MODEL_ORDER, OUT, load_logs, micro

logs = load_logs()
models = ["Claude\nSonnet 4.6", "GPT 5.4", "Gemini\n3.1 Pro", "Qwen\n3.6 Plus"]
rows = [list(logs[("vanilla", model)].values()) for model in MODEL_ORDER]
cvc_acc = [round(100 * sum(r["binary_accuracy"] for r in rs) / len(rs), 1) for rs in rows]
cwe_f1 = [round(micro(rs, "cwe")[2], 1) for rs in rows]
func_f1 = [round(micro(rs, "function")[2], 1) for rs in rows]
line_f1 = [round(micro(rs, "line")[2], 1) for rs in rows]

avg_detect = np.mean(cvc_acc)
avg_line = np.mean(line_f1)
delta = avg_detect - avg_line

# ---------- style ----------
plt.rcParams.update(
    {
        "font.family": "sans-serif",
        "font.size": 10,
        "axes.linewidth": 0.5,
        "xtick.major.width": 0.4,
        "ytick.major.width": 0.4,
    }
)

fig, ax = plt.subplots(figsize=(6.8, 3.0))

n = len(models)
x = np.arange(n)
bar_w = 0.17
gap = 0.02

# Colors
c_detect = "#E8943C"
c_cwe = "#9B8EC8"
c_func = "#3A9B78"
c_line = "#2670A9"

# ---------- subtle gap shading between avg detection and avg line F1 ----------
ax.axhspan(avg_line, avg_detect, color="#E8943C", alpha=0.06, zorder=0)

# Bolder horizontal lines at the averages
ax.axhline(
    y=avg_detect, color=c_detect, linewidth=1.6, linestyle="-", alpha=0.7, zorder=1
)
ax.axhline(y=avg_line, color=c_line, linewidth=1.6, linestyle="-", alpha=0.7, zorder=1)


# ---------- bars ----------
offsets = [-1.5, -0.5, 0.5, 1.5]

bars_detect = ax.bar(
    x + offsets[0] * (bar_w + gap),
    cvc_acc,
    bar_w,
    color=c_detect,
    edgecolor="white",
    linewidth=0.3,
    zorder=3,
    label="CVC accuracy",
)
bars_cwe = ax.bar(
    x + offsets[1] * (bar_w + gap),
    cwe_f1,
    bar_w,
    color=c_cwe,
    edgecolor="white",
    linewidth=0.3,
    zorder=3,
    label="CWE F1",
)
bars_func = ax.bar(
    x + offsets[2] * (bar_w + gap),
    func_f1,
    bar_w,
    color=c_func,
    edgecolor="white",
    linewidth=0.3,
    zorder=3,
    label="Function F1",
)
bars_line = ax.bar(
    x + offsets[3] * (bar_w + gap),
    line_f1,
    bar_w,
    color=c_line,
    edgecolor="white",
    linewidth=0.3,
    zorder=3,
    label="Line F1",
)


# ---------- value labels ----------
def label_bars(bars, color, fmt="{:.0f}"):
    for bar in bars:
        h = bar.get_height()
        if h < 4:
            continue
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            h + 0.6,
            fmt.format(h),
            ha="center",
            va="bottom",
            fontsize=7,
            color=color,
            fontweight="medium",
        )


label_bars(bars_detect, "#8B5A1E")
label_bars(bars_cwe, "#5B4F8A", fmt="{:.1f}")
label_bars(bars_func, "#1D6B4E", fmt="{:.1f}")
label_bars(bars_line, "#164D7A", fmt="{:.1f}")

# ---------- horizontal-line labels (far right) ----------
label_x = n + 0.02
ax.text(
    label_x,
    avg_detect + 0.8,
    "CVC Acc.",
    fontsize=8,
    color=c_detect,
    va="bottom",
    ha="right",
    fontweight="medium",
)
ax.text(
    label_x,
    avg_line - 0.8,
    "Line F1",
    fontsize=8,
    color=c_line,
    va="top",
    ha="right",
    fontweight="medium",
)

# ---------- delta annotation (right margin) ----------
ann_x = n - 0.45
ax.annotate(
    "",
    xy=(ann_x, avg_line + 0.5),
    xytext=(ann_x, avg_detect - 0.5),
    arrowprops=dict(arrowstyle="<->", color="#993322", lw=1.2, shrinkA=0, shrinkB=0),
)
ax.text(
    ann_x + 0.08,
    (avg_detect + avg_line) / 2,
    f"avg. \u0394{delta:.0f}",
    fontsize=7.5,
    color="#993322",
    va="center",
    ha="left",
    fontstyle="italic",
    linespacing=1.15,
)

# ---------- formatting ----------
ax.set_xticks(x)
ax.set_xticklabels(models, fontsize=12)
ax.set_ylabel("Score (%)", fontsize=13)
ax.set_ylim(0, 70)
ax.set_xlim(-0.55, n + 0.05)
ax.tick_params(axis="y", labelsize=11)

ax.yaxis.grid(True, linewidth=0.25, alpha=0.35, color="#999999")
ax.set_axisbelow(True)

ax.spines["top"].set_visible(False)
ax.spines["right"].set_visible(False)
ax.spines["left"].set_color("#AAAAAA")
ax.spines["bottom"].set_color("#AAAAAA")

legend = ax.legend(
    loc="lower center",
    bbox_to_anchor=(0.5, 1.02),
    ncol=4,
    frameon=True,
    fontsize=11,
    handlelength=1.1,
    handletextpad=0.35,
    columnspacing=1.0,
    borderpad=0.35,
    fancybox=False,
    edgecolor="#DDDDDD",
)
legend.get_frame().set_linewidth(0.3)

plt.tight_layout(pad=0.4)
OUT_DIR = str(OUT)
plt.savefig(os.path.join(OUT_DIR, "detection_localization_gap.pdf"), bbox_inches="tight", dpi=300)
plt.savefig(os.path.join(OUT_DIR, "detection_localization_gap.png"), bbox_inches="tight", dpi=200)
print(f"Figures saved to {OUT_DIR}")

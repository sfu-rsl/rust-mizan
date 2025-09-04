import pandas as pd
from pathlib import Path

current_dir = Path(__file__).parent.parent
data_dir = current_dir / "data"
latex_dir = current_dir / "latex"

df = pd.read_csv(data_dir / "dataset_statistics_comprehensive.csv")
stats = dict(zip(df["metric"], df["value"]))

with open(latex_dir / "TEMPLATE_dataset_summary.tex", "r") as f:
    template = f.read()

replacements = {
    "42": str(int(stats["total_vulnerabilities"])),
    "35 (83.3\\%)": f'{int(stats["vulnerabilities_with_fix"])} ({stats["vulnerabilities_with_fix_percent"]}\\%)',
    "173": str(int(stats["total_code_samples"])),
    "77": str(int(stats["crate_level_samples"])),
    "39": str(int(stats["file_level_samples"])),
    "57": str(int(stats["function_level_samples"]))
}

for old, new in replacements.items():
    template = template.replace(old, new, 1)

with open(latex_dir / "dataset_summary_generated.tex", "w") as f:
    f.write(template)
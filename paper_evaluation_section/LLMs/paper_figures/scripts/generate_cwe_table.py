import json
from pathlib import Path
from collections import defaultdict
import html

current_dir = Path(__file__).parent.parent
data_dir = current_dir / "data"
latex_dir = current_dir / "latex"

mizan_json_path = current_dir.parent.parent.parent / "mizan.json"
with open(mizan_json_path, "r") as f:
    mizan_data = json.load(f)

mapping_path = data_dir / "vulnerability_mapping.json"
with open(mapping_path, "r") as f:
    vuln_mapping = json.load(f)

cwe_descriptions = {}
for vuln_id, vuln_info in vuln_mapping.items():
    cwe = vuln_info.get("cwe", "")
    description = vuln_info.get("description", "")
    if cwe and description:
        description = html.unescape(description)
        if "Race Condition" in description:
            description = "Race Condition"
        elif "Memory Buffer" in description:
            description = "Memory Buffer Violation"
        elif "Array Index" in description:
            description = "Improper Validation of Array Index"
        cwe_descriptions[cwe] = description

cwe_vuln_count = defaultdict(set)
cwe_sample_count = defaultdict(int)

for vuln in mizan_data["vulnerabilities"]:
    vuln_id = vuln["id"]
    for sample in vuln["code_samples"]:
        for cwe in sample.get("cwe_type", []):
            cwe_vuln_count[cwe].add(vuln_id)
            cwe_sample_count[cwe] += 1

cwe_stats = []
for cwe in cwe_vuln_count:
    cwe_stats.append({
        "cwe": cwe,
        "desc": cwe_descriptions.get(cwe, "Unknown"),
        "vulns": len(cwe_vuln_count[cwe]),
        "samples": cwe_sample_count[cwe]
    })

cwe_stats.sort(key=lambda x: x["samples"], reverse=True)
top_5 = cwe_stats[:5]

with open(latex_dir / "TEMPLATE_cwe_table.tex", "r") as f:
    template = f.read()

rows = []
for stat in top_5:
    rows.append(f"{stat['cwe']} & {stat['desc']} & {stat['vulns']} & {stat['samples']} \\\\")

table_content = "\n".join(rows)

lines = template.split("\n")
new_lines = []
for line in lines:
    if "CWE-416" in line:
        new_lines.append(table_content)
        break
    else:
        new_lines.append(line)

for line in lines[lines.index("\\bottomrule"):]:
    new_lines.append(line)

with open(latex_dir / "cwe_table_generated.tex", "w") as f:
    f.write("\n".join(new_lines))
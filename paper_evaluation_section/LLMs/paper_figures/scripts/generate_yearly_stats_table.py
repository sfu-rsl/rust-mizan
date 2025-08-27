import json
from pathlib import Path
from collections import defaultdict

# Setup paths
current_dir = Path(__file__).parent.parent
data_dir = current_dir / "data"
latex_dir = current_dir / "latex"

# Load mizan.json to get yearly statistics
mizan_json_path = current_dir.parent.parent.parent / "mizan.json"
with open(mizan_json_path, "r") as f:
    mizan_data = json.load(f)

# Initialize yearly counters
yearly_stats = defaultdict(lambda: {
    "vulnerabilities": set(),
    "total_samples": 0  # Count all samples (vulnerable + fixed)
})

# Process each vulnerability
for vuln in mizan_data["vulnerabilities"]:
    vuln_id = vuln["id"]
    year = vuln.get("year", None)
    
    if year:
        yearly_stats[year]["vulnerabilities"].add(vuln_id)
        
        # Count all samples (both vulnerable and fixed)
        for sample in vuln["code_samples"]:
            yearly_stats[year]["total_samples"] += 1

# Convert to sorted list
years_list = []
for year in sorted(yearly_stats.keys()):
    years_list.append({
        "year": year,
        "vulns": len(yearly_stats[year]["vulnerabilities"]),
        "samples": yearly_stats[year]["total_samples"]
    })

# Calculate totals
total_vulns = sum(stat["vulns"] for stat in years_list)
total_samples = sum(stat["samples"] for stat in years_list)

# Read template
with open(latex_dir / "TEMPLATE_yearly_stats_table.tex", "r") as f:
    template = f.read()

# Generate table rows
rows = []
for stat in years_list:
    rows.append(f"{stat['year']} & {stat['vulns']} & {stat['samples']} \\\\")

table_content = "\n".join(rows)

# Build the new table
lines = template.split("\n")
new_lines = []
in_data_section = False

for line in lines:
    if "\\midrule" in line:
        new_lines.append(line)
        in_data_section = True
    elif "\\bottomrule" in line:
        new_lines.append(table_content)
        new_lines.append("\\bottomrule")
        in_data_section = False
    elif not in_data_section:
        new_lines.append(line)

# Write generated table
output_path = latex_dir / "yearly_stats_table_generated.tex"
with open(output_path, "w") as f:
    f.write("\n".join(new_lines))

print(f"Generated yearly statistics table at: {output_path}")

# Print summary for verification
print(f"\nDataset Summary by Year:")
print(f"{'Year':<10} {'Vulnerabilities':<20} {'Code Samples':<20}")
print("-" * 50)
for stat in years_list:
    print(f"{stat['year']:<10} {stat['vulns']:<20} {stat['samples']:<20}")
print("-" * 50)
print(f"{'Total':<10} {total_vulns:<20} {total_samples:<20}")
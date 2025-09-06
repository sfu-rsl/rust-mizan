# A simple script to count the number of invalid JSON entries in a `processed_results.csv` file.
# Run this script from the `scripts` directory.
import csv

# set the experiment ID
experiment_id = "9c998f04"
csv_path = f"../../evaluation_results/experiment_{experiment_id}/processed_results.csv"


total = 0
false_count = 0

with open(csv_path, newline="", encoding="utf-8") as csvfile:
    reader = csv.DictReader(csvfile)
    for row in reader:
        total += 1
        if row["is_valid_json"].strip().upper() == "FALSE":
            false_count += 1

print(
    f"{false_count} variants are invalid JSON out of {total}. This is for experiment ID {experiment_id}."
)

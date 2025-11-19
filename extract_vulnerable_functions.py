import json
import re
import sys
from pathlib import Path


def extract_function_from_rust_file(file_path: Path, function_signature: str) -> str:
    if not file_path.exists():
        return None

    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()
        lines = content.split("\n")

    # Normalize the function signature for matching
    # Remove extra whitespace and make it more flexible
    sig_pattern = function_signature.strip()
    # Escape special regex characters
    sig_pattern = re.escape(sig_pattern)
    # Allow flexible whitespace
    sig_pattern = sig_pattern.replace(r"\ ", r"\s+")

    # Find the function start
    function_start_idx = None
    for i, line in enumerate(lines):
        if re.search(sig_pattern, line):
            function_start_idx = i
            break

    if function_start_idx is None:
        # Try a more lenient search - just look for the function name
        # Extract function name from signature
        fn_match = re.search(r"fn\s+(\w+)", function_signature)
        if fn_match:
            fn_name = fn_match.group(1)
            for i, line in enumerate(lines):
                if re.search(rf"\bfn\s+{fn_name}\s*[\(<]", line):
                    function_start_idx = i
                    break

    if function_start_idx is None:
        return None

    # Find the function end by tracking braces
    brace_count = 0
    function_lines = []
    started_counting = False

    for i in range(function_start_idx, len(lines)):
        line = lines[i]
        function_lines.append(line)

        # Count braces in the line
        for char in line:
            if char == "{":
                brace_count += 1
                started_counting = True
            elif char == "}":
                brace_count -= 1

        # If we've started counting and returned to 0, we're done
        if started_counting and brace_count == 0:
            break

    return "\n".join(function_lines)


def main():
    # Load mizan.json
    mizan_path = Path(__file__).parent / "mizan.json"

    with open(mizan_path, "r") as f:
        data = json.load(f)

    vulnerabilities = data["vulnerabilities"]
    results = []

    print(f"Processing {len(vulnerabilities)} vulnerabilities...")

    for vuln in vulnerabilities:
        vuln_id = vuln["id"]
        print(f"Processing {vuln_id}...", end=" ")

        # Find the first code sample that is vulnerable (has vulnerable functions)
        vulnerable_sample = None
        for sample in vuln["code_samples"]:
            if sample["is_vulnerability"] and sample["vulnerable_functions"]:
                vulnerable_sample = sample
                break

        if not vulnerable_sample:
            print(f"WARNING: No vulnerable sample found for {vuln_id}")
            continue

        # Get the first vulnerable function
        vulnerable_funcs = vulnerable_sample["vulnerable_functions"]
        if not vulnerable_funcs:
            print(f"WARNING: No vulnerable functions for {vuln_id}")
            continue

        # Get the first file and its first function
        first_file = list(vulnerable_funcs.keys())[0]
        first_function_sig = vulnerable_funcs[first_file][0]

        # Construct the full path to the source file
        sample_path = (
            Path(__file__).parent / "samples" / vulnerable_sample["path_to_crate"]
        )
        source_file_path = sample_path / first_file

        # Extract the function code
        function_code = extract_function_from_rust_file(
            source_file_path, first_function_sig
        )

        if function_code is None:
            print(f"ERROR: Could not extract function from {source_file_path}")
            continue

        # Create the result entry
        result = {
            "vulnerability_id": vuln_id,
            "cve": vuln.get("source_link", ""),
            "crate_name": vuln.get("crate_name", ""),
            "year": vuln.get("year", 0),
            "cwe_type": vulnerable_sample.get("cwe_type", []),
            "function_signature": first_function_sig,
            "source_file": first_file,
            "vulnerable_code": function_code,
        }

        results.append(result)
        print("✓")

    # Write to JSONL file
    output_path = Path(__file__).parent / "vulnerable_functions.jsonl"
    with open(output_path, "w") as f:
        for result in results:
            f.write(json.dumps(result) + "\n")

    print(f"\nExtracted {len(results)} vulnerable functions")
    print(f"Output written to: {output_path}")

    if len(results) != len(vulnerabilities):
        print(
            f"\nWARNING: Expected {len(vulnerabilities)} vulnerabilities but extracted {len(results)}"
        )
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env bash
set -euo pipefail

json_file="$1"
out_file="CWEs.txt"

# 1. Extract unique CWE-IDs with jq
mapfile -t cwe_ids < <(
  jq -r '
    .vulnerabilities[]
    .code_samples[]
    .cwe_type[]
  ' "$json_file" | sort -u
)

: > "$out_file"   # truncate / create

for cwe in "${cwe_ids[@]}"; do
  num="${cwe#CWE-}"       # strip prefix
  title=""

  title=$(curl -sL "https://cwe.mitre.org/data/definitions/${num}.html" \
          | sed -n -E "s/.*<h2[^>]*>CWE-${num}:[[:space:]]*([^<]+)<\/h2>.*/\1/p" \
          | head -n1)

  # If title is empty, try to get it from NVD
  if [[ -z $title ]]; then
    title=$(curl -sL "https://nvd.nist.gov/vuln/categories" \
            | grep -oE "CWE-${num}[^<]+" | head -n1 \
            | sed -E "s/^CWE-${num}[[:space:]]*//")
  fi

  printf -- '- %s: %s\n' "$cwe" "${title:-Description not found}" >> "$out_file"
done

echo "✓ CWE meanings saved to $out_file"
